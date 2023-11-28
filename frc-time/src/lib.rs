use std::{
    sync::{
        atomic::{self, AtomicBool},
        Mutex,
    },
    time::{Duration, Instant, SystemTime},
};

#[ctor::ctor]
static DEFAULT_TIME_IMPL_STATICS: Mutex<(Instant, Duration, Option<Instant>)> =
    Mutex::new((Instant::now(), Duration::from_secs(0), None));

/// Returns the current time in microseconds.
static mut UPTIME_SOURCE: fn() -> u128 = || {
    let time_statics = DEFAULT_TIME_IMPL_STATICS
        .lock()
        .expect("DEFAULT_TIME_IMPL_STATICS mutex poisoned");
    let abs_uptime = time_statics.0.elapsed().as_micros();
    let paused_uptime = time_statics
        .2
        .map(|start| start.elapsed().as_micros())
        .unwrap_or(0);
    let passed_uptime = time_statics.1.as_micros();
    abs_uptime - (paused_uptime + passed_uptime)
};

static mut UPTIME_PAUSE: fn(bool) -> () = |boolean| {
    let mut time_statics = DEFAULT_TIME_IMPL_STATICS
        .lock()
        .expect("DEFAULT_TIME_IMPL_STATICS mutex poisoned");
    match (boolean, time_statics.2) {
        (true, None) => {
            time_statics.2 = Some(Instant::now());
        }
        (false, Some(start)) => {
            time_statics.1 += start.elapsed();
            time_statics.2 = None;
        }
        _ => unreachable!("Default uptime pause check failed"),
    }
};

static mut IMPLEMENTATION_NAME: &'static str = "Default";

static IS_PAUSED: AtomicBool = AtomicBool::new(false);
static TIME_IMPL_FROZEN: atomic::AtomicBool = atomic::AtomicBool::new(false);

pub struct TimeImplementation {
    pub implementation_name: &'static str,
    /// A custom monotomic timestamp imlementation
    pub source: fn() -> u128,
    /// A custom pause implementation,
    /// this is allowed to panic.
    pub pause: fn(bool) -> (),
    /// A custom system time implementation,
    /// will not be used for control, most likely just some timestamping
    pub system_source: fn() -> Option<SystemTime>,
}

pub mod __private {
    pub use ctor::ctor;

    use crate::TimeImplementation;
    pub unsafe fn __set_time_implementation(time_imp: TimeImplementation) {
        use std::sync::atomic::Ordering;
        if super::TIME_IMPL_FROZEN.swap(true, Ordering::Relaxed) {
            panic!(
                "Cannot set time source after it has been used or previously set(old: {}, new: {})",
                super::IMPLEMENTATION_NAME,
                time_imp.implementation_name
            );
        }
        super::UPTIME_SOURCE = time_imp.source;
        super::UPTIME_PAUSE = time_imp.pause;
        super::IMPLEMENTATION_NAME = time_imp.implementation_name;
    }
}

#[inline(always)]
pub fn now() -> u128 {
    TIME_IMPL_FROZEN.store(true, atomic::Ordering::Relaxed);
    unsafe { UPTIME_SOURCE() }
}

/// Pauses the time with platforms that support it, otherwise panics
#[inline(always)]
pub unsafe fn pause(should_pause: bool) {
    if IS_PAUSED.swap(should_pause, atomic::Ordering::Relaxed) == should_pause {
        if should_pause {
            panic!("Cannot pause if already paused")
        } else {
            panic!("Cannot un-pause if not paused")
        }
    } else {
        UPTIME_PAUSE(should_pause)
    }
}

/// Returns true if the time source is paused
#[inline(always)]
pub fn is_paused() -> bool {
    IS_PAUSED.load(atomic::Ordering::Relaxed)
}

/// Sets the function used to get the current time.
/// Uses ctor to set the function in "life before main".
/// If this is called multiple times there will be a panic
#[macro_export]
macro_rules! set_time_getter {
    ($getter:ident) => {
        #[$crate::__private::ctor]
        fn ____uptime_source_ctor_decl() {
            unsafe {
                $crate::__private::__set_uptime_source($getter, |_| {
                    panic!("Pause is not implemented");
                });
            }
        }
    };
}

#[cfg(test)]
mod test {
    use std::thread;

    #[test]
    fn test_time() {
        use super::*;
        let start = now();
        thread::sleep(Duration::from_millis(100));
        let end = now();
        assert!(end - start >= 100_000);
    }

    #[test]
    fn test_pause() {
        use super::*;
        unsafe {
            pause(true);
        }
        let start = now();
        thread::sleep(Duration::from_millis(1000));
        let end = now();
        assert!(end + 5 - start < 100);
        unsafe {
            pause(false);
        }
        thread::sleep(Duration::from_millis(1000));
        let end = now();
        assert!(end - start >= 1000_000);
    }
}

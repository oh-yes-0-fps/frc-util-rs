use std::ops::Neg;

use frc_units_macros::{unit, unit_conversion, unit_family};

unit!(Hour, f64);
unit!(Minute, f64);
unit!(Second, f64);
unit!(Millisecond, f64);
unit!(Microsecond, u64);

unit_conversion!(Second f64, Millisecond f64, second_to_millisecond);
unit_conversion!(Second f64, Microsecond u64, second_to_microsecond);
unit_conversion!(Millisecond f64, Microsecond u64, millisecond_to_microsecond);
unit_conversion!(Hour f64, Second f64, hour_to_second);
unit_conversion!(Minute f64, Second f64, minute_to_second);
unit_conversion!(Hour f64, Minute f64, hour_to_minute);
unit_conversion!(Minute f64, Millisecond f64, minute_to_millisecond);
unit_conversion!(Minute f64, Microsecond u64, minute_to_microsecond);
unit_conversion!(Hour f64, Millisecond f64, hour_to_millisecond);
unit_conversion!(Hour f64, Microsecond u64, hour_to_microsecond);

//This is a hack to satisfy unit family, will fix later
impl Neg for Microsecond {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self
    }
}

unit_family!(Time: Hour Minute Second Millisecond Microsecond);

fn second_to_millisecond(second: f64) -> f64 {
    second * 1000.0
}

fn second_to_microsecond(second: f64) -> u64 {
    (second * 1_000_000.0) as u64
}

fn millisecond_to_microsecond(millisecond: f64) -> u64 {
    (millisecond * 1000.0) as u64
}

fn hour_to_second(hour: f64) -> f64 {
    hour * 3600.0
}

fn minute_to_second(minute: f64) -> f64 {
    minute * 60.0
}

fn hour_to_minute(hour: f64) -> f64 {
    hour * 60.0
}

fn minute_to_millisecond(minute: f64) -> f64 {
    minute * 60000.0
}

fn minute_to_microsecond(minute: f64) -> u64 {
    (minute * 60_000_000.0) as u64
}

fn hour_to_millisecond(hour: f64) -> f64 {
    second_to_millisecond(hour_to_second(hour))
}

fn hour_to_microsecond(hour: f64) -> u64 {
    second_to_microsecond(hour_to_second(hour))
}

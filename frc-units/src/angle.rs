use crate::angular_velocity::{
    DegreePerSecond, RadianPerSecond, RotationPerMinute, RotationPerSecond,
};
use crate::angular_acceleration::{
    DegreePerSecondSquared, RadianPerSecondSquared, RotationPerMinuteSquared,
    RotationPerSecondSquared
};
use crate::time::{Minute, Second};
use frc_units_macros::{unit, unit_conversion, unit_dimensional_analysis, unit_family};

unit!(Degree, f64);
unit!(Radian, f64);
unit!(Rotation, f64);

unit_conversion!(Degree f64, Radian f64, degree_to_radian);
unit_conversion!(Degree f64, Rotation f64, degree_to_rotation);
unit_conversion!(Radian f64, Rotation f64, radian_to_rotation);

unit_family!(Angle: Degree Radian Rotation);

unit_dimensional_analysis!(DegreePerSecond * Second = Degree);
unit_dimensional_analysis!(RadianPerSecond * Second = Radian);
unit_dimensional_analysis!(RotationPerSecond * Second = Rotation);
unit_dimensional_analysis!(RotationPerMinute * Minute = Rotation);

unit_dimensional_analysis!(DegreePerSecondSquared * Second = DegreePerSecond);
unit_dimensional_analysis!(RadianPerSecondSquared * Second = RadianPerSecond);
unit_dimensional_analysis!(RotationPerSecondSquared * Second = RotationPerSecond);
unit_dimensional_analysis!(RotationPerMinuteSquared * Minute = RotationPerMinute);


fn degree_to_radian(degree: f64) -> f64 {
    degree.to_radians()
}

fn degree_to_rotation(degree: f64) -> f64 {
    degree / 360.0
}

fn radian_to_rotation(radian: f64) -> f64 {
    degree_to_rotation(radian.to_degrees())
}

impl Degree {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> DegreePerSecond {
        DegreePerSecond::new(self.value() * seconds.value())
    }
}

impl Radian {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> RadianPerSecond {
        RadianPerSecond::new(self.value() * seconds.value())
    }
}

impl Rotation {
    #[must_use]
    pub fn per_minute(self, minutes: Minute) -> RotationPerMinute {
        RotationPerMinute::new(self.value() * minutes.value())
    }

    #[must_use]
    pub fn per_second(self, seconds: Second) -> RotationPerSecond {
        RotationPerSecond::new(self.value() * seconds.value())
    }
}

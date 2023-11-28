use frc_units_macros::{unit, unit_conversion, unit_family};

unit!(DegreePerSecondSquared, f64);
unit!(RadianPerSecondSquared, f64);
unit!(RotationPerSecondSquared, f64);
unit!(RotationPerMinuteSquared, f64);

unit_conversion!(DegreePerSecondSquared f64, RadianPerSecondSquared f64, degree_per_second_squared_to_radian_per_second_squared);
unit_conversion!(DegreePerSecondSquared f64, RotationPerSecondSquared f64, degree_per_second_squared_to_rotation_per_second_squared);
unit_conversion!(DegreePerSecondSquared f64, RotationPerMinuteSquared f64, degree_per_second_squared_to_rotation_per_minute_squared);
unit_conversion!(RadianPerSecondSquared f64, RotationPerSecondSquared f64, radian_per_second_squared_to_rotation_per_second_squared);
unit_conversion!(RadianPerSecondSquared f64, RotationPerMinuteSquared f64, radian_per_second_squared_to_rotation_per_minute_squared);
unit_conversion!(RotationPerSecondSquared f64, RotationPerMinuteSquared f64, rotation_per_second_squared_to_rotation_per_minute_squared);

unit_family!(AngularAccel: DegreePerSecondSquared RadianPerSecondSquared RotationPerSecondSquared RotationPerMinuteSquared);

fn degree_per_second_squared_to_radian_per_second_squared(
    degree_per_second_squared: f64,
) -> f64 {
    degree_per_second_squared.to_radians()
}


fn degree_per_second_squared_to_rotation_per_second_squared(
    degree_per_second_squared: f64,
) -> f64 {
    degree_per_second_squared / 360.0
}


fn degree_per_second_squared_to_rotation_per_minute_squared(
    degree_per_second_squared: f64,
) -> f64 {
    degree_per_second_squared / 360.0 * 60.0
}


fn radian_per_second_squared_to_rotation_per_second_squared(
    radian_per_second_squared: f64,
) -> f64 {
    degree_per_second_squared_to_rotation_per_second_squared(radian_per_second_squared.to_degrees())
}


fn radian_per_second_squared_to_rotation_per_minute_squared(
    radian_per_second_squared: f64,
) -> f64 {
    degree_per_second_squared_to_rotation_per_minute_squared(radian_per_second_squared.to_degrees())
}


fn rotation_per_second_squared_to_rotation_per_minute_squared(
    rotation_per_second_squared: f64,
) -> f64 {
    rotation_per_second_squared * 60.0
}

use frc_units_macros::{unit, unit_conversion, unit_family};

unit!(Celsius, f64);
unit!(Fahrenheit, f64);
unit!(Kelvin, f64);

unit_conversion!(Celsius f64, Fahrenheit f64, celsius_to_fahrenheit);
unit_conversion!(Celsius f64, Kelvin f64, celsius_to_kelvin);
unit_conversion!(Fahrenheit f64, Kelvin f64, fahrenheit_to_kelvin);

unit_family!(Temperature: Celsius Fahrenheit Kelvin);

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius.mul_add(1.8, 32.0)
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + 459.67) * (5.0 / 9.0)
}

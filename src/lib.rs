pub mod macros;

pub fn make_strings(value: f32, label: &str) -> (String, String) {
    (format!("{:.2}", value), label.to_string())
}

pub fn ease_in_expo(x: f32) -> f32 {
    if x <= 0.0 {
        0.0
    } else {
        (2.0f32.powf(10.0 * x) - 1.0) / (2.0f32.powf(10.0) - 1.0)
    }
}

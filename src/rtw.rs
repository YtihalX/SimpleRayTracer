pub fn clamp(x: f64, lower: f64, upper: f64) -> f64 {
    x.max(lower).min(upper)
}

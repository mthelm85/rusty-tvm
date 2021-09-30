use std::f64;

pub fn eair(i: &f64, n: &f64) -> f64 {
    (f64::powf(1.0 + (i / 100.0 / n), *n) - 1.0) * 100.0
}
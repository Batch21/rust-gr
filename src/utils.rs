pub fn s_curve1(t: f64, days: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    } else if t < days {
        return (t / days).powf(2.5);
    }
    1.0
}

pub fn s_curve2(t: f64, days: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    } else if t < days {
        return 0.5 * (t / days).powf(2.5);
    } else if t < 2.0 * days {
        return 1.0 - 0.5 * (2.0 - t / days).powf(2.5);
    }
    1.0
}
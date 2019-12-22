pub struct Metrics {
    pub down: f64,
    pub up: f64,
    pub ping: i32,
}

pub trait Analyzer {
    fn analyze(&self) -> Metrics;
}
use std::time::Duration;

pub trait Escalator { fn threshold(&self, elapsed: Duration) -> f64; }

pub struct LinearEsc { pub base: f64, pub slope: f64, pub cap: f64 }

impl Escalator for LinearEsc {
    fn threshold(&self, elapsed: Duration) -> f64 {
        (self.base + self.slope * elapsed.as_secs_f64()).min(self.cap)
    }
}
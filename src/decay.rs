use std::time::Duration;

/// Trait for different decay strategies
pub trait Decay { 
    fn weight(&self, age: Duration) -> f64; 
}

/// Exponential decay: weight decreases exponentially over time
pub struct ExpDecay(pub Duration);

/// Linear decay: weight decreases linearly over time
pub struct LinearDecay(pub Duration);

/// Step decay: weight decreases in discrete steps
pub struct StepDecay(pub Duration);

impl Decay for ExpDecay {
    fn weight(&self, age: Duration) -> f64 {
        // Formula: 0.5^(age/half_life)
        0.5_f64.powf(age.as_secs_f64() / self.0.as_secs_f64())
    }
}

impl Decay for LinearDecay {
    fn weight(&self, age: Duration) -> f64 {
        // Formula: max(0.1, 1 - age/duration)
        (1.0 - age.as_secs_f64() / self.0.as_secs_f64()).max(0.1)
    }
}

impl Decay for StepDecay {
    fn weight(&self, age: Duration) -> f64 {
        // Formula: 1 / (steps + 1)
        let steps = age.as_secs() / self.0.as_secs();
        1.0 / (steps + 1) as f64
    }
}

// Allow using `Box<dyn Decay>` wherever a `Decay` is expected
impl<D: Decay + ?Sized> Decay for Box<D> {
    fn weight(&self, age: Duration) -> f64 { 
        (**self).weight(age) 
    }
}
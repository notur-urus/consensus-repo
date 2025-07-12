use std::time::{Duration,SystemTime};

#[derive(Clone)]
pub struct Vote {
    pub value: String,   
    pub ts: SystemTime,   
}

pub struct Window {
    pub start: SystemTime,    
    pub duration: Duration,   
}

impl Window {
    pub fn is_open(&self) -> bool {
        self.start.elapsed().unwrap() < self.duration
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed().unwrap()
    }
}

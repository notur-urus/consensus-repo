
use std::collections::HashMap;
use std::time::SystemTime;

use crate::decay::Decay;
use crate::threshold::Escalator;
use crate::vote::{Vote, Window};

pub struct Consensus {
    decay: Box<dyn Decay>,           
    escalator: Box<dyn Escalator>,   
    window: Window,                  
    votes: Vec<Vote>,                
    min_weight: f64,   
}

impl Consensus {
    pub fn new(
        decay: Box<dyn Decay>,
        escalator: Box<dyn Escalator>,
        window: Window,
    ) -> Self {
        Self {
            decay,
            escalator,
            window,
            votes: vec![],
            min_weight: 0.1,  
        }
    }

    pub fn cast(&mut self, value: &str) {
        if self.window.is_open() {
            self.votes.push(Vote {
                value: value.into(),
                ts: SystemTime::now(),
            });
        }
    }

    pub fn result(&self) -> Option<String> {
        let threshold = self.escalator.threshold(self.window.elapsed());

        let mut weights: HashMap<String, f64> = HashMap::new();
        
        for vote in &self.votes {
            let weight = self.decay.weight(vote.ts.elapsed().unwrap()).max(self.min_weight);
            *weights.entry(vote.value.clone()).or_default() += weight;
        }
        
        let total: f64 = weights.values().sum();
        
        if total == 0.0 {
            return None;  
        }

        weights
            .into_iter()
            .find(|(_, weight)| *weight / total >= threshold)
            .map(|(value, _)| value)
    }

    pub fn vote_count(&self) -> usize {
        self.votes.len()
    }

    pub fn current_threshold(&self) -> f64 {
        self.escalator.threshold(self.window.elapsed())
    }

    pub fn is_window_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn time_remaining(&self) -> Option<std::time::Duration> {
        let elapsed = self.window.elapsed();
        let remaining = self.window.duration.checked_sub(elapsed)?;
        Some(remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decay::ExpDecay;
    use crate::threshold::LinearEsc;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_consensus_majority_wins() {
        let decay = Box::new(ExpDecay(Duration::from_secs(60)));
        let escalator = Box::new(LinearEsc {
            base: 0.51, 
            slope: 0.0,  
            cap: 1.0,
        });
        let window = Window {
            start: SystemTime::now(),
            duration: Duration::from_secs(10),
        };
        
        let mut consensus = Consensus::new(decay, escalator, window);

        for _ in 0..3 {
            consensus.cast("A");
        }
        consensus.cast("B");

        assert_eq!(consensus.result(), Some("A".into()));
    }

    #[test]
    fn test_no_consensus_when_below_threshold() {
        let decay = Box::new(ExpDecay(Duration::from_secs(60)));
        let escalator = Box::new(LinearEsc {
            base: 0.75,
            slope: 0.0,  
            cap: 1.0,
        });
        let window = Window {
            start: SystemTime::now(),
            duration: Duration::from_secs(10),
        };
        
        let mut consensus = Consensus::new(decay, escalator, window);

        consensus.cast("A");
        consensus.cast("B");
        consensus.cast("A");

        assert_eq!(consensus.result(), None);
    }

    #[test]
    fn test_decay_reduces_old_vote_weight() {
        let decay = Box::new(ExpDecay(Duration::from_secs(1)));
        let escalator = Box::new(LinearEsc {
            base: 0.51,
            slope: 0.0,
            cap: 1.0,
        });
        let window = Window {
            start: SystemTime::now(),
            duration: Duration::from_secs(10),
        };
        
        let mut consensus = Consensus::new(decay, escalator, window);

        consensus.cast("X");
        
        std::thread::sleep(Duration::from_secs(2));
        
        consensus.cast("Y");

        assert_eq!(consensus.result(), Some("Y".into()));
    }

    #[test]
    fn test_vote_count() {
        let decay = Box::new(ExpDecay(Duration::from_secs(60)));
        let escalator = Box::new(LinearEsc {
            base: 0.51,
            slope: 0.0,
            cap: 1.0,
        });
        let window = Window {
            start: SystemTime::now(),
            duration: Duration::from_secs(10),
        };
        
        let mut consensus = Consensus::new(decay, escalator, window);
        
        assert_eq!(consensus.vote_count(), 0);
        
        consensus.cast("A");
        assert_eq!(consensus.vote_count(), 1);
        
        consensus.cast("B");
        assert_eq!(consensus.vote_count(), 2);
    }

    #[test]
    fn test_window_status() {
        let decay = Box::new(ExpDecay(Duration::from_secs(60)));
        let escalator = Box::new(LinearEsc {
            base: 0.51,
            slope: 0.0,
            cap: 1.0,
        });
        let window = Window {
            start: SystemTime::now(),
            duration: Duration::from_secs(1), 
        };
        
        let consensus = Consensus::new(decay, escalator, window);
        
        assert!(consensus.is_window_open());
        
        std::thread::sleep(Duration::from_secs(2));
        
        assert!(!consensus.is_window_open());
    }
}
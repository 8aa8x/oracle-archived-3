use thiserror::Error;
use tokio::time::{Duration, Instant};

#[derive(Error, Debug)]
pub enum RotatorError {
    #[error("vector doesn't contain any values")]
    EmptyVector,
}

#[derive(Debug)]
pub struct Rotator<T> {
    all: Vec<T>,
    pub current: T,
    max_retries: usize,
    exponential_backoff: usize,
    retries: Vec<Instant>,
}

impl<T> Rotator<T> {
    /// Create a new Rotator with a custom type
    pub fn new(
        mut all: Vec<T>,
        max_retries: usize,
        exponential_backoff: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let first = all.pop().ok_or(RotatorError::EmptyVector)?;

        Ok(Self {
            all,
            current: first,
            max_retries,
            exponential_backoff,
            retries: Vec::default(),
        })
    }

    /// Get the amount of retries in the past given duration
    pub fn retries_since(&self, duration: Duration) -> usize {
        self.retries
            .iter()
            .filter(|&&instant| Instant::now().duration_since(instant) <= duration)
            .count()
    }

    /// Change the current object to the next and resets the retries
    pub fn rotate_current(&mut self) {
        if let Some(mut new) = self.all.pop() {
            std::mem::swap(&mut self.current, &mut new);
            self.retries.clear();
        }
    }
}

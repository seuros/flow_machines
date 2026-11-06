use core::num::NonZeroU32;

/// Whether a producer should currently run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FlowState {
    /// The producer may attempt admission.
    Running,
    /// The producer should stop submitting new work.
    Paused,
}

/// A state transition to propagate to a producer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Signal {
    /// Stop producing until resumed.
    Pause,
    /// Production may resume, subject to admission credits.
    Resume,
    /// No state transition occurred.
    Unchanged,
}

/// Hysteretic high/low occupancy thresholds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Watermarks {
    low: u32,
    high: NonZeroU32,
    state: FlowState,
}

impl Watermarks {
    /// Construct a running controller, or return `None` unless `low < high`.
    #[must_use]
    pub const fn new(low: u32, high: NonZeroU32) -> Option<Self> {
        if low >= high.get() {
            return None;
        }
        Some(Self {
            low,
            high,
            state: FlowState::Running,
        })
    }

    /// Observe occupancy and emit a pause/resume edge when a threshold is crossed.
    #[must_use]
    pub const fn observe(&mut self, occupancy: u32) -> Signal {
        match self.state {
            FlowState::Running if occupancy >= self.high.get() => {
                self.state = FlowState::Paused;
                Signal::Pause
            }
            FlowState::Paused if occupancy <= self.low => {
                self.state = FlowState::Running;
                Signal::Resume
            }
            _ => Signal::Unchanged,
        }
    }

    /// Current producer state, useful when a producer first attaches.
    #[must_use]
    pub const fn state(&self) -> FlowState {
        self.state
    }
}

#[cfg(test)]
mod tests;

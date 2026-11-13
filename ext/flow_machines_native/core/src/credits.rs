use core::num::NonZeroU32;

use crate::Refusal;

/// Aggregate capacity accounting for caller-owned work.
///
/// The caller must serialize this state with admission and return every acquired
/// unit exactly once. This type does not track individual permits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditWindow {
    capacity: NonZeroU32,
    in_flight: u32,
}

impl CreditWindow {
    /// Construct an empty window with positive capacity.
    #[must_use]
    pub const fn new(capacity: NonZeroU32) -> Self {
        Self {
            capacity,
            in_flight: 0,
        }
    }

    /// Acquire all requested units.
    ///
    /// # Errors
    ///
    /// [`Refusal::Insufficient`] when `units` exceeds [`Self::available`]:
    /// there is no partial grant and nothing changes.
    pub const fn try_acquire(&mut self, units: NonZeroU32) -> Result<(), Refusal> {
        let available = self.available();
        if units.get() > available {
            return Err(Refusal::Insufficient { available });
        }
        self.in_flight += units.get();
        Ok(())
    }

    /// Return all requested units.
    ///
    /// The window counts units, not permits: an honoured release does not
    /// prove that this particular work owned credits, so a repeated release
    /// cannot be recognised as a repeat. It is honoured while units remain in
    /// flight and refused once they do not.
    ///
    /// # Errors
    ///
    /// [`Refusal::ExceedsInFlight`] when `units` exceeds [`Self::in_flight`]:
    /// there is no partial return and nothing changes.
    pub const fn release(&mut self, units: NonZeroU32) -> Result<(), Refusal> {
        if units.get() > self.in_flight {
            return Err(Refusal::ExceedsInFlight {
                in_flight: self.in_flight,
            });
        }
        self.in_flight -= units.get();
        Ok(())
    }

    /// Maximum outstanding units.
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        self.capacity.get()
    }

    /// Units currently accounted as outstanding.
    #[must_use]
    pub const fn in_flight(&self) -> u32 {
        self.in_flight
    }

    /// Units still available for acquisition.
    #[must_use]
    pub const fn available(&self) -> u32 {
        self.capacity.get() - self.in_flight
    }
}

#[cfg(test)]
mod tests;

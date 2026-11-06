use core::num::NonZeroU32;

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

    /// Acquire all requested units, or return `false` without changing state.
    #[must_use]
    pub const fn try_acquire(&mut self, units: NonZeroU32) -> bool {
        if units.get() > self.available() {
            return false;
        }
        self.in_flight += units.get();
        true
    }

    /// Return all requested units, or return `false` if that exceeds the balance.
    ///
    /// A successful return does not prove that this particular work owned credits.
    #[must_use]
    pub const fn release(&mut self, units: NonZeroU32) -> bool {
        if units.get() > self.in_flight {
            return false;
        }
        self.in_flight -= units.get();
        true
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

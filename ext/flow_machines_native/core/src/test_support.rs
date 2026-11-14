//! Helpers shared by the crate's test modules.

use core::num::NonZeroU32;

/// A positive test amount; zero is a broken test assumption.
pub fn nonzero(value: u32) -> NonZeroU32 {
    let Some(value) = NonZeroU32::new(value) else {
        panic!("test amounts are positive");
    };
    value
}

/// xorshift64: a fixed seed keeps a trace reproducible.
pub struct Rng(pub u64);

impl Rng {
    /// The next draw, in `0..bound`.
    pub fn below(&mut self, bound: u32) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        let Ok(draw) = u32::try_from(self.0 >> 40) else {
            panic!("24 bits fit");
        };
        draw % bound
    }
}

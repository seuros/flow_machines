//! Helpers shared by the crate's test modules.

use core::num::NonZeroU32;

use matryona_machines::testing::XorShift64;

/// A positive test amount; zero is a broken test assumption.
pub fn nonzero(value: u32) -> NonZeroU32 {
    let Some(value) = NonZeroU32::new(value) else {
        panic!("test amounts are positive");
    };
    value
}

/// The next draw of `rng`, in `0..bound`.
pub fn below(rng: &mut XorShift64, bound: u32) -> u32 {
    let Ok(draw) = u32::try_from(rng.below(u64::from(bound))) else {
        panic!("a draw below a u32 fits a u32");
    };
    draw
}

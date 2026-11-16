#![no_std]
#![cfg_attr(not(test), deny(clippy::indexing_slicing))]
#![doc = include_str!("../README.md")]

mod credits;
mod refusal;
#[cfg(test)]
mod test_support;
mod watermarks;

pub use credits::CreditWindow;
pub use refusal::Refusal;
pub use watermarks::{FlowState, Signal, Watermarks};

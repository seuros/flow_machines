#![no_std]
#![doc = include_str!("../README.md")]

mod credits;
mod refusal;
#[cfg(test)]
mod test_support;
mod watermarks;

pub use credits::CreditWindow;
pub use refusal::Refusal;
pub use watermarks::{FlowState, Signal, Watermarks};

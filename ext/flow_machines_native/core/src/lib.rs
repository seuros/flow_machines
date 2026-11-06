#![no_std]
#![doc = include_str!("../README.md")]

mod credits;
mod watermarks;

pub use credits::CreditWindow;
pub use watermarks::{FlowState, Signal, Watermarks};

//! Thin Ruby bindings for capacity accounting and producer signals.

mod credits;
mod init;
mod watermarks;

use std::num::NonZeroU32;

use flow_machines::Refusal;
use magnus::{Error, Integer, Ruby, TryConvert, Value, prelude::*};

/// A count argument, named in every message so the caller sees which one
/// was wrong. Counts must be Ruby Integers: Magnus otherwise permits
/// truncating Floats, and a Hash here is a caller who used keywords.
pub(crate) fn count(ruby: &Ruby, value: Value, name: &str) -> Result<u32, Error> {
    if Integer::from_value(value).is_none() {
        let class = value.class();
        return Err(Error::new(
            ruby.exception_type_error(),
            format!("{name} must be an Integer, not {class}"),
        ));
    }
    u32::try_convert(value).map_err(|_| {
        Error::new(
            ruby.exception_range_error(),
            format!("{name} must be in 0..={}", u32::MAX),
        )
    })
}

/// The core's `NonZeroU32`: zero is out of range, like a negative count.
pub(crate) fn positive(ruby: &Ruby, value: Value, name: &str) -> Result<NonZeroU32, Error> {
    NonZeroU32::new(count(ruby, value, name)?).ok_or_else(|| {
        Error::new(
            ruby.exception_range_error(),
            format!("{name} must be positive"),
        )
    })
}

/// One symbol per core variant, the snake case of its name.
pub(crate) const fn refusal(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::Insufficient { .. } => "insufficient",
        Refusal::ExceedsInFlight { .. } => "exceeds_in_flight",
    }
}

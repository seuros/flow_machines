//! Thin Ruby bindings for capacity accounting and producer signals.

mod credits;
mod init;
mod watermarks;

use std::num::NonZeroU32;

use flow_machines::Refusal;
use magnus::{Error, Integer, Ruby, TryConvert, Value};

/// Counts must be Ruby Integers: Magnus otherwise permits truncating Floats.
#[derive(Clone, Copy)]
pub(crate) struct Count(pub(crate) u32);

impl TryConvert for Count {
    fn try_convert(value: Value) -> Result<Self, Error> {
        if Integer::from_value(value).is_none() {
            let ruby = Ruby::get_with(value);
            return Err(Error::new(
                ruby.exception_type_error(),
                "count must be an Integer",
            ));
        }
        u32::try_convert(value).map(Self)
    }
}

/// The core's `NonZeroU32`: zero is out of range, like a negative count.
pub(crate) fn positive(ruby: &Ruby, count: Count) -> Result<NonZeroU32, Error> {
    NonZeroU32::new(count.0)
        .ok_or_else(|| Error::new(ruby.exception_range_error(), "count must be positive"))
}

/// One symbol per core variant, the snake case of its name.
pub(crate) const fn refusal(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::Insufficient { .. } => "insufficient",
        Refusal::ExceedsInFlight { .. } => "exceeds_in_flight",
    }
}

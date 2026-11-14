//! Thin Ruby bindings for capacity accounting and producer signals.

mod credits;
mod init;
mod watermarks;

use flow_machines::Refusal;

/// One symbol per core variant, the snake case of its name.
pub(crate) const fn refusal(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::Insufficient { .. } => "insufficient",
        Refusal::ExceedsInFlight { .. } => "exceeds_in_flight",
    }
}

//! Thin Ruby bindings for capacity accounting and producer signals.

mod credits;
mod init;
mod watermarks;

use flow_machines::Refusal;

matryona_machines::symbols! {
    /// One symbol per core variant, the snake case of its name; the counts
    /// a variant carries stay in Rust.
    pub(crate) Refusals for Refusal as "a refusal" => names_only {
        Insufficient { .. } => "insufficient",
        ExceedsInFlight { .. } => "exceeds_in_flight",
    }
}

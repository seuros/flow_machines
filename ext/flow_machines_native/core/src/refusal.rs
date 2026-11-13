use core::fmt;

/// Why a call was not honoured. A refused call changes nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The acquisition asks for more units than the window has available.
    Insufficient {
        /// Units the window could still grant.
        available: u32,
    },
    /// The release returns more units than the window accounts as in flight.
    ExceedsInFlight {
        /// Units the window accounts as outstanding.
        in_flight: u32,
    },
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Insufficient { available } => {
                write!(f, "more units asked for than the {available} available")
            }
            Self::ExceedsInFlight { in_flight } => {
                write!(f, "more units returned than the {in_flight} in flight")
            }
        }
    }
}

impl core::error::Error for Refusal {}

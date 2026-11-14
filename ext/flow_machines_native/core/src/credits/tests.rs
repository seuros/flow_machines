use super::*;
use crate::test_support::{Rng, nonzero};

/// One call in a scripted trace, and the answer the window must give.
#[derive(Clone, Copy)]
enum Step {
    Acquire(u32, Result<(), Refusal>),
    Release(u32, Result<(), Refusal>),
}

/// Run `steps` against a fresh window; every refusal must leave it as it was.
fn replay(capacity: u32, steps: &[Step]) -> CreditWindow {
    let mut window = CreditWindow::new(nonzero(capacity));
    for (index, &step) in steps.iter().enumerate() {
        let before = window.clone();
        let (outcome, expected) = match step {
            Step::Acquire(units, expected) => (window.try_acquire(nonzero(units)), expected),
            Step::Release(units, expected) => (window.release(nonzero(units)), expected),
        };
        assert_eq!(outcome, expected, "step {index} of capacity {capacity}");
        if outcome.is_err() {
            assert_eq!(window, before, "refused step {index} changed the window");
        }
    }
    window
}

#[test]
fn scripted_traces_are_all_or_nothing() {
    use Step::{Acquire, Release};
    const fn short(available: u32) -> Result<(), Refusal> {
        Err(Refusal::Insufficient { available })
    }
    const fn over(in_flight: u32) -> Result<(), Refusal> {
        Err(Refusal::ExceedsInFlight { in_flight })
    }
    let cases: [(u32, &[Step], u32); 4] = [
        // Capacity is a boundary with no partial grant.
        (
            5,
            &[
                Acquire(3, Ok(())),
                Acquire(3, short(2)),
                Acquire(2, Ok(())),
                Acquire(1, short(0)),
            ],
            5,
        ),
        // Over-release creates no credits.
        (
            5,
            &[Release(1, over(0)), Acquire(2, Ok(())), Release(3, over(2))],
            2,
        ),
        // Aggregate credits carry no owner, so a repeated release cannot be
        // recognised: it is honoured while units remain in flight.
        (
            4,
            &[
                Acquire(2, Ok(())),
                Release(1, Ok(())),
                Release(1, Ok(())),
                Release(1, over(0)),
            ],
            0,
        ),
        // The full u32 range does not overflow.
        (
            u32::MAX,
            &[Acquire(u32::MAX, Ok(())), Acquire(1, short(0))],
            u32::MAX,
        ),
    ];
    for (capacity, steps, in_flight) in cases {
        let window = replay(capacity, steps);
        assert_eq!(window.capacity(), capacity);
        assert_eq!(window.in_flight(), in_flight);
        assert_eq!(window.available(), capacity - in_flight);
    }
}

#[test]
fn refusals_describe_themselves() {
    extern crate std;
    use std::string::ToString;

    assert_eq!(
        Refusal::Insufficient { available: 2 }.to_string(),
        "more units asked for than the 2 available"
    );
    assert_eq!(
        Refusal::ExceedsInFlight { in_flight: 3 }.to_string(),
        "more units returned than the 3 in flight"
    );
}

#[test]
fn randomized_trace_matches_a_balance_model() {
    const CAPACITY: u32 = 17;
    let mut rng = Rng(0x4209_4209_4209_4209);
    let mut window = CreditWindow::new(nonzero(CAPACITY));
    let mut outstanding = 0;
    let (mut honoured, mut refusals) = (0, 0);
    for _ in 0..20_000 {
        let amount = 1 + rng.below(23);
        let before = window.clone();
        let (outcome, expected) = if rng.below(2) == 0 {
            let available = CAPACITY - outstanding;
            let expected = if amount <= available {
                outstanding += amount;
                Ok(())
            } else {
                Err(Refusal::Insufficient { available })
            };
            (window.try_acquire(nonzero(amount)), expected)
        } else {
            let expected = if amount <= outstanding {
                outstanding -= amount;
                Ok(())
            } else {
                Err(Refusal::ExceedsInFlight {
                    in_flight: outstanding,
                })
            };
            (window.release(nonzero(amount)), expected)
        };
        assert_eq!(outcome, expected);
        if outcome.is_err() {
            assert_eq!(window, before);
            refusals += 1;
        } else {
            honoured += 1;
        }
        assert_eq!(window.in_flight(), outstanding);
        assert_eq!(window.available() + outstanding, CAPACITY);
    }
    assert!(honoured > 1_000 && refusals > 1_000);
}

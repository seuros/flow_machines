use super::*;
use crate::test_support::{below, nonzero};
use matryona_machines::testing::{XorShift64, refuses};

/// One call in a scripted trace, and the answer the window must give.
#[derive(Clone, Copy)]
enum Step {
    Acquire(u32, Result<(), Refusal>),
    Release(u32, Result<(), Refusal>),
}

/// A window call taking a count of units.
type Call = fn(&mut CreditWindow, NonZeroU32) -> Result<(), Refusal>;

/// Run `steps` against a fresh window; every refusal must leave it as it was.
fn replay(capacity: u32, steps: &[Step]) -> CreditWindow {
    let mut window = CreditWindow::new(nonzero(capacity));
    for (index, &step) in steps.iter().enumerate() {
        let (units, expected, call): (_, _, Call) = match step {
            Step::Acquire(units, expected) => (units, expected, CreditWindow::try_acquire),
            Step::Release(units, expected) => (units, expected, CreditWindow::release),
        };
        match expected {
            Err(refusal) => refuses(&mut window, refusal, |w| call(w, nonzero(units))),
            Ok(()) => assert_eq!(
                call(&mut window, nonzero(units)),
                Ok(()),
                "step {index} of capacity {capacity}"
            ),
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
    let mut rng = XorShift64::new(0x4209_4209_4209_4209);
    let mut window = CreditWindow::new(nonzero(CAPACITY));
    let mut outstanding = 0;
    let (mut honoured, mut refusals) = (0, 0);
    for _ in 0..20_000 {
        let amount = 1 + below(&mut rng, 23);
        let (call, expected): (Call, _) = if below(&mut rng, 2) == 0 {
            let available = CAPACITY - outstanding;
            let expected = if amount <= available {
                outstanding += amount;
                Ok(())
            } else {
                Err(Refusal::Insufficient { available })
            };
            (CreditWindow::try_acquire, expected)
        } else {
            let expected = if amount <= outstanding {
                outstanding -= amount;
                Ok(())
            } else {
                Err(Refusal::ExceedsInFlight {
                    in_flight: outstanding,
                })
            };
            (CreditWindow::release, expected)
        };
        if let Err(refusal) = expected {
            refuses(&mut window, refusal, |w| call(w, nonzero(amount)));
            refusals += 1;
        } else {
            assert_eq!(call(&mut window, nonzero(amount)), Ok(()));
            honoured += 1;
        }
        assert_eq!(window.in_flight(), outstanding);
        assert_eq!(window.available() + outstanding, CAPACITY);
    }
    assert!(honoured > 1_000 && refusals > 1_000);
}

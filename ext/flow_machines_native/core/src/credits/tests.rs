#![allow(clippy::expect_used)] // Failed test assumptions should panic.

use super::*;

fn units(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).expect("test units are positive")
}

/// The call must be refused with `refusal` and leave the whole window as it was.
fn refused(
    window: &CreditWindow,
    outcome: Result<(), Refusal>,
    before: &CreditWindow,
    refusal: Refusal,
) {
    assert_eq!(outcome, Err(refusal));
    assert_eq!(window, before);
}

#[test]
fn capacity_is_an_all_or_nothing_boundary() {
    let mut window = CreditWindow::new(units(5));
    assert_eq!(window.capacity(), 5);
    assert_eq!(window.try_acquire(units(3)), Ok(()));
    let before = window.clone();
    let outcome = window.try_acquire(units(3));
    refused(
        &window,
        outcome,
        &before,
        Refusal::Insufficient { available: 2 },
    );
    assert_eq!(window.try_acquire(units(2)), Ok(()));
    assert_eq!(window.available(), 0);
    let before = window.clone();
    let outcome = window.try_acquire(units(1));
    refused(
        &window,
        outcome,
        &before,
        Refusal::Insufficient { available: 0 },
    );
    assert_eq!(window.release(units(5)), Ok(()));
    assert_eq!(window.in_flight(), 0);
}

#[test]
fn over_release_does_not_create_credits() {
    let mut window = CreditWindow::new(units(5));
    let before = window.clone();
    let outcome = window.release(units(1));
    refused(
        &window,
        outcome,
        &before,
        Refusal::ExceedsInFlight { in_flight: 0 },
    );
    assert_eq!(window.try_acquire(units(2)), Ok(()));
    let before = window.clone();
    let outcome = window.release(units(3));
    refused(
        &window,
        outcome,
        &before,
        Refusal::ExceedsInFlight { in_flight: 2 },
    );
}

#[test]
fn a_repeated_release_is_not_recognised_as_a_repeat() {
    // Aggregate credits carry no owner, so the window cannot prove a repeat:
    // it is honoured while units remain in flight and refused once they do not.
    let mut window = CreditWindow::new(units(4));
    assert_eq!(window.try_acquire(units(2)), Ok(()));
    assert_eq!(window.release(units(1)), Ok(()));
    assert_eq!(window.release(units(1)), Ok(()));
    let before = window.clone();
    let outcome = window.release(units(1));
    refused(
        &window,
        outcome,
        &before,
        Refusal::ExceedsInFlight { in_flight: 0 },
    );
}

#[test]
fn maximum_capacity_does_not_overflow() {
    let mut window = CreditWindow::new(units(u32::MAX));
    assert_eq!(window.try_acquire(units(u32::MAX)), Ok(()));
    let before = window.clone();
    let outcome = window.try_acquire(units(1));
    refused(
        &window,
        outcome,
        &before,
        Refusal::Insufficient { available: 0 },
    );
    assert_eq!(window.release(units(u32::MAX)), Ok(()));
    assert_eq!(window.available(), u32::MAX);
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

/// xorshift64: a fixed seed keeps the trace reproducible.
fn next(state: &mut u64) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    u32::try_from(*state >> 40).expect("24 bits fit")
}

#[test]
fn randomized_trace_matches_a_balance_model() {
    const CAPACITY: u32 = 17;
    let mut seed = 0x4209_4209_4209_4209;
    let mut window = CreditWindow::new(units(CAPACITY));
    let mut outstanding = 0;
    let (mut honoured, mut refusals) = (0, 0);
    for _ in 0..20_000 {
        let amount = 1 + next(&mut seed) % 23;
        let before = window.clone();
        let (outcome, expected) = if next(&mut seed).is_multiple_of(2) {
            let available = CAPACITY - outstanding;
            let expected = if amount <= available {
                outstanding += amount;
                Ok(())
            } else {
                Err(Refusal::Insufficient { available })
            };
            (window.try_acquire(units(amount)), expected)
        } else {
            let expected = if amount <= outstanding {
                outstanding -= amount;
                Ok(())
            } else {
                Err(Refusal::ExceedsInFlight {
                    in_flight: outstanding,
                })
            };
            (window.release(units(amount)), expected)
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

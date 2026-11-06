#![allow(clippy::expect_used)] // Failed test assumptions should panic.

use super::*;

fn units(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).expect("test units are positive")
}

#[test]
fn capacity_is_an_all_or_nothing_boundary() {
    let mut window = CreditWindow::new(units(5));
    assert_eq!(window.capacity(), 5);
    assert!(window.try_acquire(units(3)));
    let before = window.clone();
    assert!(!window.try_acquire(units(3)));
    assert_eq!(window, before);
    assert!(window.try_acquire(units(2)));
    assert_eq!(window.available(), 0);
    assert!(!window.try_acquire(units(1)));
    assert!(window.release(units(5)));
    assert_eq!(window.in_flight(), 0);
}

#[test]
fn over_release_does_not_create_credits() {
    let mut window = CreditWindow::new(units(5));
    assert!(!window.release(units(1)));
    assert!(window.try_acquire(units(2)));
    let before = window.clone();
    assert!(!window.release(units(3)));
    assert_eq!(window, before);
}

#[test]
fn maximum_capacity_does_not_overflow() {
    let mut window = CreditWindow::new(units(u32::MAX));
    assert!(window.try_acquire(units(u32::MAX)));
    assert!(!window.try_acquire(units(1)));
    assert!(window.release(units(u32::MAX)));
    assert_eq!(window.available(), u32::MAX);
}

#[test]
fn arbitrary_accounting_preserves_the_capacity_invariant() {
    let mut window = CreditWindow::new(units(17));
    let mut outstanding = 0;
    for step in 1..=10_000 {
        let amount = 1 + (step * 7) % 23;
        if step % 3 == 0 {
            assert_eq!(window.release(units(amount)), amount <= outstanding);
            if amount <= outstanding {
                outstanding -= amount;
            }
        } else {
            assert_eq!(
                window.try_acquire(units(amount)),
                amount <= 17 - outstanding
            );
            if amount <= 17 - outstanding {
                outstanding += amount;
            }
        }
        assert_eq!(window.in_flight(), outstanding);
        assert_eq!(window.available() + outstanding, 17);
    }
}

#![allow(clippy::expect_used)] // Failed test assumptions should panic.

use super::*;

fn high(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).expect("test high watermark is positive")
}

#[test]
fn rejects_equal_or_inverted_thresholds() {
    assert_eq!(Watermarks::new(5, high(5)), None);
    assert_eq!(Watermarks::new(6, high(5)), None);
    assert!(Watermarks::new(0, high(1)).is_some());
}

#[test]
fn crossing_thresholds_emits_edges_only() {
    let mut control = Watermarks::new(2, high(5)).expect("valid thresholds");
    let readings = [0, 4, 5, 9, 4, 3, 2, 1, 5];
    let signals = readings.map(|occupancy| control.observe(occupancy));
    assert_eq!(
        signals,
        [
            Signal::Unchanged,
            Signal::Unchanged,
            Signal::Pause,
            Signal::Unchanged,
            Signal::Unchanged,
            Signal::Unchanged,
            Signal::Resume,
            Signal::Unchanged,
            Signal::Pause,
        ],
    );
    assert_eq!(control.state(), FlowState::Paused);
}

#[test]
fn extremes_and_zero_low_watermark_work() {
    let mut control = Watermarks::new(0, high(u32::MAX)).expect("valid thresholds");
    assert_eq!(control.observe(u32::MAX), Signal::Pause);
    assert_eq!(control.observe(1), Signal::Unchanged);
    assert_eq!(control.observe(0), Signal::Resume);
    assert_eq!(control.state(), FlowState::Running);
}

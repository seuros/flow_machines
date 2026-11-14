use super::*;
use crate::test_support::{below, nonzero};
use matryona_machines::testing::{XorShift64, unchanged};

fn control(low: u32, high: u32) -> Watermarks {
    let Some(control) = Watermarks::new(low, nonzero(high)) else {
        panic!("{low} < {high} are valid thresholds");
    };
    control
}

#[test]
fn rejects_equal_or_inverted_thresholds() {
    assert_eq!(Watermarks::new(5, nonzero(5)), None);
    assert_eq!(Watermarks::new(6, nonzero(5)), None);
}

/// Thresholds, the readings with the signal each must emit, and the final state.
type Trace = (u32, u32, &'static [(u32, Signal)], FlowState);

#[test]
fn crossing_thresholds_emits_edges_only() {
    use Signal::{Pause, Resume, Unchanged};
    let cases: [Trace; 2] = [
        (
            2,
            5,
            &[
                (0, Unchanged),
                (4, Unchanged),
                (5, Pause),
                (9, Unchanged),
                (4, Unchanged),
                (3, Unchanged),
                (2, Resume),
                (1, Unchanged),
                (5, Pause),
            ],
            FlowState::Paused,
        ),
        // Extremes: a zero low watermark and the largest high one.
        (
            0,
            u32::MAX,
            &[(u32::MAX, Pause), (1, Unchanged), (0, Resume)],
            FlowState::Running,
        ),
    ];
    for (low, high, readings, state) in cases {
        let mut control = control(low, high);
        for &(occupancy, signal) in readings {
            assert_eq!(
                control.observe(occupancy),
                signal,
                "{low}..{high} at {occupancy}"
            );
        }
        assert_eq!(control.state(), state);
    }
}

#[test]
fn randomized_trace_matches_a_reference_state_machine() {
    let mut rng = XorShift64::new(0x4210_4210_4210_4210);
    let mut control = control(2, 7);
    let mut paused = false;
    for _ in 0..20_000 {
        let occupancy = below(&mut rng, 11);
        let next_paused = if paused {
            occupancy > 2
        } else {
            occupancy >= 7
        };
        let expected = match (paused, next_paused) {
            (false, true) => Signal::Pause,
            (true, false) => Signal::Resume,
            (false, false) | (true, true) => Signal::Unchanged,
        };
        if expected == Signal::Unchanged {
            assert_eq!(unchanged(&mut control, |c| c.observe(occupancy)), expected);
        } else {
            assert_eq!(control.observe(occupancy), expected);
        }
        paused = next_paused;
        let state = if paused {
            FlowState::Paused
        } else {
            FlowState::Running
        };
        assert_eq!(control.state(), state);
    }
}

# flow-machines

> **AMS Cerf**, NCC-4209, *Cerf*-class traffic escort.

Engine-room flight manual: count docking credits before admitting more traffic.

Decision-only flow control with no runtime, I/O, allocator, or dependencies.

## Capacity Credits

```rust
use core::num::NonZeroU32;
use flow_machines::{CreditWindow, Refusal};

# fn example() -> Option<()> {
let mut window = CreditWindow::new(NonZeroU32::new(2)?);
assert_eq!(window.try_acquire(NonZeroU32::MIN), Ok(()));
assert_eq!(window.try_acquire(NonZeroU32::MIN), Ok(()));
assert_eq!(
    window.try_acquire(NonZeroU32::MIN),
    Err(Refusal::Insufficient { available: 0 }),
);
assert_eq!(window.release(NonZeroU32::MIN), Ok(()));
assert_eq!(window.available(), 1);
assert_eq!(
    window.release(NonZeroU32::new(2)?),
    Err(Refusal::ExceedsInFlight { in_flight: 1 }),
);
# Some(())
# }
# assert_eq!(example(), Some(()));
```

Acquisition and release are all-or-nothing. A refused operation leaves state
unchanged; capacity never overflows. Units can be requests, messages, or bytes,
but must be consistent within a window.

This is **aggregate accounting**, not an owned-permit API. The adapter must return
each acquired credit exactly once, including on cancellation and failure.
A release larger than the outstanding balance is refused, but a repeated release
cannot be distinguished from a legitimate completion. Credits are local
state, not authenticated distributed permissions.

## High/Low Watermarks

```rust
use core::num::NonZeroU32;
use flow_machines::{Signal, Watermarks};

# fn example() -> Option<()> {
let mut control = Watermarks::new(2, NonZeroU32::new(5)?)?;
assert_eq!(control.observe(5), Signal::Pause);
assert_eq!(control.observe(4), Signal::Unchanged);
assert_eq!(control.observe(2), Signal::Resume);
# Some(())
# }
# assert_eq!(example(), Some(()));
```

`low < high` is required. A running producer pauses at `occupancy >= high`;
a paused producer resumes at `occupancy <= low`. The gap prevents oscillation.
Signals are edges: repeated observations in the same state return `Unchanged`.
Newly attached producers should inspect `state()` as well as future signals.

Watermarks do not enforce capacity. Combine them with credits: producers should
honor the signals, while credit acquisition remains the hard admission boundary.

## Contract

- **Refusals.** `Refusal` is the crate's only refusal type. `try_acquire`
  answers `Err(Refusal::Insufficient { available })` when `units` exceeds
  `available()`; `release` answers `Err(Refusal::ExceedsInFlight { in_flight })`
  when `units` exceeds `in_flight()`. Each carries the balance that was
  compared against. A refused call changes nothing: the window equals a clone
  taken before the call.
- **Honoured outcomes.** `Ok(())` from `try_acquire` and `release` means all
  `units` moved. `Watermarks::observe` never refuses: `Signal::Pause` and
  `Signal::Resume` are the two edges, `Signal::Unchanged` changed nothing.
- **Repeats.** None is recognised. Credits are an aggregate count with no
  owner or handle, so the window cannot prove that a call replays an earlier
  one. A repeated `release` is honoured while units remain in flight and
  refused with `ExceedsInFlight` once they do not.
- **Construction.** `CreditWindow::new` cannot fail. `Watermarks::new` answers
  `None` unless `low < high`.
- **Scopes and restarts.** The crate issues no ids and holds no handles, so
  there is no scope and nothing to resume. After a restart, build a new window
  and re-acquire what is still in flight.
- **Time.** No call takes or reads a clock. Every number is a count in the
  caller's unit.
- **Exhaustive.** `Refusal`, `FlowState` and `Signal` are exhaustive: a new
  variant is a breaking change every `match` must handle.

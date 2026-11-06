# flow-machines

> **AMS Cerf**, NCC-4209, *Cerf*-class traffic escort.

Engine-room flight manual: count docking credits before admitting more traffic.

Decision-only flow control with no runtime, I/O, allocator, or dependencies.

## Capacity Credits

```rust
use core::num::NonZeroU32;
use flow_machines::CreditWindow;

# fn example() -> Option<()> {
let mut window = CreditWindow::new(NonZeroU32::new(2)?);
assert!(window.try_acquire(NonZeroU32::MIN));
assert!(window.try_acquire(NonZeroU32::MIN));
assert!(!window.try_acquire(NonZeroU32::MIN));
assert!(window.release(NonZeroU32::MIN));
assert_eq!(window.available(), 1);
# Some(())
# }
# assert_eq!(example(), Some(()));
```

Acquisition and release are all-or-nothing. Failed operations leave state
unchanged; capacity never overflows. Units can be requests, messages, or bytes,
but must be consistent within a window.

This is **aggregate accounting**, not an owned-permit API. The adapter must return
each acquired credit exactly once, including on cancellation and failure.
A release larger than the outstanding balance is rejected, but repeated releases
cannot always be distinguished from legitimate completions. Credits are local
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

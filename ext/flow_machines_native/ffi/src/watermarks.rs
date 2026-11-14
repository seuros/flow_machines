//! `FlowMachines::Watermarks`: edge-triggered high/low occupancy decisions.

use std::cell::RefCell;

use flow_machines::{FlowState, Signal, Watermarks};
use magnus::typed_data::Obj;
use magnus::{Error, RModule, Ruby, Symbol, Value, function, method, prelude::*};
use matryona_machines::ruby::{integer, positive, symbol};

#[magnus::wrap(class = "FlowMachines::Watermarks", free_immediately, size)]
struct Wrapper(RefCell<Watermarks>);

impl Wrapper {
    fn new(ruby: &Ruby, low: Value, high: Value) -> Result<Self, Error> {
        let low = integer(low, "low")?;
        let control = Watermarks::new(low, positive(high, "high")?)
            .ok_or_else(|| Error::new(ruby.exception_range_error(), "low must be below high"))?;
        Ok(Self(RefCell::new(control)))
    }

    fn observe(ruby: &Ruby, rb_self: Obj<Self>, occupancy: Value) -> Result<Symbol, Error> {
        rb_self.check_frozen()?;
        let occupancy = integer(occupancy, "occupancy")?;
        let signal = rb_self.0.borrow_mut().observe(occupancy);
        // Finish the mutable borrow before allocating a Ruby value.
        Ok(symbol(
            ruby,
            match signal {
                Signal::Pause => "pause",
                Signal::Resume => "resume",
                Signal::Unchanged => "unchanged",
            },
        ))
    }

    fn state(ruby: &Ruby, rb_self: &Self) -> Symbol {
        let state = rb_self.0.borrow().state();
        symbol(
            ruby,
            match state {
                FlowState::Running => "running",
                FlowState::Paused => "paused",
            },
        )
    }
}

/// Register the watermark controller under the gem namespace.
pub fn define(ruby: &Ruby, module: RModule) -> Result<(), Error> {
    let class = module.define_class("Watermarks", ruby.class_object())?;
    class.undef_default_alloc_func();
    class.define_singleton_method("new", function!(Wrapper::new, 2))?;
    class.define_method("observe", method!(Wrapper::observe, 1))?;
    class.define_method("state", method!(Wrapper::state, 0))
}

//! `FlowMachines::Watermarks`: edge-triggered high/low occupancy decisions.

use std::cell::RefCell;

use flow_machines::{FlowState, Signal, Watermarks};
use magnus::scan_args::{get_kwargs, scan_args};
use magnus::typed_data::Obj;
use magnus::{Error, RHash, RModule, Ruby, Symbol, Value, function, method, prelude::*};

use crate::{Count, positive};

#[magnus::wrap(class = "FlowMachines::Watermarks", free_immediately, size)]
struct Wrapper(RefCell<Watermarks>);

impl Wrapper {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), (), (), (), RHash, ()>(args)?;
        let (Count(low), high) =
            get_kwargs::<_, (Count, Count), (), ()>(args.keywords, &["low", "high"], &[])?.required;
        let control = Watermarks::new(low, positive(ruby, high)?)
            .ok_or_else(|| Error::new(ruby.exception_arg_error(), "low must be less than high"))?;
        Ok(Self(RefCell::new(control)))
    }

    fn observe(ruby: &Ruby, rb_self: Obj<Self>, occupancy: Count) -> Result<Symbol, Error> {
        rb_self.check_frozen()?;
        let signal = rb_self.0.borrow_mut().observe(occupancy.0);
        // Finish the mutable borrow before allocating a Ruby value.
        let name = match signal {
            Signal::Pause => "pause",
            Signal::Resume => "resume",
            _ => "unchanged",
        };
        Ok(ruby.to_symbol(name))
    }

    fn state(ruby: &Ruby, rb_self: &Self) -> Symbol {
        let state = rb_self.0.borrow().state();
        ruby.to_symbol(if state == FlowState::Paused {
            "paused"
        } else {
            "running"
        })
    }

    fn paused(&self) -> bool {
        self.0.borrow().state() == FlowState::Paused
    }

    fn inspect(&self) -> String {
        format!(
            "#<FlowMachines::Watermarks state={}>",
            if self.paused() { "paused" } else { "running" }
        )
    }
}

/// Register the watermark controller under the gem namespace.
pub fn define(ruby: &Ruby, module: RModule) -> Result<(), Error> {
    let class = module.define_class("Watermarks", ruby.class_object())?;
    class.define_singleton_method("new", function!(Wrapper::new, -1))?;
    class.define_method("observe", method!(Wrapper::observe, 1))?;
    class.define_method("state", method!(Wrapper::state, 0))?;
    class.define_method("paused?", method!(Wrapper::paused, 0))?;
    class.define_method("inspect", method!(Wrapper::inspect, 0))
}

//! `FlowMachines::CreditWindow`: aggregate counts, not individually owned permits.

use std::cell::RefCell;
use std::num::NonZeroU32;

use flow_machines::{CreditWindow, Refusal};
use magnus::typed_data::Obj;
use magnus::{Error, RModule, Ruby, Symbol, Value, function, method, prelude::*};
use matryona_machines::ruby::{SymbolNames, positive, symbol};

use crate::Refusals;

#[magnus::wrap(class = "FlowMachines::CreditWindow", free_immediately, size)]
struct Wrapper(RefCell<CreditWindow>);

impl Wrapper {
    fn new(capacity: Value) -> Result<Self, Error> {
        let capacity = positive(capacity, "capacity")?;
        Ok(Self(RefCell::new(CreditWindow::new(capacity))))
    }

    fn try_acquire(ruby: &Ruby, rb_self: Obj<Self>, units: Value) -> Result<Symbol, Error> {
        Self::apply(ruby, rb_self, units, CreditWindow::try_acquire, "acquired")
    }

    fn release(ruby: &Ruby, rb_self: Obj<Self>, units: Value) -> Result<Symbol, Error> {
        Self::apply(ruby, rb_self, units, CreditWindow::release, "released")
    }

    /// Check and convert everything that can raise, then call the core once.
    fn apply(
        ruby: &Ruby,
        rb_self: Obj<Self>,
        units: Value,
        operation: fn(&mut CreditWindow, NonZeroU32) -> Result<(), Refusal>,
        honoured: &'static str,
    ) -> Result<Symbol, Error> {
        rb_self.check_frozen()?;
        let units = positive(units, "units")?;
        let outcome = operation(&mut rb_self.0.borrow_mut(), units);
        Ok(symbol(
            ruby,
            outcome.map_or_else(|refused| Refusals::name(&refused), |()| honoured),
        ))
    }

    fn capacity(&self) -> u32 {
        self.0.borrow().capacity()
    }

    fn in_flight(&self) -> u32 {
        self.0.borrow().in_flight()
    }

    fn available(&self) -> u32 {
        self.0.borrow().available()
    }
}

/// Register the credit-window class under the gem namespace.
pub fn define(ruby: &Ruby, module: RModule) -> Result<(), Error> {
    let class = module.define_class("CreditWindow", ruby.class_object())?;
    class.undef_default_alloc_func();
    class.define_singleton_method("new", function!(Wrapper::new, 1))?;
    class.define_method("try_acquire", method!(Wrapper::try_acquire, 1))?;
    class.define_method("release", method!(Wrapper::release, 1))?;
    class.define_method("capacity", method!(Wrapper::capacity, 0))?;
    class.define_method("in_flight", method!(Wrapper::in_flight, 0))?;
    class.define_method("available", method!(Wrapper::available, 0))
}

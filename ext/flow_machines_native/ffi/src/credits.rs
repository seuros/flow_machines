//! `FlowMachines::CreditWindow`: aggregate counts, not individually owned permits.

use std::cell::RefCell;

use flow_machines::CreditWindow;
use magnus::typed_data::Obj;
use magnus::{Error, RModule, Ruby, Symbol, function, method, prelude::*};

use crate::{Count, positive, refusal};

#[magnus::wrap(class = "FlowMachines::CreditWindow", free_immediately, size)]
struct Wrapper(RefCell<CreditWindow>);

impl Wrapper {
    fn new(ruby: &Ruby, capacity: Count) -> Result<Self, Error> {
        Ok(Self(RefCell::new(CreditWindow::new(positive(
            ruby, capacity,
        )?))))
    }

    fn try_acquire(ruby: &Ruby, rb_self: Obj<Self>, units: Count) -> Result<Symbol, Error> {
        rb_self.check_frozen()?;
        let units = positive(ruby, units)?;
        let outcome = rb_self.0.borrow_mut().try_acquire(units);
        Ok(ruby.to_symbol(outcome.map_or_else(refusal, |()| "acquired")))
    }

    fn release(ruby: &Ruby, rb_self: Obj<Self>, units: Count) -> Result<Symbol, Error> {
        rb_self.check_frozen()?;
        let units = positive(ruby, units)?;
        let outcome = rb_self.0.borrow_mut().release(units);
        Ok(ruby.to_symbol(outcome.map_or_else(refusal, |()| "released")))
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
    class.define_singleton_method("new", function!(Wrapper::new, 1))?;
    class.define_method("try_acquire", method!(Wrapper::try_acquire, 1))?;
    class.define_method("release", method!(Wrapper::release, 1))?;
    class.define_method("capacity", method!(Wrapper::capacity, 0))?;
    class.define_method("in_flight", method!(Wrapper::in_flight, 0))?;
    class.define_method("available", method!(Wrapper::available, 0))
}

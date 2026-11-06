//! `FlowMachines::CreditWindow`: aggregate counts, not individually owned permits.

use std::cell::RefCell;

use flow_machines::CreditWindow;
use magnus::scan_args::{get_kwargs, scan_args};
use magnus::typed_data::Obj;
use magnus::{Error, RHash, RModule, Ruby, Value, function, method, prelude::*};

use crate::{Count, positive};

#[magnus::wrap(class = "FlowMachines::CreditWindow", free_immediately, size)]
struct Wrapper(RefCell<CreditWindow>);

impl Wrapper {
    fn new(ruby: &Ruby, args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), (), (), (), RHash, ()>(args)?;
        let (capacity,) =
            get_kwargs::<_, (Count,), (), ()>(args.keywords, &["capacity"], &[])?.required;
        Ok(Self(RefCell::new(CreditWindow::new(positive(
            ruby, capacity,
        )?))))
    }

    fn try_acquire(ruby: &Ruby, rb_self: Obj<Self>, units: Count) -> Result<bool, Error> {
        rb_self.check_frozen()?;
        let units = positive(ruby, units)?;
        Ok(rb_self.0.borrow_mut().try_acquire(units))
    }

    fn release(ruby: &Ruby, rb_self: Obj<Self>, units: Count) -> Result<bool, Error> {
        rb_self.check_frozen()?;
        let units = positive(ruby, units)?;
        Ok(rb_self.0.borrow_mut().release(units))
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

    fn inspect(&self) -> String {
        let window = self.0.borrow();
        format!(
            "#<FlowMachines::CreditWindow capacity={} in_flight={} available={}>",
            window.capacity(),
            window.in_flight(),
            window.available()
        )
    }
}

/// Register the credit-window class under the gem namespace.
pub fn define(ruby: &Ruby, module: RModule) -> Result<(), Error> {
    let class = module.define_class("CreditWindow", ruby.class_object())?;
    class.define_singleton_method("new", function!(Wrapper::new, -1))?;
    class.define_method("try_acquire", method!(Wrapper::try_acquire, 1))?;
    class.define_method("release", method!(Wrapper::release, 1))?;
    class.define_method("capacity", method!(Wrapper::capacity, 0))?;
    class.define_method("in_flight", method!(Wrapper::in_flight, 0))?;
    class.define_method("available", method!(Wrapper::available, 0))?;
    class.define_method("inspect", method!(Wrapper::inspect, 0))
}

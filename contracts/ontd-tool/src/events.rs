use crate::Address;
use ontio_std::abi::EventBuilder;

pub fn new_pending_admin_event(new_pending_admin: &Address) {
    EventBuilder::new()
        .string("setPendingAdmin")
        .address(new_pending_admin)
        .notify();
}

pub fn new_admin_event(old_admin: &Address, new_pending_admin: &Address) {
    EventBuilder::new()
        .string("acceptAdmin")
        .address(old_admin)
        .address(new_pending_admin)
        .notify();
}

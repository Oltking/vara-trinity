use gstd::{msg, ActorId};

pub fn ensure_feeder(feeder_address: &ActorId) {
    let caller = msg::source();
    if caller != *feeder_address {
        panic!("Unauthorized: caller is not the feeder address");
    }
}

pub fn ensure_owner(owner: &ActorId) {
    let caller = msg::source();
    if caller != *owner {
        panic!("Unauthorized: caller is not the owner");
    }
}

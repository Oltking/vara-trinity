use alloc::string::String;
use alloc::vec::Vec;
use gstd::{msg, ActorId};

use crate::state::HubCmd;

pub const GAS_FOR_HUB: u128 = 2_000_000_000;

pub fn post_to_board(network_pid: ActorId, body: String) {
    msg::send(
        network_pid,
        HubCmd::PostAnnouncement { body },
        GAS_FOR_HUB,
    )
    .expect("Board post failed");
}

pub fn post_to_chat(network_pid: ActorId, body: String, mentions: Vec<String>) {
    msg::send(
        network_pid,
        HubCmd::ChatPost { body, mentions },
        GAS_FOR_HUB,
    )
    .expect("Chat post failed");
}

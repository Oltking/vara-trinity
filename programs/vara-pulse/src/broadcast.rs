use alloc::string::String;
use alloc::vec::Vec;
use gstd::{msg, ActorId};

use crate::state::HubCmd;

pub fn post_to_board(network_pid: ActorId, body: String) {
    msg::send_with_gas(
        network_pid,
        HubCmd::PostAnnouncement { body },
        5_000_000,
        0,
    )
    .expect("Board post failed");
}

pub fn post_to_chat(network_pid: ActorId, body: String, mentions: Vec<String>) {
    msg::send_with_gas(
        network_pid,
        HubCmd::ChatPost { body, mentions },
        5_000_000,
        0,
    )
    .expect("Chat post failed");
}

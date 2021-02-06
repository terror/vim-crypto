mod common;
mod crypto;
mod event;
mod messages;
mod request;

use crate::common::*;

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}

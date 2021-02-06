mod common;
mod crypto;
mod event;
mod messages;
mod request;

use crate::common::*;
#[macro_use]
extern crate prettytable;

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}

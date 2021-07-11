use crate::common::*;

mod client;
mod common;
mod crypto;
mod error;
mod event;
mod messages;

#[macro_use]
extern crate prettytable;

fn main() {
  let mut event_handler = EventHandler::new();
  match event_handler.recv() {
    Ok(()) => {}
    Err(e) => event_handler
      .nvim
      .command(&format!("echo {:?}", e.to_string()))
      .unwrap(),
  }
}

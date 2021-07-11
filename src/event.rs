use crate::common::*;

pub struct EventHandler {
  pub nvim: Neovim,
  client: Client,
}

impl EventHandler {
  pub fn new() -> EventHandler {
    let mut session = Session::new_parent().unwrap();
    session.set_infinity_timeout();

    let nvim = Neovim::new(session);
    let client = Client::new();

    EventHandler { nvim, client }
  }

  pub fn recv(&mut self) -> Result<(), Error> {
    let rec = self.nvim.session.start_event_loop_channel();

    for (event, values) in rec {
      match Messages::from(event) {
        Messages::CryptoTop => {
          self
            .nvim
            .command(&format!(
              "echo {:?}",
              Crypto::parse_top(self.client.get()?)?
            ))
            .context(error::NeovimError)?;
        }

        Messages::Crypto => {
          if let Some(symbol) = values.get(0) {
            self
              .nvim
              .command(&format!(
                "echo {:?}",
                Crypto::parse_one(self.client.get_one(&symbol.as_str().unwrap())?)?
              ))
              .context(error::NeovimError)?;
          }
        }

        Messages::Unknown(event) => {
          self
            .nvim
            .command(&format!("echo \"Unknown command: {}\"", event))
            .context(error::NeovimError)?;
        }
      }
    }

    Ok(())
  }
}

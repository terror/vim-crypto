use crate::common::*;

pub struct EventHandler {
    nvim: Neovim,
    crypto: Crypto,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();
        let nvim = Neovim::new(session);
        let crypto = Crypto::new();
        EventHandler { nvim, crypto }
    }

    pub fn recv(&mut self) {
        let rec = self.nvim.session.start_event_loop_channel();

        for (event, _) in rec {
            match Messages::from(event) {
                Messages::Crypto => {
                    let res = request::fetch();

                    let res = match res {
                        Ok(s) => s,
                        Err(err) => err.to_string(),
                    };

                    self.nvim
                        .command(&format!("echo {:?}", self.crypto.parse(res)))
                        .unwrap()
                }
                Messages::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}

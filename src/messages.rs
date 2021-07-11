use crate::common::*;

#[derive(Debug)]
pub enum Messages {
  Crypto,
  CryptoTop,
  Unknown(String),
}

impl From<String> for Messages {
  fn from(event: String) -> Self {
    match &event[..] {
      "CryptoTop" => Messages::CryptoTop,
      "Crypto" => Messages::Crypto,
      _ => Messages::Unknown(event),
    }
  }
}

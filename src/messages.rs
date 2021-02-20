pub enum Messages {
  Crypto,
  Unknown(String),
}

impl From<String> for Messages {
  fn from(event: String) -> Self {
    match &event[..] {
      "crypto" => Messages::Crypto,
      _ => Messages::Unknown(event),
    }
  }
}

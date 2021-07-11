use crate::common::*;

#[allow(dead_code)]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Failed to parse request data."))]
  ParseError { source: serde_json::Error },

  #[snafu(display("Failed to fetch data from url: {}", url))]
  RequestError {
    source: reqwest::Error,
    url:    String,
  },

  #[snafu(display("Failed to issue a command to `Neovim`."))]
  NeovimError {
    source: neovim_lib::neovim::CallError,
  },
}

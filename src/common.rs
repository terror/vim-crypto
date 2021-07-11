// stdlib
pub use std::fmt::{self, Display};

// dependencies
pub(crate) use {
  neovim_lib::{self, Neovim, NeovimApi, Session},
  prettytable::{format, Cell, Row, Table as PrettyTable},
  reqwest::{self, blocking},
  serde::Deserialize,
  serde_json,
  snafu::{ResultExt, Snafu},
};

// modules
pub(crate) use crate::error;

// structs and enums
pub(crate) use crate::{
  client::Client, crypto::Crypto, error::Error, event::EventHandler, messages::Messages,
  table::Table,
};

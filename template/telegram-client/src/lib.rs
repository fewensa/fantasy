//!
//! # telegram-client
//!
//! Telegram client for rust.
//!
//!
//! ## Note
//!
//! Note that you need [libtdjson.so][td] in your path for building and running your application. See also [rtdlib][rtdlib] for more details.
//!
//! # Examples
//!
//!
//! ```rust
//! # use telegram_client::api::Api;
//! # use telegram_client::client::Client;
//!
//! # fn start() {
//!   let api = Api::default();
//!   let mut client = Client::new(api.clone());
//!   let listener = client.listener();
//!
//!   listener.on_receive(|(api, json)| {
//!     debug!("receive {}", json);
//!     Ok(())
//!   });
//!
//!   client.daemon("telegram-rs");
//! # }
//!```
//!
//!
//!
//! [td]: https://github.com/tdlib/td
//! [rtdlib]: https://github.com/fewensa/rtdlib
//!

#[macro_use]
extern crate log;

mod rtd;
mod handler;
mod tip;

pub mod api;
pub mod client;
pub mod listener;
pub mod errors;


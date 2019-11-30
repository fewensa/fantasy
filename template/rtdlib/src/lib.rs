//!
//! # rtdlib
//! rtdlib is libtdjson binding.
//!
//! The more document of libtdjson you can see [td][td].
//!
//! # Examples
//!
//! ## Deserialize json to rtdlib types.
//!
//! ```rust
//! # use rtdlib::types::{UpdateAuthorizationState, RObject};
//!
//! let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#;
//! let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
//! assert_eq!("updateAuthorizationState", state.td_name());
//! let rjson = state.to_json();
//! assert!(rjson.is_ok(), true);
//! assert_eq!(json, rjson.unwrap());
//! ```
//!
//! ## Send json to libtdjson
//!
//! ```rust
//! # use rtdlib::tdjson::Tdlib;
//!
//! let tdlib = Tdlib::new();
//! let request = r#"{"@type": "getMe"}"#;
//! tdlib.send(request);
//! ```
//!
//! ```rust
//! # use rtdlib::tdjson::Tdlib;
//! # use rtdlib::types::{GetMe, RObject};
//!
//! let tdlib = Tdlib::new();
//! let get_me = GetMe::builder().build();
//! let json = get_me.to_json().expect("Bad json");
//! tdlib.send(&json[..]);
//! ```
//!
//! [td]: https://github.com/tdlib/td
//!

#[macro_use]
extern crate serde_derive;

pub mod tdjson;
pub mod types;
pub mod errors;

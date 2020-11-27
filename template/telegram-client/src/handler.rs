use rtdlib::types as rtd_types;

use crate::api::aevent::EventApi;
use crate::listener::Lout;
use crate::errors::TGError;
use crate::observer;
use crate::tip;
use rtdlib::types::*;

pub struct Handler<'a> {
  api: &'a EventApi,
  lout: &'a Lout,
  warn_unregister_listener: &'a bool,
}

// macro_rules! event_handler {
//   ($event_name:ident, $td_type:ident) => {
//     |api: &EventApi, lout: &Lout, json: &String, extra: &String| {
//       if let Some(ev) = lout.$event_name() {
//         match rtd_types::from_json::<rtd_types::$td_type>(json) {
//           Ok(t) => {
//             if let Err(_e) = ev((api, &t)) {
//               if let Some(ev) = lout.exception() { ev((api, &TGError::new("EVENT_HANDLER_ERROR"))); }
//             }
//             observer::notify(extra.to_string(), rtd_types::TdType::$td_type(t));
//           }
//           Err(e) => {
//             error!("{}", tip::data_fail_with_json(json));
//             eprintln!("{:?}", e);
//             if let Some(ev) = lout.exception() { ev((api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
//           }
//         }
//         return;
//       }
//       warn!("{}", tip::un_register_listener(stringify!($event_name)));
//     }
//   };
// }
//
// macro_rules! update_handler {
//   ($event_name:ident, $td_type:ident, $return_type:ident) => {
//     |api: &EventApi, lout: &Lout, json: &String, extra: &String| {
//       if let Some(ev) = lout.$event_name() {
//         let td_type = rtd_types::from_json::<$td_type>(json).unwrap();
//         match rtd_types::from_json::<$return_type>(json) {
//           Ok(t) => {
//             if let Err(_e) = ev((api, &td_type)) {
//               if let Some(ev) = lout.exception() { ev((api, &TGError::new("EVENT_HANDLER_ERROR"))); }
//             }
//             observer::notify(extra.to_string(), TdType::$return_type($return_type::$td_type(td_type)));
//           }
//           Err(e) => {
//             error!("{}", tip::data_fail_with_json(json));
//             eprintln!("{:?}", e);
//             if let Some(ev) = lout.exception() { ev((api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
//           }
//         }
//         return;
//       }
//       warn!("{}", tip::un_register_listener(stringify!($event_name)));
//     }
//   };
// }

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a EventApi, lout: &'a Lout, warn_unregister_listener: &'a bool) -> Self {
    Self {
      api,
      lout,
      warn_unregister_listener,
    }
  }

  pub fn handle(&self, json: &'a String) {
    let (td_type, extra) = match rtd_types::detect_td_type_and_extra(json) {
      (Some(t), Some(e)) => (t, Some(e)),
      (Some(t), None) => (t, None),
      (None, _) => {
        warn!("{}", tip::data_fail_with_json(json));
        return;
      }
    };

    if let Some(ev) = self.lout.receive() {
      if let Err(e) = ev((self.api, json)) {
        if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
      }
    }
    match rtd_types::from_json::<rtd_types::TdType>(json) {
      Ok(t) => {
        match self.lout.handle_type(self.api, &t) {
          Ok(true) => return,
          Ok(false) => {
            if *self.warn_unregister_listener {
              warn!("{}", tip::un_register_listener(stringify!(t)));
            }
          }
          Err(_err) => {
            if let Some(ev) = self.lout.exception() {
              ev((self.api, &TGError::new("EVENT_HANDLER_ERROR")));
            }
          }
        }

        // observer handler
        // todox: not have rtdtype, need add to rtdlib project, see api.rs#TdType
        if let Some(ext) = extra {
          observer::notify(ext, t);
        }
      }
      Err(e) => {
        error!("{}", tip::data_fail_with_json(json));
        // eprintln!("{:?}", e);
        error!("{:?}", e);
        if let Some(ev) = self.lout.exception() { ev((self.api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
      }
    }
  }
}

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
  pub(crate) fn new(api: &'a EventApi, lout: &'a Lout) -> Self {
    Self {
      api,
      lout,
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


    macro_rules! event_handler {
      ($event_name:ident, $td_type:ident) => {

        match rtd_types::from_json::<rtd_types::$td_type>(json) {
          Ok(t) => {

            // event handler
            if self.lout.is_support(&td_type) {
              if let Some(ev) = self.lout.$event_name() {
                if let Err(_e) = ev((self.api, &t)) {
                  if let Some(ev) = self.lout.exception() {
                    ev((self.api, &TGError::new("EVENT_HANDLER_ERROR")));
                  }
                }
                return;
              }
              warn!("{}", tip::un_register_listener(stringify!($event_name)));
            } else {
              warn!("{}", tip::not_have_listener(td_type));
            }

            // observer handler
            // todox: not have rtdtype, need add to rtdlib project, see api.rs#TdType
            if let Some(ext) = extra {
              observer::notify(ext, TdType::$td_type(t));
            }
          }
          Err(e) => {
            error!("{}", tip::data_fail_with_json(json));
            // eprintln!("{:?}", e);
            error!("{:?}", e);
            if let Some(ev) = self.lout.exception() { ev((self.api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
          }
        }

      };
    }

    match &td_type[..] {
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}      "{{token.name}}" => event_handler!({{token.name  | to_snake}}, {{token.name | to_camel}}),
{% endif %}{% endfor %}
{% for token in tokens %}{% if token.is_return_type %}      "{{token.name}}" => event_handler!({{token.name | to_snake}}, {{token.name | to_camel}}),
{% endif %}{% endfor %}
{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}      "{{token.name | to_snake | to_camel_lowercase}}" => event_handler!({{name | to_snake}}, {{token.name | to_camel}}),
{% endfor %}

      _ => {
        warn!("{}", tip::data_fail_with_json(json))
      }
    }
  }

}


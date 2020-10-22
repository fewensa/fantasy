use rtdlib::types as rtd_types;

use crate::api::Api;
use crate::listener::Lout;
use crate::errors::TGError;
use crate::observer::Observer;
use crate::tip;

pub struct Handler<'a> {
  api: &'a Api,
  lout: &'a Lout,
  observer: &'a Observer,
}

macro_rules! event_handler {
  ($event_name:ident, $td_type:ident, $return_type:ident) => {
    |api: &Api, lout: &Lout, observer: &Observer, json: &String, extra: &String| async {
      if let Some(ev) = lout.$event_name() {
        match rtd_types::from_json::<rtd_types::$td_type>(json) {
          Ok(t) => {
            observer.notify(extra.to_string(), rtd_types::TdType::$return_type(t)).await;
            if let Err(_e) = ev((api, &t)) {
              if let Some(ev) = lout.exception() { ev((api, &TGError::new("EVENT_HANDLER_ERROR"))); }
            }
          }
          Err(e) => {
            error!("{}", tip::data_fail_with_json(json));
            eprintln!("{:?}", e);
            if let Some(ev) = lout.exception() { ev((api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
          }
        }
        return;
      }
      warn!("{}", tip::un_register_listener(stringify!($event_name)));
    }
  };
}

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a Api, lout: &'a Lout, observer: &'a Observer) -> Self {
    Self {
      api,
      lout,
      observer,
    }
  }

  pub async fn handle(&self, json: &'a String) {
    let (td_type, extra) = match rtd_types::detect_td_type_and_extra(json) {
      (Some(t), Some(e)) => (t, Some(e)),
      (Some(t), None) => (t, None),
      (None, _) => {
        warn!("{}", tip::data_fail_with_json(json));
        return;
      }
    };
    if !self.lout.is_support(&td_type) {
      warn!("{}", tip::not_have_listener(td_type));
      return;
    }

    if let Some(ev) = self.lout.receive() {
      if let Err(e) = ev((self.api, json)) {
        if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
      }
    }

    match &td_type[..] {
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}      "{{token.name}}" => event_handler!({{token.name  | to_snake}}, {{token.name | to_camel}}, {{token.blood}})(self.api, self.lout, self.observer, json, &extra.unwrap()).await,
{% endif %}{% endfor %}
{% for token in tokens %}{% if token.is_return_type %}      "{{token.name}}" => event_handler!({{token.name | to_snake}}, {{token.name | to_camel}}, {{token.blood}})(self.api, self.lout, self.observer, json, &extra.unwrap()).await,
{% endif %}{% endfor %}
{% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}      "{{token.name | to_snake | to_camel_lowercase}}" => event_handler!({{name | to_snake}}, {{token.name | to_camel}}, {{token.blood}})(self.api, self.lout, self.observer, json, &extra.unwrap()).await,
{% endfor %}
      _ => {
        warn!("{}", tip::data_fail_with_json(json))
      }
    }
  }

}


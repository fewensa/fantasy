use rtdlib::types::Update;
use rtdlib::types::RObject;

use crate::api::Api;
use crate::listener::{Listener, Lout};
use crate::errors::TGError;
use crate::tip;

pub struct Handler<'a> {
  api: &'a Api,
  lout: &'a Lout,
}

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a Api, lout: &'a Lout) -> Self {
    Self {
      api,
      lout,
    }
  }

  pub fn handle(&self, json: &'a String) {
    match Update::from_json(json) {
      Ok(update) => {

        if let Some(ev) = self.lout.receive() {
          ev((self.api, &update));
        }

        if !self.lout.is_support(update.td_name()) {
          warn!("{}", tip::not_have_listener(update.td_name()));
          return;
        }

        {% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
        update.on_{{token.name | td_remove_prefix(prefix='Update') | to_snake}}(|t| {
          if let Some(ev) = self.lout.{{token.name | td_remove_prefix(prefix='Update') | to_snake}}() {
            if let Err(e) = ev((self.api, t)) {
              if let Some(ev) = self.lout.exception() {
                ev((self.api, &e));
              }
            }
            return;
          }
          warn!("{}", tip::un_register_listener(update.td_name()));
        });
        {% endif %}{% endfor %}
      },
      Err(e) => {
        if let Some(ev) = self.lout.exception() {
          let mut ex = TGError::new("CONVERT_JSON_FAIL");
          ex.set_message(json);
          ev((self.api, &ex));
        }
      }
    }
  }
}


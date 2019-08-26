use rtdlib::types as rtd_types;
use rtdlib::types::{RObject, Update, Error};

use crate::api::Api;
use crate::listener::{Listener, Lout};
use crate::errors::TGError;
use crate::tip;

pub struct Handler<'a> {
  api: &'a Api,
  lout: &'a Lout,
}

macro_rules! event_update {
  ($event_name:ident, $on:ident) => {
    |api: &Api, lout: &Lout, update: &Update| {
      update.$on(|t| {
        if let Some(ev) = lout.$event_name() {
          if let Err(e) = ev((api, t)) {
            if let Some(ev) = lout.exception() { ev((api, &e)); }
          }
        }
      });
    }
  };
}

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a Api, lout: &'a Lout) -> Self {
    Self {
      api,
      lout,
    }
  }

  pub fn handle(&self, json: &'a String) {

    let td_type = match rtd_types::detect_td_type(json) {
      Some(t) => t,
      None => {
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

    match Update::from_json(json) {
      Ok(update) => {
        self.handler_update(&update);
        return;
      }
      Err(e) => {
        warn!("{}\n{:?}", tip::data_fail_with_json(json), e);
      }
    };

    match &td_type[..] {
      {% for name, td_type in listener %}{% set token = find_token(token_name = td_type) %}
      "{{token.name | to_snake | to_camel_lowercase}}" => {
        if let Some(ev) = self.lout.{{name | to_snake}}() {
          if let Ok(t) = rtd_types::from_json::<rtd_types::{{token.name | to_camel}}>(json) {
            if let Err(e) = ev((self.api, &t)) {
              if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
            }
          }
        }
      }
      {% endfor %}
      _ => {
        warn!("{}", tip::data_fail_with_json(json))
      }
    }

  }

  fn handler_update(&self, update: &Update) {

    {% for token in tokens %}{% if token.blood and token.blood == 'Update' %} {% set ev_name=token.name | td_remove_prefix(prefix='Update') | to_snake %}
    if update.is_{{ev_name}}() { event_update!({{ev_name}}, on_{{ev_name}})(self.api, self.lout, update); return; }
    {% endif %}{% endfor %}

    {#
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
    #}

  }

}


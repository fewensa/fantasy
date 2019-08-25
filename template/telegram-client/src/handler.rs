use rtdlib::types::{Update, Error};
use rtdlib::types::RObject;

use crate::api::Api;
use crate::listener::{Listener, Lout};
use crate::errors::TGError;
use crate::tip;

pub struct Handler<'a> {
  api: &'a Api,
  lout: &'a Lout,
}

macro_rules! handler_event {
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
    let update_event = Update::from_json(json);
    if let Ok(update) = update_event {
      self.handler_update(&update);
      return;
    }

    let mut already_handler = false;
    let ok_event = rtdlib::types::Ok::from_json(json);
    if let Ok(ok) = ok_event {
      already_handler = true;
      self.handler_ok(&ok);
      return;
    } else {
      let error_event = rtdlib::types::Error::from_json(json);
      if let Ok(error) = error_event {
        already_handler = true;
        self.handler_error(&error);
        return;
      }
    }
    if already_handler { return; }

    warn!("{}", tip::data_fail_with_json(json));
  }

  fn handler_error(&self, error: &rtdlib::types::Error) {
    if let Some(ev) = self.lout.error() {
      if let Err(e) = ev((self.api, error)) {
        if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
      }
      return;
    }
    warn!("{}", tip::un_register_listener(error.td_name()));
  }

  fn handler_ok(&self, ok: &rtdlib::types::Ok) {
    if let Some(ev) = self.lout.ok() {
      if let Err(e) = ev((self.api, ok)) {
        if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
      }
      return;
    }
    warn!("{}", tip::un_register_listener(ok.td_name()));
  }

  fn handler_update(&self, update: &Update) {

    if let Some(ev) = self.lout.receive() {
      ev((self.api, update));
    }

    if !self.lout.is_support(update.td_name()) {
      warn!("{}", tip::not_have_listener(update.td_name()));
      return;
    }

    {% for token in tokens %}{% if token.blood and token.blood == 'Update' %} {% set ev_name=token.name | td_remove_prefix(prefix='Update') | to_snake %}
    if update.is_{{ev_name}}() { handler_event!({{ev_name}}, on_{{ev_name}})(self.api, self.lout, update); return; }
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


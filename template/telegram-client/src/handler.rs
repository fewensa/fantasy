use rtdlib::types::Update;
use crate::types::*;

use crate::api::Api;
use crate::handler::handler_receive::ReceiveHandler;
use crate::listener::{Listener, Lout};
use crate::errors;
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
        self.lout.receive().map(|env| {
          env((self.api, &update));
        });
        {% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
        update.on_{{token.name | td_remove_prefix(prefix='Update') | to_snake}}(|t| {
          self.lout.{{token.name | td_remove_prefix(prefix='Update') | to_snake}}()
            .map_or_else(|| {
              warn!(tip::not_have_listener(t.td_name()));
            }, |env| {
              env((self.api, &update));
            })
        });
        {% endif %}{% endfor %}
      },
      Err(e) => {
        self.lout.exception().map(|ev| {
          let ex = TGException::new(json, e);
          ev((self.api, &ex))
        });
      }
    }
  }
}


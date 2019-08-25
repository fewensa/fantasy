use core::borrow::Borrow;
use std::sync::Arc;

use rtdlib::tdjson;
use rtdlib::errors::*;
use rtdlib::types::*;

use crate::tip;


#[derive(Debug, Clone)]
pub struct Api {
  tdlib: Arc<tdjson::Tdlib>
}

impl Default for Api {
  fn default() -> Self {
    Api::new(tdjson::Tdlib::new())
  }
}

impl Api {
  pub fn new(tdlib: tdjson::Tdlib) -> Self {
    Self { tdlib: Arc::new(tdlib) }
  }

  #[doc(hidden)]
  pub fn tdlib(&self) -> &tdjson::Tdlib {
    self.tdlib.borrow()
  }

  pub fn send<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<()> {
    let json = fnc.to_json()?;
    info!("===> {}", json);
    self.tdlib.send(&json[..]);
    Ok(())
  }

  pub fn receive(&self, timeout: f64) -> Option<String> {
    let receive = self.tdlib.receive(timeout);
    if receive.is_some() {
      info!("<=== {}", receive.clone().map_or("NONE".to_string(), |v| v));
    }
    receive
  }

  pub fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>> {
    let json = fnc.to_json()?;
    info!("===>>> {}", json);
    Ok(self.tdlib.execute(&json[..]))
  }

{% for token in tokens %}{% if token.type_ == 'Function' %}
  pub fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> {% if token.blood and token.blood == 'Ok' %}RTDResult<()>{% else %}RTDResult<{{token.blood}}>{% endif %} {
  {% if token.blood and token.blood == 'Ok' %}  self.send({{token.name | to_snake}}.as_ref()) {% else %}
    match self.execute({{token.name | to_snake}}.as_ref())? {
      Some(json) => Ok({{token.blood}}::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  {% endif %}
  }
{% endif %}{% endfor %}

}

use core::borrow::Borrow;
use std::sync::Arc;

use regex::Regex;
use rtdlib::errors::*;
use rtdlib::tdjson;
use rtdlib::types::*;

use crate::tip;

#[derive(Debug, Clone)]
pub struct ApiBuilder {
  inner: Api
}

impl ApiBuilder {
  pub fn new() -> Self {
    Self {
      inner: Api {
        tdlib: Arc::new(tdjson::Tdlib::new()),
        log: true,
        unsafe_log: false
      }
    }
  }

  pub fn build(&self) -> Api {
    self.inner.clone()
  }

  fn tdlib(&mut self, tdlib: tdjson::Tdlib) -> &mut Self{
    self.inner.tdlib = Arc::new(tdlib);
    self
  }

  pub fn log(&mut self, open: bool) -> &mut Self {
    self.inner.log = open;
    self
  }

  pub fn unsafe_log(&mut self, unsafe_log: bool) -> &mut Self {
    self.inner.unsafe_log = unsafe_log;
    self
  }
}


#[derive(Debug, Clone)]
pub struct Api {
  tdlib: Arc<tdjson::Tdlib>,
  log: bool,
  unsafe_log: bool,
}

impl Default for Api {
  fn default() -> Self {
    ApiBuilder::new().build()
  }
}

impl Api {

  pub fn builder() -> ApiBuilder {
    ApiBuilder::new()
  }

  pub fn new(tdlib: tdjson::Tdlib) -> Self {
    ApiBuilder::new().tdlib(tdlib).build()
  }

  #[doc(hidden)]
  pub fn tdlib(&self) -> &tdjson::Tdlib {
    self.tdlib.borrow()
  }

  fn safe_log(&self, text: &String) -> String {
    if self.unsafe_log {
      return text.clone();
    }
    let regex_api_id = Regex::new(r#"api_id":\d*"#).expect("Regex fail");
    let hide_api_id = regex_api_id.replace_all(text, r#"api_id":"****""#);
    let regex_api_hash = Regex::new(r#"api_hash":"[0-9|a-f]*""#).expect("Regex fail");
    let hide_api_hash = regex_api_hash.replace_all(&hide_api_id, r#"api_hash":"**********""#);
    hide_api_hash.into_owned()
  }

  pub fn send<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<()> {
    let json = fnc.to_json()?;
    if self.log {
      info!("===> {}", self.safe_log(&json));
    }
    self.tdlib.send(&json[..]);
    Ok(())
  }

  pub fn receive(&self, timeout: f64) -> Option<String> {
    let receive = self.tdlib.receive(timeout);
    if self.log {
      if receive.is_some() {
        info!("<=== {}", receive.clone().map_or("<NONE>".to_string(), |v| self.safe_log(&v)));
      }
    }
    receive
  }

  pub fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>> {
    let json = fnc.to_json()?;
    if self.log {
      info!("===>>> {}", self.safe_log(&json));
    }
    Ok(self.tdlib.execute(&json[..]))
  }

{#
  // now don't know which function is synchronously function, so, not use this block.
{% for token in tokens %}{% if token.type_ == 'Function' %}
  pub fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> {% if token.blood and token.blood == 'Ok' %}RTDResult<{{token.blood}}>{% else %}RTDResult<()>{% endif %} {
    {% if token.blood and token.blood == 'Ok' %}
    match self.execute({{token.name | to_snake}}.as_ref())? {
      Some(json) => Ok({{token.blood}}::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
    {% else %}  self.send({{token.name | to_snake}}.as_ref()){% endif %}
  }
{% endif %}{% endfor %}
#}

{% for token in tokens %}{% if token.type_ == 'Function' %}
  pub fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> RTDResult<()> {
    self.send({{token.name | to_snake}}.as_ref())
  }
{% endif %}{% endfor %}


}

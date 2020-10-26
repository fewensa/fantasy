use crate::api::Api;
use crate::observer::{subscribe, unsubscribe};
use rtdlib::types::*;
use rtdlib::errors::{RTDResult, RTDError};
use futures::StreamExt;

pub struct AsyncApi {
  api: Api,
}

impl AsyncApi {
  pub fn new(api: Api) -> Self {
    Self { api}
  }

  #[doc(hidden)]
  pub fn api(&self) -> &Api {
    &self.api
  }


{% for token in tokens %}
  {% if token.type_ == 'Function' %}
  pub async fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> RTDResult<{{token.blood | to_camel}}> {
    let mut rec = subscribe({{token.name | to_snake}}.as_ref().extra().to_string());
    self.api.send({{token.name | to_snake}}.as_ref())?;
    let {{token.blood | to_snake}} = rec.next().await.unwrap();
    unsubscribe({{token.name | to_snake}}.as_ref().extra());
    match {{token.blood | to_snake}} {
      TdType::{{token.blood | to_camel}}({{token.blood | to_snake}}) => {Ok({{token.blood | to_snake}})}
      _ => {Err(RTDError::Custom("invalid type"))}
    }
  }
  {% endif %}
{% endfor %}

}

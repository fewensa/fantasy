use crate::api::Api;
use crate::observer;
use rtdlib::types::*;
use rtdlib::errors::{RTDResult, RTDError};
use futures::StreamExt;

macro_rules! async_caller {
  ($td_type:ident) => {
    async fn async_caller<Fnc: RFunction>(api: &Api, fnc: Fnc) -> RTDResult<$td_type> {
      let extra = fnc.extra()
        .ok_or(RTDError::Custom("invalid libtd response type, not have `extra` field"))?;
      let mut rec = observer::subscribe(&extra);
      api.send(&fnc)?;
      let val = rec.next().await;
      observer::unsubscribe(&extra);
      match val {
        Some(TdType::$td_type(v)) => { Ok(v) }
        // Some(TdType::Error(v)) => { Err(RTDError::Custom(v.message())) }
        Some(TdType::Error(v)) => { Err(RTDError::custom("todo: real error message")) }
        _ => { Err(RTDError::custom("invalid libtd response type, unexpected `extra` field")) }
      }
    }
  }
}

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

{% for token in tokens %}{% if token.type_ == 'Function' %}
  pub async fn {{token.name | to_snake}}<C: AsRef<{{token.name | to_camel}}>>(&self, {{token.name | to_snake}}: C) -> RTDResult<{{token.blood | to_camel}}> {
    async_caller!({{token.blood | to_camel}});
    async_caller(&self.api, {{token.name | to_snake}}.as_ref()).await
  }
{% endif %}{% endfor %}

}

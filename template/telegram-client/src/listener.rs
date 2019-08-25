use std::sync::Arc;

use rtdlib::types::*;
use crate::errors::*;
use crate::api::Api;


/// Telegram client event listener
#[derive(Clone, Default)]
pub struct Listener {
  exception: Option<Arc<Fn((&Api, &TGError)) + Send + Sync + 'static>>,

  receive: Option<Arc<Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static>>,
  ok: Option<Arc<Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static>>,
  error: Option<Arc<Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static>>,

{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}  {{token.name | td_remove_prefix(prefix='Update') | to_snake}}: Option<Arc<Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>>,
{% endif %}{% endfor %}
}


impl Listener {
  pub fn new() -> Self { Listener::default() }

  pub(crate) fn has_receive_listen(&self) -> bool { self.receive.is_some() }

  pub(crate) fn lout(&self) -> Lout { Lout::new(self.clone()) }


  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static {
    self.receive = Some(Arc::new(fnc));
    self
  }

  /// when telegram client throw exception
  pub fn on_exception<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &TGError)) + Send + Sync + 'static {
    self.exception = Some(Arc::new(fnc));
    self
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn on_ok<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static {
    self.ok = Some(Arc::new(fnc));
    self
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn on_error<F>(&mut self, fnc: F) -> &mut Self where F: Fn(((&Api, &Error))) -> TGResult<()> + Send + Sync + 'static {
    self.error = Some(Arc::new(fnc));
    self
  }



{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn on_{{token.name | td_remove_prefix(prefix='Update') | to_snake}}<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static {
    self.{{token.name | td_remove_prefix(prefix='Update') | to_snake}} = Some(Arc::new(fnc));
    self
  }
{% endif %}{% endfor %}
}


/// Get listener
pub struct Lout {
  listener: Listener,
  supports: Vec<&'static str>
}

impl Lout {
  fn new(listener: Listener) -> Self {
    let supports = vec![
{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}      "{{token.name}}",
{% endif %}{% endfor %}
    ];
    Self { listener, supports }
  }

  pub fn is_support<S: AsRef<str>>(&self, name: S) -> bool {
    self.supports.iter()
      .find(|&&item| item == name.as_ref())
      .is_some()
  }

  /// when telegram client throw exception
  pub fn exception(&self) -> &Option<Arc<Fn((&Api, &TGError)) + Send + Sync + 'static>> {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(&self) -> &Option<Arc<Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.receive
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn ok(&self) -> &Option<Arc<Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.ok
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn error(&self) -> &Option<Arc<Fn(((&Api, &Error))) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.error
  }



{% for token in tokens %}{% if token.blood and token.blood == 'Update' %}
  /// {{token.description}}
  pub fn {{token.name | td_remove_prefix(prefix='Update') | to_snake}}(&self) -> &Option<Arc<Fn((&Api, &{{token.name | to_camel}})) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.{{token.name | td_remove_prefix(prefix='Update') | to_snake}}
  }
{% endif %}{% endfor %}
}



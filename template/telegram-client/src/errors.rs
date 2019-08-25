use std::any::Any;

use std::{io, fmt, error};
use std::fmt::Debug;

pub trait TGDatable: Debug {
  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct TGError {
  key: &'static str,
  message: Option<String>,
  data: HasMap<String, Box<TGDatable>>,
  context: Option<Box<std::error::Error>>
}


impl fmt::Display for TGError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]: {}{}", self.key, self.message, self.context.map_or("".to_string(), |e| format!(" => {:?}", e)))
  }
}

impl error::Error for TGError {
  fn description(&self) -> &str {
    self.key
  }

  fn cause(&self) -> Option<&error::Error> {
    None
  }
}


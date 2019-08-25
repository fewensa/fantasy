
use rtdlib::errors::RTDError;

#[derive(Debug, Clone)]
pub struct TGException {
  json: String,
  e: RTDError,
}

impl TGException {
  pub fn new<S: AsRef<str>>(json: S, e: RTDError) -> Self {
    Self { json: json.as_ref().to_string(), e }
  }
  pub fn json(&self) -> &String { &self.json }
  pub fn e(&self) -> &RTDError { &self.e }
}

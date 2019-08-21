use tl_parser::types::TLTokenGroup;
use std::path::Path;

pub struct RTD<'a, P: AsRef<Path>> {
  base_path: P,
  tokens: &'a Vec<TLTokenGroup>
}

impl<'a, P: AsRef<Path>> RTD<'a, P> {
  pub fn new(base_path: P, tokens: &'a Vec<TLTokenGroup>) -> Self {
    Self { base_path, tokens }
  }


  pub fn generate(&self) -> Result<(), failure::Error> {
    Ok(())
  }
}




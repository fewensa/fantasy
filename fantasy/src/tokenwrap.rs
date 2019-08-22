use tl_parser::types::TLTokenGroup;



lazy_static! {
  static ref SKIP_TYPES: Vec<&'static str> = {
    vec![
      "double",
      "string",
      "int32",
      "int53",
      "int64",
      "bytes",
      "boolFalse",
      "boolTrue",
      "vector",
    ]
  };
}

#[derive(Debug)]
pub struct TokenWrap {
  tokens: Vec<TLTokenGroup>
}

impl TokenWrap {
  pub fn new(tokens: Vec<TLTokenGroup>) -> Self {
    Self { tokens }
  }

  pub fn tokens(&self) -> &Vec<TLTokenGroup> {
    &self.tokens
  }

  pub fn all_types(&self) -> Vec<String> {
    self.tokens.iter()
      .filter(|&item| !SKIP_TYPES.contains(&&item.name()[..]))
      .map(|item| item.name())
      .collect()
  }
}

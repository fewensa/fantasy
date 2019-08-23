use case::CaseExt;

use tl_parser::types::*;

use crate::tdfill::TDTypeFill;

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
  tokens: Vec<TLTokenGroup>,
  tdtypefill: TDTypeFill
}

impl TokenWrap {
  pub fn new(tokens: Vec<TLTokenGroup>, tdtypefill: TDTypeFill) -> Self {
    Self { tokens, tdtypefill }
  }

  pub fn tokens(&self) -> &Vec<TLTokenGroup> {
    &self.tokens
  }

  /// all td types
  pub fn all_types(&self) -> Vec<String> {
    self.tokens.iter()
      .filter(|&item| !SKIP_TYPES.contains(&&item.name()[..]))
      .map(|item| item.name())
      .collect()
  }

  /// is skip type
  pub fn is_skip_type(&self, type_name: String) -> bool {
    SKIP_TYPES.contains(&&type_name[..])
  }

  /// The file where td type is located
  pub fn which_file(&self, type_name: String) -> String {
    let token = self.tokens.iter()
      .find(|&item| item.name() == type_name)
      .expect(&format!("Can't found this type -> {}", type_name));
    if token.type_() == TLTokenGroupType::Trait {
      return token.name().to_snake();
    }
    if token.type_() == TLTokenGroupType::Function {
      return "functions".to_string();
    }
    match token.blood() {
      Some(blood) => {
        if blood == type_name {
          type_name.to_snake()
        } else {
          blood.to_snake()
        }
      },
      None => token.name().to_snake()
    }
  }

}

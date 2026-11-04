use std::path::Path;

use tl_parser::parser::parser::TLParser;
use tl_parser::types::TLTokenGroupType;

fn tokens() -> Vec<tl_parser::types::TLTokenGroup> {
  let path = Path::new("../").join("schema/v1.8.69/td_api.tl");
  TLParser::new(path).parse().expect("parse td_api.tl")
}

#[test]
fn test_td() {
  let tokens = tokens();
  assert!(!tokens.is_empty());

  let double = tokens
    .iter()
    .find(|token| token.name() == "double")
    .expect("double");
  assert_eq!(double.blood(), Some("Double".to_string()));

  let string = tokens
    .iter()
    .find(|token| token.name() == "string")
    .expect("string");
  assert_eq!(string.blood(), Some("String".to_string()));

  assert!(tokens.iter().any(|token| token.name() == "testCallEmpty"));
  assert!(tokens.iter().any(|token| token.name() == "tonNft"));
  assert!(tokens.iter().any(|token| token.name() == "proxyTypeWeb"));
  assert!(tokens.iter().any(|token| {
    token.type_() == TLTokenGroupType::Function && token.name() == "setTdlibParameters"
  }));
}

use std::path::Path;

use tl_parser::parser::parser::TLParser;

fn main() {
  let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../schema/v1.8.69/td_api.tl");
  let parser = TLParser::new(path);
  match parser.parse() {
    Ok(tokens) => println!("parsed {} tokens", tokens.len()),
    Err(e) => panic!("{:?}", e),
  }
}

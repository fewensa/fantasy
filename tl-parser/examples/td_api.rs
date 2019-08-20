use std::fs::File;

use failure::Error;

use tl_parser::parser::*;
use tl_parser::parser::parser::TLParser;

#[macro_use]
extern crate log;

fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);

  let path = toolkit::path::root_dir().join("schema/master/td_api.tl");
  let parser = TLParser::new(path);
  match parser.parse() {
    Ok(ret) => debug!("finish"),
    Err(e) => panic!("{:?}", e)
  }
}

#[macro_use]
extern crate failure;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate log;

use std::{env, fs};
use std::path::{Path, PathBuf};

use config::Config;
use rtd::RTD;
use tl_parser::parser::parser::TLParser;

mod config;
mod rtd;


fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);

//  let mut config = Config::default();

  let project_path = Path::new("./");

  let path = project_path.join("schema/master/td_api.tl");
  let tokens = TLParser::new(path).parse().unwrap();

//  debug!("{:?} -> {}", path_rtd, path_rtd.exists());
//  RTD::new(path_rtd, &tokens).generate().unwrap();
}

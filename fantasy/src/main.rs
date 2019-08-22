#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate typed_builder;

use std::path::{Path, PathBuf};

use tera::Tera;

use cycle::*;
use rtd::RTD;
use tl_parser::parser::parser::TLParser;
use tokenwrap::TokenWrap;

mod cycle;
mod rtd;
mod tokenwrap;
mod terafill;

fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);


  let project_path = Path::new("./");

  let config: Config = Config::builder()
    .path_rtd(project_path.join("../rtdlib"))
    .path_telegram_client(project_path.join("../telegram-client"))
    .path_template(project_path.join("template"))
    .file_tl(project_path.join("schema/master/td_api.tl"))
    .build();

  let mut tera = Tera::new("template/**/*").expect("Can not create Tera template engine.");

  terafill::filter(&mut tera);

  let renderer = Renderer::builder().tera(tera).build();

  let tokens = TLParser::new(config.file_tl()).parse().unwrap();
  let tknwrap = TokenWrap::new(tokens);

  let cycle: Cycle = Cycle::builder()
    .config(config)
    .tknwrap(tknwrap)
    .renderer(renderer)
    .build();

  RTD::new(&cycle).generate().unwrap();
}

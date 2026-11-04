use std::path::{Path, PathBuf};

use anyhow::Context;
use tera::Tera;

use cycle::*;
use rtd::RTD;
use tgclient::TGClient;
use tl_parser::parser::parser::TLParser;
use tokenwrap::TokenWrap;

mod cycle;
mod rtd;
mod tdfill;
mod terafill;
mod tgclient;
mod tokenwrap;

fn main() {
  simple_logger::SimpleLogger::new()
    .with_level(log::LevelFilter::Info)
    .init()
    .expect("logger");

  let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
  let rtd_out = std::env::var("RTD_OUT")
    .map(PathBuf::from)
    .unwrap_or_else(|_| workspace.join("target/rtd-out"));
  let tl_file = std::env::var("TL_FILE")
    .map(PathBuf::from)
    .unwrap_or_else(|_| workspace.join("schema/v1.8.69/td_api.tl"));

  let tdtypefill = tdfill::TDTypeFill::new(workspace.join("schema/td_type_fill.toml"))
    .expect("read schema/td_type_fill.toml");

  let config = Config::builder()
    .path_rtd(rtd_out)
    .path_telegram_client(
      std::env::var("TG_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace.join("target/telegram-client-out")),
    )
    .path_template(workspace.join("template"))
    .file_tl(tl_file)
    .build();

  let mut tera = Tera::new(&format!("{}/**/*", config.path_template().display()))
    .expect("Can not create Tera template engine.");

  let tokens = TLParser::new(config.file_tl())
    .parse()
    .expect("parse td_api.tl");
  let tknwrap = TokenWrap::new(tokens, tdtypefill);

  terafill::fill(&mut tera, tknwrap.clone()).expect("register template helpers");

  let renderer = Renderer::builder().tera(tera).build();
  let cycle = Cycle::builder()
    .config(config)
    .tknwrap(tknwrap)
    .renderer(renderer)
    .build();

  RTD::new(&cycle).generate().expect("generate rtdlib");
  if std::env::var_os("TG_OUT").is_some() {
    TGClient::new(&cycle)
      .generate()
      .expect("generate telegram-client");
  }

  rustfmt_tree(&cycle.config().path_rtd().join("src")).expect("rustfmt rtdlib output");
  if std::env::var_os("TG_OUT").is_some() {
    rustfmt_tree(&cycle.config().path_telegram_client().join("src"))
      .expect("rustfmt telegram-client output");
  }
}

/// Format generated Rust the same way rtdlib does (`edition = 2021`, two-space indent).
fn rustfmt_tree(dir: &Path) -> anyhow::Result<()> {
  let mut files = Vec::new();
  collect_rs(dir, &mut files)?;
  files.sort();
  if files.is_empty() {
    return Ok(());
  }
  for chunk in files.chunks(64) {
    let status = std::process::Command::new("rustfmt")
      .arg("--edition")
      .arg("2021")
      .arg("--config")
      .arg("tab_spaces=2")
      .args(chunk)
      .status()
      .context("run rustfmt")?;
    if !status.success() {
      anyhow::bail!("rustfmt failed");
    }
  }
  Ok(())
}

fn collect_rs(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
  if !dir.exists() {
    return Ok(());
  }
  for entry in std::fs::read_dir(dir)? {
    let path = entry?.path();
    if path.is_dir() {
      collect_rs(&path, files)?;
    } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
      files.push(path);
    }
  }
  Ok(())
}

use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::bail;
use colored::Colorize;
use log::debug;
use tera::{Context, Tera};
use typed_builder::TypedBuilder;

use crate::TokenWrap;

/// fantasy config
#[derive(Debug, Clone, TypedBuilder)]
pub struct Config {
  /// rtdlib project root path
  path_rtd: PathBuf,
  /// telegram client project root path
  path_telegram_client: PathBuf,
  /// tl schema file path
  file_tl: PathBuf,
  /// template project path
  path_template: PathBuf,
}

impl Config {
  pub fn path_rtd(&self) -> &PathBuf {
    &self.path_rtd
  }
  pub fn path_telegram_client(&self) -> &PathBuf {
    &self.path_telegram_client
  }
  pub fn file_tl(&self) -> &PathBuf {
    &self.file_tl
  }
  pub fn path_template(&self) -> &PathBuf {
    &self.path_template
  }
}

/// cycle
#[derive(Debug, TypedBuilder)]
pub struct Cycle {
  config: Config,
  /// tera template engine
  renderer: Renderer,
  /// token wrap
  tknwrap: TokenWrap,
}

impl Cycle {
  pub fn config(&self) -> &Config {
    &self.config
  }
  pub fn tknwrap(&self) -> &TokenWrap {
    &self.tknwrap
  }
  pub fn renderer(&self) -> &Renderer {
    &self.renderer
  }
}

#[derive(Debug, TypedBuilder)]
pub struct Renderer {
  tera: Tera,
}

impl Renderer {
  pub fn render<S: AsRef<str>, P: AsRef<Path>>(
    &self,
    tpl_file: S,
    write_to: P,
    context: &mut Context,
  ) -> anyhow::Result<()> {
    let write_to = write_to.as_ref();
    let tpl_file = tpl_file.as_ref();

    let mut first_write = false;
    if !write_to.exists() {
      let Some(write_dir) = write_to.parent() else {
        bail!("Cant not get write dir");
      };
      if !write_dir.exists() {
        std::fs::create_dir_all(write_dir)?;
      }
      first_write = true;
    }
    context.insert("first_write", &first_write);

    match self.tera.render(tpl_file, context) {
      Ok(body) => {
        debug!(
          "USE TEMPLATE [{}] WRITE TO [{}]",
          tpl_file.blue(),
          write_to.to_str().map_or("", |v| v).blue()
        );
        let mut file = std::fs::OpenOptions::new()
          .create(true)
          .append(true)
          .open(write_to)?;
        file.write_all(body.as_bytes())?;
        Ok(())
      }
      Err(e) => bail!("Tera template fail: {:?}", e),
    }
  }
}

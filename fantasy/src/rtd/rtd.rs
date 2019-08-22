use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use case::CaseExt;
use colored::Colorize;
use tera::Context;

use tl_parser::types::TLTokenGroup;

use crate::Cycle;
use crate::tokenwrap::TokenWrap;

pub struct RTD<'a> {
  cycle: &'a Cycle,
}

impl<'a> RTD<'a> {
  pub fn new(cycle: &'a Cycle) -> Self {
    Self { cycle }
  }


  pub fn generate(&self) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let path_template: PathBuf = config.path_template().join("rtdlib");

    if !path_template.is_dir() {
      return bail!("RTD template path is not dir -> {:?}", path_template);
    }

    // move root path file
    self.copy_file_to(&path_template, config.path_rtd())?;
    // generate src file
    self.gen_src(&path_template)?;
    Ok(())
  }

  /// copy template file to target path
  fn copy_file_to<P: AsRef<Path>>(&self, from_dir: P, to_dir: P) -> Result<(), failure::Error> {
    let from_dir = from_dir.as_ref();
    let to_dir = to_dir.as_ref();
    if !to_dir.exists() {
      std::fs::create_dir_all(to_dir)?;
    }
    let rtd_read_dir: Vec<Result<DirEntry, std::io::Error>> = from_dir.read_dir()?.collect();
    for entry in rtd_read_dir {
      let entry_path = entry?.path();
      if !entry_path.is_file() { continue; }
      let file_name = match entry_path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => return bail!("Can not read file name")
      };
      let copy_to = to_dir.join(file_name);
      debug!("COPY {} -> {}", entry_path.to_str().map_or("", |v| v).blue(), copy_to.to_str().map_or("", |v| v).blue());
      std::fs::copy(entry_path, copy_to)?;
    }
    Ok(())
  }

  /// generate src rs file.
  fn gen_src<P: AsRef<Path>>(&self, path_template: P) -> Result<(), failure::Error> {
    let config = self.cycle.config();
    let path_template = path_template.as_ref();

    let template_src: PathBuf = path_template.join("src");
    let rtd_src: PathBuf = config.path_rtd().join("_src");
    // copy src path rs file to target dir
    self.copy_file_to(&template_src, &rtd_src)?;

    // generate common rs
    self.gen_common(path_template)?;

    self.gen_types(path_template)?;

    Ok(())
  }

  /// generate common rs file
  fn gen_common<P: AsRef<Path>>(&self, path_template: P) -> Result<(), failure::Error> {
    let path_template = path_template.as_ref();
    Ok(())
  }

  /// generate types
  fn gen_types<P: AsRef<Path>>(&self, path_template: P) -> Result<(), failure::Error> {
    let config = self.cycle.config();

    let path_template = path_template.as_ref();
    let tknwrap = self.cycle.tknwrap();
    let rtd_src: PathBuf = config.path_rtd().join("_src");

    let mut context = Context::new();
    for td_type in tknwrap.all_types() {
      self.cycle.renderer().render("rtdlib/src/types/td_type.rs", rtd_src.join(&format!("{}.rs", td_type.to_snake())[..]), &context)?;
    }
    Ok(())
  }
}




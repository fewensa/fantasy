use std::fs;
use std::path::Path;

use failure::Error;

use crate::errors;
use crate::parser::group_parser;
use crate::parser::tl;

pub struct TLParser<P: AsRef<Path>> {
  path: P
}

impl<P: AsRef<Path>> TLParser<P> {
  pub fn new(path: P) -> Self {
    Self { path }
  }

  pub fn parse(&self) -> Result<(), Error> {
    let path = self.path.as_ref();
    if !path.exists() {
      return bail!("tl file not found -> {:?}", path);
    }

    debug!("Reading {:?}", path);
    let tlbody = fs::read_to_string(path)?;
    debug!("Read ok");

    debug!("Start parse tl schema group");

    let grammars = group_parser::parse(&tlbody)?;
    debug!("GROUPS: {:#?}", grammars);

    let tokens = tl::token_group(&grammars)?;

    debug!("Parse tl schema group finish");

    Ok(())
  }
}





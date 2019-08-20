use std::fmt::Debug;
use std::any::Any;

/// tl trait
#[derive(Debug, Clone, TypedBuilder)]
pub struct TLTrait {
  // trait name
  pub(crate) name: String,
  // trait description
  pub(crate) description: String,
}

pub trait TLGrammar: Debug {
  fn as_any(&self) -> &dyn Any;
  fn start(&self) -> i32;
  fn end(&self) -> i32;
  fn token(&self) -> TLToken;
}


#[derive(Debug, Clone)]
pub enum TLToken {
  Group,
  Paragraph,
}

impl TLGrammar for TLGroup {
  fn as_any(&self) -> &Any {
    self
  }

  fn start(&self) -> i32 {
    self.start
  }

  fn end(&self) -> i32 {
    self.end
  }

  fn token(&self) -> TLToken {
    TLToken::Group
  }
}

impl TLGrammar for TLParagraph {
  fn as_any(&self) -> &Any {
    self
  }

  fn start(&self) -> i32 {
    match *self {
      TLParagraph::Functions {start, end} => start
    }
  }

  fn end(&self) -> i32 {
    match *self {
      TLParagraph::Functions {start, end} => end
    }
  }

  fn token(&self) -> TLToken {
    TLToken::Paragraph
  }
}

/// tl schema group line
#[derive(Debug, Clone, TypedBuilder)]
pub struct TLGroupLine {
  pub(crate) line: i32,
  pub(crate) text: String,
}

/// tl schema group
#[derive(Debug, Clone, TypedBuilder)]
pub struct TLGroup {
  pub(crate) start: i32,
  pub(crate) end: i32,
  pub(crate) lines: Vec<TLGroupLine>,
}

/// tl schema paragraph
#[derive(Debug, Clone)]
pub enum TLParagraph {
  Functions {
    start: i32,
    end: i32
  }
}





#![allow(dead_code)]

use std::any::Any;
use std::fmt::Debug;

use typed_builder::TypedBuilder;

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
  fn token(&self) -> TLTextToken;
}

impl dyn TLGrammar {
  pub fn is_group(&self) -> bool {
    self.as_any().downcast_ref::<TLGroup>().is_some()
  }

  pub fn is_paragraph(&self) -> bool {
    self.as_any().downcast_ref::<TLParagraph>().is_some()
  }

  pub fn on_group<F: FnOnce(&TLGroup)>(&self, fnc: F) -> &Self {
    if let Some(t) = self.as_any().downcast_ref::<TLGroup>() {
      fnc(t);
    }
    self
  }

  pub fn on_paragraph<F: FnOnce(&TLParagraph)>(&self, fnc: F) -> &Self {
    if let Some(t) = self.as_any().downcast_ref::<TLParagraph>() {
      fnc(t);
    }
    self
  }

  pub fn to_group(&self) -> Option<TLGroup> {
    self.as_any().downcast_ref::<TLGroup>().cloned()
  }

  pub fn to_paragraph(&self) -> Option<TLParagraph> {
    self.as_any().downcast_ref::<TLParagraph>().cloned()
  }
}

#[derive(Debug, Clone)]
pub enum TLTextToken {
  Group,
  Paragraph,
}

impl TLGrammar for TLGroup {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn start(&self) -> i32 {
    self.start
  }

  fn end(&self) -> i32 {
    self.end
  }

  fn token(&self) -> TLTextToken {
    TLTextToken::Group
  }
}

impl TLGrammar for TLParagraph {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn start(&self) -> i32 {
    match *self {
      TLParagraph::Functions { start, end: _ } => start,
    }
  }

  fn end(&self) -> i32 {
    match *self {
      TLParagraph::Functions { start: _, end } => end,
    }
  }

  fn token(&self) -> TLTextToken {
    TLTextToken::Paragraph
  }
}

/// tl schema group line
#[derive(Debug, Clone, TypedBuilder)]
pub struct TLGroupLine {
  pub(crate) line: i32,
  pub(crate) token: TLGroupLineToken,
  pub(crate) text: String,
}

#[derive(Debug, Clone)]
pub enum TLGroupLineToken {
  Trait,
  Description,
  Struct,
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
  Functions { start: i32, end: i32 },
}

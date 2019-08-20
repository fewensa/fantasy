use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

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

impl TLGrammar {
  pub fn is_group(&self) -> bool {
    match self.as_any().downcast_ref::<TLGroup>() {
      Some(_) => true,
      None => false,
    }
  }

  pub fn is_paragraph(&self) -> bool {
    match self.as_any().downcast_ref::<TLParagraph>() {
      Some(_) => true,
      None => false,
    }
  }

  pub fn on_group<F: FnOnce(&TLGroup)>(&self, fnc: F) -> &Self {
    match self.as_any().downcast_ref::<TLGroup>() {
      Some(t) => fnc(t),
      None => {},
    };
    self
  }

  pub fn on_paragraph<F: FnOnce(&TLParagraph)>(&self, fnc: F) -> &Self {
    match self.as_any().downcast_ref::<TLParagraph>() {
      Some(t) => fnc(t),
      None => {},
    };
    self
  }
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
      TLParagraph::Functions { start, end } => start
    }
  }

  fn end(&self) -> i32 {
    match *self {
      TLParagraph::Functions { start, end } => end
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
  Functions {
    start: i32,
    end: i32,
  }
}

/// token argument
#[derive(Debug, Clone)]
pub struct TLTokenArgType {
  name: String,
  components: Vec<TLTokenArgType>,
}

#[derive(Debug, Clone)]
pub enum TLTokenGroupType {
  Struct,
  Trait,
  Function,
}

/// tl schema token group
#[derive(Debug, Clone)]
pub struct TLTokenGroup {
  /// all description, include trait/struct description and argument description
  description: HashMap<String, String>,
  /// trait/struct name
  name: String,
  /// trait/struct argument map
  argument: HashMap<String, TLTokenArgType>,
  /// token group type
  type_: TLTokenGroupType,
  /// when type is struct, blood is super type
  ///      type is trait, blood is none
  ///      type is function, blood is return type
  blood: Option<String>,
}






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
  fn token(&self) -> TLTextToken;
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

  pub fn to_group(&self) -> Option<TLGroup> {
    self.as_any().downcast_ref::<TLGroup>().map(|v| v.clone())
  }

  pub fn to_paragraph(&self) -> Option<TLParagraph> {
    self.as_any().downcast_ref::<TLParagraph>().map(|v| v.clone())
  }
}


#[derive(Debug, Clone)]
pub enum TLTextToken {
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

  fn token(&self) -> TLTextToken {
    TLTextToken::Group
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
  Functions {
    start: i32,
    end: i32,
  }
}

/// token argument
#[derive(Debug, Clone, TypedBuilder)]
pub struct TLTokenArgType {
  pub(crate) sign_name: String,
  pub(crate) sign_type: String,
  #[builder(default)]
  pub(crate) description: Option<String>,
  pub(crate) components: Vec<TLTokenComponentType>,
}

/// token component type
#[derive(Debug, Clone, TypedBuilder)]
pub struct TLTokenComponentType {
  sign_type: String,
  components: Vec<TLTokenComponentType>,
}

#[derive(Debug, Clone)]
pub enum TLTokenGroupType {
  Struct,
  Trait,
  Function,
}

/// tl schema token group
#[derive(Debug, Clone, TypedBuilder)]
pub struct TLTokenGroup {
  /// all description, include trait/struct description and argument description
  pub(crate) description_all: Option<HashMap<String, String>>,
  /// this group description
  pub(crate) description: Option<String>,
  /// trait/struct name
  pub(crate) name: String,
  /// trait/struct argument map
  pub(crate) arguments: Vec<TLTokenArgType>,
  /// token group type
  pub(crate) type_: TLTokenGroupType,
  /// when type is struct, blood is super type
  ///      type is trait, blood is none
  ///      type is function, blood is return type
  pub(crate) blood: Option<String>,
}

impl TLTokenGroup {
  pub fn description_all (&self) -> Option<HashMap<String, String>>{ self.description_all.clone() }
  pub fn description     (&self) -> Option<String>                 { self.description    .clone() }
  pub fn name            (&self) -> String                         { self.name           .clone() }
  pub fn arguments       (&self) -> Vec<TLTokenArgType>            { self.arguments      .clone() }
  pub fn type_           (&self) -> TLTokenGroupType               { self.type_          .clone() }
  pub fn blood           (&self) -> Option<String>                 { self.blood          .clone() }
}

impl TLTokenArgType {
  pub fn sign_name   (&self) -> String                     { self.sign_name  .clone() }
  pub fn sign_type   (&self) -> String                     { self.sign_type  .clone() }
  pub fn description (&self) -> Option<String>             { self.description.clone() }
  pub fn components  (&self) -> Vec<TLTokenComponentType>  { self.components .clone() }
}

impl TLTokenComponentType {
  pub fn sign_type   (&self) -> String                     { self.sign_type  .clone() }
  pub fn components  (&self) -> Vec<TLTokenComponentType>  { self.components .clone() }
}






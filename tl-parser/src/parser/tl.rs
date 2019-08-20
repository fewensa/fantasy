use failure::Error;
use text_reader::TextReader;
use crate::types::*;

pub fn token_group(grammars: &Vec<Box<TLGrammar>>) -> Result<Vec<TLTokenGroup>, Error> {
  let tokens = vec![];

  debug!("Start parse token group");
  for grammar in grammars {
    grammar.on_group(|group: &TLGroup| {

    });
    grammar.on_paragraph(|paragraph: &TLParagraph| {
      debug!("PARAGRAPH: {:?}", paragraph);
    });
  }

  debug!("Parse token group finish");
  Ok(tokens)
}

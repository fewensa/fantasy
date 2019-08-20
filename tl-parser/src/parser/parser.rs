use std::fs;
use std::path::Path;

use failure::Error;
use rstring_builder::StringBuilder;
use text_reader::TextReader;

use crate::errors;
use crate::tl;
use crate::types::*;

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

    let grammars = parse_group_use_text_reader(&tlbody)?;

    debug!("GROUPS: {:#?}", grammars);
    debug!("Parse tl schema group finish");

    Ok(())
  }
}


fn parse_group_use_text_reader<S: AsRef<str>>(schema: S) -> Result<Vec<Box<TLGrammar>>, Error> {
  let mut reader = TextReader::new(schema.as_ref());

  let mut grammars: Vec<Box<TLGrammar>> = vec![];
  let mut group_lines: Vec<TLGroupLine> = vec![];

  while reader.has_next() {
    match reader.next() {
      Some('/') => {
        if reader.cursor() != 1 {
          continue;
        }
        let mut detector = reader.detector();
        if detector.next_text("/@class ").no() {
          detector.rollback();
          continue;
        }
        // is class(trait) line, add group
        let group = TLGroup::builder()
          .start(reader.line() as i32)
          .end(reader.line() as i32)
          .lines(vec![
            TLGroupLine::builder()
              .line(reader.line() as i32)
              .text(reader.this_line().expect("Impossible error"))
              .build()
          ])
          .build();
        grammars.push(Box::new(group));


        // skip to next line
        while reader.has_next() {
          // skip this line
          if reader.next() != Some('\n') {
            continue;
          }
          // the next not first char is '\n' line
          while reader.has_next() {
            if reader.next() != Some('\n') {
              reader.back();
              break;
            }
          }
          break;
        }
      }
      Some(';') => {
        let line = reader.this_line();
        if line.is_none() { return bail!("tl schema fail line -> [{}] , line_text -> {:?}", reader.line(), reader.this_line()); }
        let line_text = line.unwrap();
        let mut line = line_text.chars();
        // if current char is ; and this line first char is not / , group end
        if line.next() != Some('/') {
          group_lines.push(TLGroupLine::builder()
            .line(reader.line() as i32)
            .text(reader.this_line().expect("Impossible error"))
            .build());
          let gpsfirst = group_lines.get(0);
          if gpsfirst.is_none() {
            return bail!("tl schema fail line -> [{}] line_text -> {}", reader.line(), line_text);
          }
          let gpsfirst = gpsfirst.unwrap();

          let group = TLGroup::builder()
            .start(gpsfirst.line)
            .end(reader.line() as i32)
            .lines(group_lines.clone())
            .build();
          grammars.push(Box::new(group));
          group_lines.clear();

          // skip multi \n flag
          while reader.has_next() {
            if reader.next() != Some('\n') {
              reader.back();
              break;
            }
          }
          continue;
        }
      }
      Some('-') => {
        if reader.cursor() != 1 {
          continue;
        }
        let line = reader.this_line().expect("Impossible error");
        if line.starts_with("---") && line.ends_with("---") {
          let line = line.replace("---", "");
          let line_number = reader.line() as i32;
          match &line.to_lowercase()[..] {
            "functions" => grammars.push(Box::new(TLParagraph::Functions { start: line_number, end: line_number })),
            _ => return bail!("Unsupport paragraph token -> {} line -> [{}] line_text -> {}", line, reader.line(), line)
          }

          // skip to next line
          while reader.has_next() {
            // skip this line
            if reader.next() != Some('\n') {
              continue;
            }
            // the next not first char is '\n' line
            while reader.has_next() {
              if reader.next() != Some('\n') {
                reader.back();
                break;
              }
            }
            break;
          }
          continue;
        }
        // faild token
        return bail!("tl schema fail line -> [{}] line_text -> {}", reader.line(), line);
      }
      Some('\n') => {
        // back a char
        reader.back();
        let line = reader.line();
        let line_text = reader.this_line();

        // skip multi \n flag
        while reader.has_next() {
          if reader.next() != Some('\n') {
            reader.back();
            break;
          }
        }
        group_lines.push(TLGroupLine::builder()
          .line(line as i32)
          .text(line_text.expect("Impossible error"))
          .build());
      }
      Some(ch) => {}
      None => {}
    }
  }

  Ok(grammars)
}




use std::collections::HashMap;

use case::CaseExt;
use serde::ser::Error;
use serde_json::Value;
use tera::Tera;

pub fn filter(tera: &mut Tera) -> Result<(), failure::Error> {
  self::add_filter_case(tera)?;
  Ok(())
}

pub fn add_filter_case(tera: &mut Tera) -> Result<(), failure::Error> {
  // snake
  fn case_to_snake_filter(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_snake()).unwrap()),
      None => Err(tera::Error::from(serde_json::Error::custom(format!("Error value {:?}", value))))
    }
  }
  fn case_to_camel(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_camel()).unwrap()),
      None => Err(tera::Error::from(serde_json::Error::custom(format!("Error value {:?}", value))))
    }
  }
  fn case_to_camel_lowercase(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_camel_lowercase()).unwrap()),
      None => Err(tera::Error::from(serde_json::Error::custom(format!("Error value {:?}", value))))
    }
  }
  fn case_to_dashed(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_dashed()).unwrap()),
      None => Err(tera::Error::from(serde_json::Error::custom(format!("Error value {:?}", value))))
    }
  }
  fn case_to_capitalized(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_capitalized()).unwrap()),
      None => Err(tera::Error::from(serde_json::Error::custom(format!("Error value {:?}", value))))
    }
  }
  tera.register_filter("to_snake", case_to_snake_filter);
  tera.register_filter("to_camel", case_to_camel);
  tera.register_filter("to_camel_lowercase", case_to_camel_lowercase);
  tera.register_filter("to_dashed", case_to_dashed);
  tera.register_filter("to_capitalized", case_to_capitalized);
  Ok(())
}

use std::collections::HashMap;

use case::CaseExt;
use rstring_builder::StringBuilder;
use serde::ser::Error;
use serde_json::Value;
use tera::Tera;

use tl_parser::types::{TLTokenArgType, TLTokenComponentType, TLTokenGroup};

use crate::tdfill::TDTypeFill;

pub fn fill(tera: &mut Tera, tdtypefill: TDTypeFill) -> Result<(), failure::Error> {
  self::add_filter_case(tera)?;
  self::add_filter_safe(tera)?;
  self::add_td_fnc(tera, tdtypefill.clone())?;
  Ok(())
}

fn add_filter_case(tera: &mut Tera) -> Result<(), failure::Error> {
  // snake
  fn case_to_snake_filter(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_snake()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_camel(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_camel()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_camel_lowercase(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_camel_lowercase()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_dashed(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_dashed()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  fn case_to_capitalized(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => Ok(serde_json::value::to_value(text.to_capitalized()).unwrap()),
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  tera.register_filter("to_snake", case_to_snake_filter);
  tera.register_filter("to_camel", case_to_camel);
  tera.register_filter("to_camel_lowercase", case_to_camel_lowercase);
  tera.register_filter("to_dashed", case_to_dashed);
  tera.register_filter("to_capitalized", case_to_capitalized);
  Ok(())
}

fn add_filter_safe(tera: &mut Tera) -> Result<(), failure::Error> {
  fn safe_field(value: Value, arg: HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
      Some(text) => match text {
        "type" => Ok(serde_json::value::to_value("type_".to_string()).unwrap()),
        _ => Ok(serde_json::value::to_value(text.to_snake()).unwrap())
      },
      None => Err(format!("Error value {:?}", value).into())
    }
  }
  tera.register_filter("safe_field", safe_field);
  Ok(())
}

fn add_td_fnc(tera: &mut Tera, tdtypefill: TDTypeFill) -> Result<(), failure::Error> {

  // argument type
  let td_arg = Box::new(move |argument: HashMap<String, Value>| -> tera::Result<Value> {
    let token: TLTokenGroup = match argument.get("token") {
      Some(t) => match serde_json::from_value(t.clone()) {
        Ok(a) => a,
        Err(e) => return Err("Can't covert token to TLTokenGroup".into())
      },
      None => return Err("Can't found token".into())
    };
    let arg: TLTokenArgType = match argument.get("arg") {
      Some(t) => match serde_json::from_value(t.clone()) {
        Ok(a) => a,
        Err(e) => return Err("Can't covert arg to TLTokenArgType".into())
      },
      None => return Err("Can't found arg".into())
    };
    let mut arg_type = tdtypefill.mapper(arg.sign_type()).map_or(arg.sign_type().to_camel(), |v| v);
    let components = arg.components();
    if !components.is_empty() {
      let component_type = self::fill_type_components(components, &tdtypefill);
      arg_type = format!("{}{}", arg_type, component_type);
    }

    arg_type = tdtypefill.td_filter_type(token.name(), arg.sign_name(), arg_type);

    Ok(serde_json::value::to_value(arg_type).unwrap())
  });


  tera.register_function("td_arg", td_arg);
  Ok(())
}


fn fill_type_components(components: Vec<TLTokenComponentType>, tdtypefill: &TDTypeFill) -> String {
  let mut rets = vec![];
  for component in components {
    let mut sign_type = component.sign_type();
    let sub_components = component.components();
    if !sub_components.is_empty() {
      let component_type = fill_type_components(sub_components, tdtypefill);
      sign_type = format!("{}{}", sign_type, component_type);
    }
    rets.push(format!("<{}>", tdtypefill.mapper(&sign_type).map_or(sign_type.to_camel(), |v| v)));
  }
  rets.join(", ")
}


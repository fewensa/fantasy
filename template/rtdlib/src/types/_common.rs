
use std::fmt::Debug;
use std::str::FromStr;
use crate::tdkit;


/// All tdlib type abstract class defined the same behavior
pub trait RObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str;
  /// convert TDLib type to rust enum RTDType
  fn td_type(&self) -> RTDType;
  /// The string that implements the return of to_json should be called `tdkit::fill_json_struct` for optimization,
  /// appending the `@struct` field, although usually struct will actively generate `@struct`, but not in `Object` and `Function`,
  /// because the implementation of typetag cannot be automatically generated.
  fn to_json(&self) -> String;
}


/// TDLib all class name mappers
#[derive(Debug, Clone)]
pub enum RTDType {
  {% for item in common.clzs %}{{ item.name }},
{% endfor %}
}

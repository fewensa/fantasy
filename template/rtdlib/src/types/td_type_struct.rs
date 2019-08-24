{% set struct_name = token.name | to_camel %}
/// {{token.description}}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct {{struct_name}} {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  {% for field in token.arguments %}/// {{field.description}}
  {{field.sign_name | td_safe_field}}: {{td_arg(arg=field, token=token)}},
  {% endfor %}
}

impl RObject for {{struct_name}} {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "{{token.name}}" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}
{% if token.blood and token.blood | to_snake != token.name | to_snake %}
{% set blood_token = find_token(token_name=token.blood) %}
{% if blood_token.type_ == 'Trait' %}impl TD{{token.blood | to_camel}} for {{struct_name}} {}{% endif %}
{% endif %}
{% if token.type_ == 'Function' %}impl RFunction for {{struct_name}} {}{% endif %}

impl {{struct_name}} {
  pub fn builder() -> RTD{{struct_name}}Builder {
    RTD{{struct_name}}Builder { inner: {{struct_name}}::default() }
  }
{% for field in token.arguments %}{% set field_type = td_arg(arg=field, token=token) %}{% set is_primitive = is_primitive(type_ = field_type) %}
  pub fn {{field.sign_name | td_safe_field}}(&self) -> {% if not is_primitive %}&{% endif %}{{field_type}} { {% if not is_primitive %}&{% endif %}self.{{field.sign_name | td_safe_field}} }
{% endfor %}
}

#[doc(hidden)]
pub struct RTD{{struct_name}}Builder {
  inner: {{struct_name}}
}

impl RTD{{struct_name}}Builder {
  pub fn build(self) -> {{struct_name}} { self.inner }
{% for field in token.arguments %}{%set field_type=td_arg(arg=field, token=token)%}{% set is_optional = is_optional(type_=field_type) %}
  pub fn {{field.sign_name | td_safe_field}}(&mut self, t: {{td_arg(arg=field, token=token, builder_arg=true)}}) -> &mut Self {
    self.inner.{{field.sign_name | td_safe_field}} = {% if is_optional %}Some(t){% else %}t{% endif %};
    self
  }
{% endfor %}
}

impl AsRef<{{struct_name}}> for {{struct_name}} {
  fn as_ref(&self) -> &{{struct_name}} { self }
}

impl AsRef<{{struct_name}}> for RTD{{struct_name}}Builder {
  fn as_ref(&self) -> &{{struct_name}} { &self.inner }
}


{% if first_write %}
use std::fmt::Debug;

use crate::types::*;
{% endif %}

{% if token.type_ == "Trait" %}

// {{token.description}}
++ {{token.type_}}

{% else %}

/// {{token.description}}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{token.name | to_camel}} {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
{% for field in token.arguments %}  /// {{field.description}}
  {{field.sign_name | safe_field}}: {{td_arg(arg=field, token=token)}},
{% endfor %}
}

{% endif %}

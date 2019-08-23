
{% if first_write %}
use std::fmt::Debug;

use crate::types::*;
{% endif %}

{% if token.type_ == "Trait" %}

++ {{token.type_}}

{% else %}

pub struct {{token.name | to_camel}} {
{% for field in token.arguments %}  {{field.sign_name | safe_field}}: {{field.sign_type}},
{% endfor %}
}

{% endif %}

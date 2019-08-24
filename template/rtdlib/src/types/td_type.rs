{% if first_write %}
use crate::types::*;
use crate::errors::*;
{% endif %}

{% if token.type_ == "Trait" %}
{% if first_write %}
use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};
{% endif %}

{% include "rtdlib/src/types/td_type_trait.rs" %}
{% else %}
{% include "rtdlib/src/types/td_type_struct.rs" %}
{% endif %}

// {% include 'doc.template' %}
// {% do require('structures_irreducibles') %}

use crate::core::*;

// ---------- Irreducible Structures ----------

/// Represents an irreducible structure.
pub trait Irreducible: Structure {
    /// Returns the "inner" value of the structure.
    fn inner_value(&self) -> InnerValue {
        InnerValue::None
    }
}

/// Represents the "Inner" (Rust) value of an irreducible structure.
pub enum InnerValue {
    /// Represents a boolean inner value.
    Bool(bool),
    
    /// Represents a 64-bit floating-point inner value.
    F64(f64),

    /// Represents a 128-bit integer inner value.
    I128(i128),

    /// Represents a string inner value.
    Str(&'static str),

    /// Represents no inner value.
    None
}


// ---------- Implementations ----------

// {% for structure in structures_irreducibles %}

// ----- {{ structure.name }} -----

/// {{ structure.desc }}
pub struct {{ structure.name }}(pub {{ structure.inner_value_type }});

/// Method implementations for `{{ structure.name }}`
impl {{ structure.name }} {
    /// Creates a new instance of the structure, returning an expression bound to it.
    pub fn new(val: {{ structure.inner_value_type }}) -> Expression {
        Expression(std::rc::Rc::new({{ structure.name }}(val)))
    }
}

/// `Structure` implementation for `{{ structure.name }}`
impl Structure for {{ structure.name }} {
    /// Returns the name of this `Structure`.
    fn name(&self) -> Option<String> {
        Some(String::from("{{ structure.name }}"))
    }

    /// Returns the string representation of this `Structure`.
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

/// `Irreducible` implementation for `{{ structure.name }}`
impl Irreducible for {{ structure.name }} {
    /// Returns the "inner" value of the structure.
    fn inner_value(&self) -> InnerValue {
        {{ structure.inner_value_name }}(self.0)
    }
}

// {% endfor %}

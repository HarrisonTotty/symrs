// {% include 'doc.template' %}

use crate::core::structure::*;
use crate::structures::operations::elementary::*;
use crate::structures::irreducibles::*;


// ------------ Expressions ------------

/// Represents a mathematical expression referencing an internal structure.
///
/// # Remarks
/// Expressions in `symrs` are smart pointers to a defined `Structure`.
#[derive(Clone)]
pub struct Expression(pub std::rc::Rc<Structure>);

impl Expression {
    /// Creates a new `Expression` with the given underlying structure.
    pub fn new(structure: std::rc::Rc<Structure>) -> Self {
        Expression(structure)
    }
}

/// Implements `std::ops::Deref` on `Expression`.
impl std::ops::Deref for Expression {
    type Target = std::rc::Rc<Structure>;
    fn deref(&self) -> &std::rc::Rc<Structure> {
        &self.0
    }
}

// {# ----- Operator Implementations ----- #}

// {% for op in ['Add', 'Div', 'Mul', 'Sub'] %}
// {% for other in ['Expression', 'i128', 'f64'] %}

// {% if op == 'Add' %}
// {% set new_builder = 'Addition::new' %}
// {% elif op == 'Div' %}
// {% set new_builder = 'Division::new' %}
// {% elif op == 'Mul' %}
// {% set new_builder = 'Multiplication::new' %}
// {% elif op == 'Sub' %}
// {% set new_builder = 'Subtraction::new' %}
// {% endif %}

// {% if other == 'i128' %}
// {% set other_builder = 'Integer::new(other)' %}
// {% elif other == 'f64' %}
// {% set other_builder = 'Real::new(other)' %}
// {% else %}
// {% set other_builder = 'other' %}
// {% endif %}

/// Implements `std::ops::{{ op }}` between `Expression` and `{{ other }}`.
impl std::ops::{{ op }}<{{ other }}> for Expression {
    type Output = Expression;
    fn {{ op|lower }}(self, other: {{ other }}) -> Expression {
        {{ new_builder }}(self, {{ other_builder }})
    }
}

// {% endfor %}
// {% endfor %}

// -------------------------------------

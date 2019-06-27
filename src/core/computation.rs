// {% include 'doc.template' %}

use crate::core::expression::*;

/// The result of the application of a computation on an expression.
pub type ApplicationResult = Result<Option<Expression>, String>;

/// Represents a `symrs` computation.
///
/// Computations apply themselves to expressions. 
pub trait Computation {
    /// Applies the computation to an expression, effectively returning if any
    /// invalid operation was performed and whether or not a new expression was
    /// generated.
    fn apply(&self, _expression: Expression) -> ApplicationResult {
        Ok(None)
    }
}

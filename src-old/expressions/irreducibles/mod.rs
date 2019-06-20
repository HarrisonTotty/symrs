//! Contains the core irreducible expressions.

use std::rc::Rc;
use crate::core::*;

/// Represents a logical truth value.
pub struct Logical(pub bool);
    
/// Represents an integer value.
pub struct Integer(pub i128);

/// Represents a real value.
pub struct Real(pub f64);

/// Represents a mathematical symbol with a particular name.
pub struct Symbol(pub &'static str);

/// Implements custom functions for `Logical`.
impl Logical {
    /// Creates a new instance of `Logical` with the specified internal value.
    pub fn new(val: bool) -> ExprRef {
        ExprRef(Rc::new(Logical(val)))
    }
}

/// Implements `Expression` for `Logical`.
impl Expression for Logical {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    /// Returns the contained inner value of the expression.
    fn inner_value(&self) -> InnerValue {
        InnerValue::Bool(self.0)
    }
}

/// Implements `Expression` for `Integer`.
impl Expression for Integer {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    /// Returns the contained inner value of the expression.
    fn inner_value(&self) -> InnerValue {
        InnerValue::Integer(self.0)
    }
}

/// Implements custom functions for `Integer`.
impl Integer {
    /// Creates a new instance of `Logical` with the specified internal value.
    pub fn new(val: i128) -> ExprRef {
        ExprRef(Rc::new(Integer(val)))
    }
}

/// Implements `Expression` for `Real`.
impl Expression for Real {
    fn to_string(&self) -> String {
        format!("{}", self.0)
    }
    
    /// Returns the contained inner value of the expression.
    fn inner_value(&self) -> InnerValue {
        InnerValue::Float(self.0)
    }
}

/// Implements custom functions for `Real`.
impl Real {
    /// Creates a new instance of `Real` with the specified internal value.
    pub fn new(val: f64) -> ExprRef {
        ExprRef(Rc::new(Real(val)))
    }
}

/// Implements `Expression` for `Symbol`.
impl Expression for Symbol {
    fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Returns the contained inner value of the expression.
    fn inner_value(&self) -> InnerValue {
        InnerValue::String(self.0)
    }
}

/// Implements custom functions for `Symbol`.
impl Symbol {
    /// Creates a new instance of `Real` with the specified internal value.
    pub fn new(val: &'static str) -> ExprRef {
        ExprRef(Rc::new(Symbol(val)))
    }
}

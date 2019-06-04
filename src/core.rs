//! Contains the core `Expression` trait definition.

use std::rc::Rc;
use crate::errors::*;
use crate::expressions::irreducibles::*;
use crate::expressions::operations::elementary::*;

pub type ApplicationResult = Result<Option<ExprRef>, ApplicationError>;
pub type Args = Option<Vec<ExprRef>>;
pub type Attributes = Option<Vec<String>>;
pub type KWArgs = Option<std::collections::HashMap<String, ExprRef>>;

/// Represents the common definition pertaining to symbolic expressions.
pub trait Expression {
    
    /// "Applies" this expression with its children as arguments, returning
    /// an enumerable describing whether the application had issues and whether
    /// there was any value returned.
    fn apply(&self) -> ApplicationResult {
        Ok(None)
    }

    /// Returns a collection of "arguments" or "subexpressions" available to
    /// this expression.
    fn args(&self) -> Args {
        None
    }

    /// Returns a collection of attributes available to this expression.
    fn attributes(&self) -> Attributes {
        None
    }

    /// Returns the collection of "arguments" flattened such that expressions
    /// of the same kind give up their arguments.
    fn flat_args(&self) -> Args {
        let mykind = self.kind();
        match self.args() {
            Some(args) => {
                let mut fargs: Vec<ExprRef> = Vec::new();
                for arg in args {
                    if arg.kind() == mykind {
                        if let Some(subfargs) = arg.flat_args() {
                            fargs.extend(subfargs);
                        } else {
                            fargs.push(arg);
                        }
                    } else {
                        fargs.push(arg);
                    }
                }
                Some(fargs)
            },
            None => None
        }
    }

    /// Returns the "Inner" value of an expression.
    /// Typically only irreducibles contain inner values.
    fn inner_value(&self) -> InnerValue {
        InnerValue::None
    }

    /// Returns a collection of "keyword arguments" available to this
    /// expression.
    fn kwargs(&self) -> KWArgs {
        None
    }
    
    /// Returns the type of the expression.
    fn kind(&self) -> &'static str {
        unsafe { std::intrinsics::type_name::<Self>() }
    }

    /// Returns the namespace of the expression.
    fn namespace(&self) -> &'static str {
        match self.kind().rsplitn(2, "::").collect::<Vec<&str>>().last() {
            Some(x) => x,
            _ => ""
        }
    }

    /// Returns the shortend type of the expression.
    fn skind(&self) -> &'static str {
        match self.kind().split("::").collect::<Vec<&str>>().last() {
            Some(x) => x,
            _ => ""
        }
    }

    /// Returns the string representation of the expression.
    fn to_string(&self) -> String {
        let name = match self.skind() {
            "" => self.kind(),
            x  => x
        };
        match (self.args(), self.kwargs()) {
            (Some(args), Some(_kwargs)) => {
                let args_str: String   = args.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let kwargs_str: String = String::from("");
                format!("{}{{{}, {}}}", name, args_str, kwargs_str)
            },
            (Some(args), None) => {
                let args_str: String = args.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}{{{}}}", name, args_str)
            },
            (None, Some(_kwargs)) => {
                String::from("")
            },
            (None, None) => String::from(name)
        }
    }
}


/// Represents the "Inner" (Rust) value of an expression.
/// This only applies to irreducible expressions.
pub enum InnerValue {
    /// Represents a boolean inner value.
    Bool(bool),
    /// Represents a floating-point inner value.
    Float(f64),
    /// Represents an integer inner value.
    Integer(i128),
    /// Represents a string inner value.
    String(&'static str),
    /// Represents no inner value.
    None
}


// ---------- Expression References ----------


/// Represents a reference to an expression.
#[derive(Clone)]
pub struct ExprRef(pub std::rc::Rc<Expression>);

/// Defines custom methods on an `ExprRef`.
impl ExprRef {
    /// Creates a new `ExprRef` from a given `std::rc::Rc<Expression>`
    pub fn new(inner: std::rc::Rc<Expression>) -> ExprRef {
        ExprRef(inner)
    }
}

/// Implements `std::ops::Add` on `ExprRef`.
impl std::ops::Add<ExprRef> for ExprRef {
    type Output = ExprRef;
    fn add(self, other:ExprRef) -> ExprRef {
        Addition::new(self, other)
    }
}

/// Implements `std::ops::Add` between `ExprRef` and numeric types.
impl std::ops::Add<i128> for ExprRef {
    type Output = ExprRef;
    fn add(self, other:i128) -> ExprRef {
        Addition::new(self, Integer::new(other))
    }
}

/// Implements `std::ops::Add` between `ExprRef` and string types.
impl std::ops::Add<&'static str> for ExprRef {
    type Output = ExprRef;
    fn add(self, other:&'static str) -> ExprRef {
        Addition::new(self, Symbol::new(other))
    }
}

/// Implements `std::ops::Mul` between `ExprRef` and numeric types.
impl std::ops::Mul<i128> for ExprRef {
    type Output = ExprRef;
    fn mul(self, other:i128) -> ExprRef {
        Multiplication::new(self, Integer::new(other))
    }
}

/// Implements `std::ops::Mul` between `ExprRef` and string types.
impl std::ops::Mul<&'static str> for ExprRef {
    type Output = ExprRef;
    fn mul(self, other:&'static str) -> ExprRef {
        Multiplication::new(self, Symbol::new(other))
    }
}

/// Implements `std::ops::Deref` on `ExprRef`.
impl std::ops::Deref for ExprRef {
    type Target = std::rc::Rc<Expression>;
    fn deref(&self) -> &std::rc::Rc<Expression> {
        &self.0
    }
}

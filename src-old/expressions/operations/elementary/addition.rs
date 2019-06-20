//! Contains the definitions of the `Addition` operation.

use crate::core::*;
use crate::errors::*;
use crate::expressions::irreducibles::*;
use std::rc::Rc;

/// Defines the structure of the `Addition` operation.
pub struct Addition(pub ExprRef, pub ExprRef);

/// Defines some functions unique to addition.
impl Addition {
    /// Creates a new addition object from two expressions.
    pub fn new(addend: ExprRef, augend: ExprRef) -> ExprRef {
        ExprRef::new(Rc::new(Addition(addend, augend)))
    }
}

/// Implements `Expression` for `Addition`.
impl crate::core::Expression for Addition {
    fn apply(&self) -> ApplicationResult {
        if let Some(flat_args) = self.flat_args() {
            let mut isum: i128                  = 0i128;
            let mut icount: u64                 = 0u64;
            let mut fsum: f64                   = 0f64;
            let mut fcount: u64                 = 0u64;
            let mut others: Vec<ExprRef> = Vec::new();
            for arg in flat_args {
                match arg.kind() {
                    "expressions::irreducibles::Integer" => {
                        if let InnerValue::Integer(x) = arg.inner_value() {
                            isum += x;
                            icount += 1;
                        }
                    },
                    "expressions::irreducibles::Real" => {
                        if let InnerValue::Float(x) = arg.inner_value() {
                            fsum += x;
                            fcount += 1;
                        }
                    },
                    _ => {
                        others.push(arg);
                    }
                };
            }
            // If we were only working with numbers:
            if others.is_empty() {
                // If we added together more than one integer and have a current
                // float value of 0:
                if icount > 1 && fsum == 0.0 {
                    return Ok(Some(Integer::new(isum)));
                }
                // If we added together more than one float and have a current
                // integer value of 0:
                else if fcount > 1 && isum == 0 {
                    return Ok(Some(Real::new(fsum)));
                }
                // If we added together more than one integer or float:
                else if icount > 1 && fcount > 1 {
                    return Ok(Some(Integer::new(isum)))
                }
            }
        }
        else {
            return Err(ApplicationError::NumArgs("Invalid number of arguments for Addition Expression."));
        }
        return Ok(None);
    }
    
    fn args(&self) -> crate::core::Args {
        Some(vec![self.0.clone(), self.1.clone()])
    }
}

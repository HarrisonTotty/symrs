//! Contains the definitions of the `Multiplication` operation.

use crate::core::*;
use crate::errors::*;
use crate::expressions::irreducibles::*;
use std::rc::Rc;

/// Defines the structure of the `Multiplication` operation.
pub struct Multiplication(pub ExprRef, pub ExprRef);

/// Defines some functions unique to multiplication.
impl Multiplication {
    /// Creates a new multiplication object from two expressions.
    pub fn new(multiplicand: ExprRef, multiplier: ExprRef) -> ExprRef {
        ExprRef::new(Rc::new(Multiplication(multiplicand, multiplier)))
    }
}

/// Implements `Expression` for `Multiplication`.
impl crate::core::Expression for Multiplication {
    fn apply(&self) -> ApplicationResult {
        if let Some(flat_args) = self.flat_args() {
            let mut itot: i128                  = 1i128;
            let mut icount: u64                 = 0u64;
            let mut ftot: f64                   = 1.0f64;
            let mut fcount: u64                 = 0u64;
            let mut others: Vec<ExprRef> = Vec::new();
            for arg in flat_args {
                match arg.kind() {
                    "expressions::irreducibles::Integer" => {
                        if let InnerValue::Integer(x) = arg.inner_value() {
                            itot *= x;
                            icount += 1;
                        }
                    },
                    "expressions::irreducibles::Real" => {
                        if let InnerValue::Float(x) = arg.inner_value() {
                            ftot *= x;
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
                if (icount > 1 || fcount > 1) && (itot == 0 || ftot == 0.0) {
                    return Ok(Some(Integer::new(0)));
                }
                else if icount > 1 && fcount <= 1 {
                    return Ok(Some(Integer::new(itot)));
                }
                else if icount <= 1 && fcount > 1 {
                    return Ok(Some(Real::new(ftot)));
                }
                // If we multiplied together more than one integer or float:
                else if icount > 1 && fcount > 1 {
                    return Ok(Some(Real::new(itot as f64 * ftot)))
                }
            }
        }
        else {
            return Err(ApplicationError::NumArgs("Invalid number of arguments for Multiplication Expression."));
        }
        return Ok(None);
        
    }
    
    fn args(&self) -> crate::core::Args {
        Some(vec![self.0.clone(), self.1.clone()])
    }
}

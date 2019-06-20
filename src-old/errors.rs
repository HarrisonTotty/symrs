//! Contains various error enums.

/// Represents an error that arose during the "application" of an expression.
pub enum ApplicationError {
    /// The expression in question lacked the appropriate number of arguments to proceed.
    NumArgs(&'static str)
}

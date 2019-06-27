// {% include 'doc.template' %}

use crate::core::expression::*;

/// Represents an attribute which a mathematical structure may exhibit.
pub enum Attribute {
    /// Represents the anticommutative property of a binary operation.
    Anticommutative,
    
    /// Represents the associative property of a binary operation.
    Associative,

    /// Represents the commutative property of a binary operation.
    Commutative,

    /// Represents the distributive property of a binary operation over another operation by name.
    Distributive(String),

    /// Represents the idempotent property of a unary operation.
    Idempotent,

    /// Represents the identity property of a binary opertation over a specified expression.
    Identity(Expression),

    /// Represents that a structure is "intrinsicly" irreducible.
    Irreducible,

    /// Represents some other attribute with the specified name.
    Other(String)
}

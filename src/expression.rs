//! Borrowed expressions with explicit mathematical input and argument references.

use crate::{Attribute, Binary, Type, Unary, Value};

/// One expression in a finite acyclic definition.
///
/// `Location` belongs to the firmware or nucleus using the definition. Shared
/// references retain the graph's structure without assigning identities or
/// copying attribute state. A reused expression does not imply a cached hardware
/// observation: authored control and read order remain part of its meaning.
#[derive(Debug)]
pub enum Expr<'e, 'a, Location> {
    /// A well-formed Boolean or fixed-width integer literal.
    Constant(Value),
    /// Observe the named attribute when this expression is reached.
    Read(&'a Attribute<Location>),
    /// Invocation argument at this position in the definition's signature.
    Argument(usize),
    /// Symbolic input at this position in the shared input environment.
    Input(usize),
    /// Boolean complement.
    Not(&'e Self),
    /// Left-to-right lazy Boolean conjunction.
    And(&'e Self, &'e Self),
    /// Left-to-right lazy Boolean disjunction.
    Or(&'e Self, &'e Self),
    /// A scalar unary operation or explicit conversion.
    Unary(Unary, &'e Self),
    /// A strict left-to-right scalar operation.
    Binary(Binary, &'e Self, &'e Self),
    /// The condition followed by exactly one same-typed alternative.
    Select(&'e Self, &'e Self, &'e Self),
    /// Shared semantic definition using the same arguments and symbolic inputs.
    /// This is a mathematical reference, not a native function invocation.
    Reference(&'e Definition<'e, 'a, Location>),
}

impl<Location> Copy for Expr<'_, '_, Location> {}

impl<Location> Clone for Expr<'_, '_, Location> {
    fn clone(&self) -> Self {
        *self
    }
}

/// A mathematical expression with its declared argument and input types.
///
/// No executable code, interpreter state, or ownership allocation is stored.
/// Reusing a definition preserves its arguments and input environment.
#[derive(Debug)]
pub struct Definition<'e, 'a, Location> {
    /// Expression defining the result.
    pub body: Expr<'e, 'a, Location>,
    /// Ordered invocation argument types.
    pub arguments: &'e [Type],
    /// Ordered symbolic input types.
    pub inputs: &'e [Type],
}

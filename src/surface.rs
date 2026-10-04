//! Mathematical state and transformations connected by direct references.

use crate::{Definition, Expr, Type};

/// One mathematical state element and its consumer-supplied location.
///
/// The location identifies the actual state; it is not a copy of its value.
/// All uses of an attribute can refer to this same declaration. No ID allocation,
/// lookup table, state publication, or hardware access is performed here.
#[derive(Debug)]
pub struct Attribute<Location> {
    /// Mathematical value domain, including integer width and signedness.
    pub ty: Type,
    /// Correspondence to state owned by the consuming firmware or nucleus.
    pub location: Location,
}

/// A Boolean condition over invocation arguments and explicitly named inputs.
///
/// The definition has a Boolean result and uses `Expr::Input` for its supplied
/// state environment. Its signature must agree with these input attributes and
/// with the operation's arguments when the predicate uses arguments.
#[derive(Debug)]
pub struct Predicate<'e, 'a, Location> {
    /// Shared mathematical definition.
    pub definition: &'e Definition<'e, 'a, Location>,
    /// Attributes corresponding to the definition's symbolic input positions.
    pub inputs: &'e [&'a Attribute<Location>],
}

/// The coupled state transformation associated with one operation.
///
/// All result expressions use the same arguments and pre-transition input
/// environment. Result position `i` defines output attribute `i`; it must have
/// that attribute's type. Unlisted state is unchanged by this mathematical
/// effect. Result expressions use symbolic inputs rather than live reads.
///
/// Outputs describe the modeled effects, not a function return to a caller.
/// Actual observed changes are communicated by the vertex. This structure does
/// not publish state or execute any operation.
#[derive(Debug)]
pub struct Effect<'e, 'a, Location> {
    /// Actual attributes supplying the shared symbolic input environment.
    pub inputs: &'e [&'a Attribute<Location>],
    /// Attributes affected by this operation, in result order.
    pub outputs: &'e [&'a Attribute<Location>],
    /// One mathematical result expression for each affected attribute.
    pub results: &'e [Expr<'e, 'a, Location>],
    /// Ordered invocation argument types.
    pub arguments: &'e [Type],
}

/// A state-transforming operation defined by its precondition and effects.
///
/// Hardware callables are connected by the firmware using this definition; an
/// operation's mathematical meaning is never inferred from a Rust function.
/// The nucleus consumes the same definition for its reasoning and compilation.
#[derive(Debug)]
pub struct Operation<'e, 'a, Location> {
    /// Mathematical condition under which the invocation is permitted.
    pub precondition: Predicate<'e, 'a, Location>,
    /// Joint transformation of the operation's affected state.
    pub effect: Effect<'e, 'a, Location>,
}

/// Borrow the mathematical surface of one appliance.
///
/// A vertex can project its native handles through this interface; nucleus can
/// implement it over definitions it owns or has received. Consumers depend only
/// on the IR and the opaque location representation chosen by the implementation.
/// No native reader or state transformer is invoked by this interface.
///
/// Iterators expose the original definitions and their direct relationships. An
/// implementation need not allocate another collection, copy state, or assign IDs.
/// Distinct callable bindings retain distinct operation roots even when their
/// expression bodies are shared, so the consumer can refer to the chosen binding.
/// The view describes semantic roots, not a wire format or an installed program.
pub trait Surface {
    /// Correspondence to state in the consumer's own reference domain.
    type Location;

    /// Exposed states, retaining their original mathematical definitions.
    fn states(&self) -> impl Iterator<Item = &Attribute<Self::Location>>;

    /// Exposed transformations, including preconditions and coupled effects.
    fn transformers(&self) -> impl Iterator<Item = &Operation<'_, '_, Self::Location>>;

    /// Device-model facts supplied as assumptions of the admitted state domain.
    /// These are argument-free Boolean predicates over their declared state inputs.
    fn axioms(&self) -> impl Iterator<Item = &Predicate<'_, '_, Self::Location>>;

    /// Device-wide properties that admitted transitions must preserve.
    /// These are argument-free Boolean predicates, separate from per-invocation
    /// preconditions. Nucleus validates and proves the applicable obligations;
    /// exposing a declaration does not establish its truth or native enforcement.
    fn invariants(&self) -> impl Iterator<Item = &Predicate<'_, '_, Self::Location>>;
}

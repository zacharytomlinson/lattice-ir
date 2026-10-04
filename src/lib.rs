//! Shared mathematical definitions for Lattice appliance surfaces.
//!
//! Definitions borrow their expressions and refer directly to attributes. They
//! can live in static firmware storage without an allocator or a runtime registry.
//! Attribute locations are supplied by the consumer: a vertex can use a reference
//! to its hardware state, while the nucleus can use its corresponding location
//! representation. This crate never reads or writes those locations.
//!
//! The vocabulary comes from Jarvis's common scalar IR. Interpretation, proof,
//! optimization, compilation, installation, and transport belong to consumers.
//! Expressions describe finite acyclic computations; references do not introduce
//! recursion. The nucleus owns whole-definition validation, including reference
//! cycles and complete argument/input/output agreement.

#![no_std]
#![forbid(unsafe_code)]

mod expression;
mod scalar;
mod surface;

pub use expression::{Definition, Expr};
pub use scalar::{Binary, Error, Integer, Type, Unary, Value};
pub use surface::{Attribute, Effect, Operation, Predicate};

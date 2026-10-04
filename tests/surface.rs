//! Nucleus-style consumption of borrowed definitions using only lattice-ir.

use lattice_ir::{Attribute, Definition, Effect, Expr, Operation, Predicate, Surface, Type, Value};

struct BorrowedSurface<'a> {
    state: &'a Attribute<&'a bool>,
    transformer: &'a Operation<'a, 'a, &'a bool>,
    axiom: &'a Predicate<'a, 'a, &'a bool>,
    invariant: &'a Predicate<'a, 'a, &'a bool>,
}

impl<'a> Surface for BorrowedSurface<'a> {
    type Location = &'a bool;

    fn states(&self) -> impl Iterator<Item = &Attribute<Self::Location>> {
        core::iter::once(self.state)
    }

    fn transformers(&self) -> impl Iterator<Item = &Operation<'_, '_, Self::Location>> {
        core::iter::once(self.transformer)
    }

    fn axioms(&self) -> impl Iterator<Item = &Predicate<'_, '_, Self::Location>> {
        core::iter::once(self.axiom)
    }

    fn invariants(&self) -> impl Iterator<Item = &Predicate<'_, '_, Self::Location>> {
        core::iter::once(self.invariant)
    }
}

fn inspect(surface: &impl Surface) {
    let state = surface.states().next().unwrap();
    let operation = surface.transformers().next().unwrap();
    let axiom = surface.axioms().next().unwrap();
    let invariant = surface.invariants().next().unwrap();
    assert!(core::ptr::eq(state, operation.effect.outputs[0]));
    assert!(core::ptr::eq(state, axiom.inputs[0]));
    assert!(core::ptr::eq(state, invariant.inputs[0]));
    // Sharing one predicate never collapses its two distinct semantic roles.
    assert!(core::ptr::eq(axiom, invariant));
    assert_eq!(surface.axioms().count(), 1);
    assert_eq!(surface.invariants().count(), 1);
}

#[test]
fn a_local_borrowed_surface_preserves_relationships_and_constraint_roles() {
    let actual_state = true;
    let state = Attribute {
        ty: Type::Bool,
        location: &actual_state,
    };
    let signature = [Type::Bool];
    let inputs = [&state];
    let condition = Definition {
        body: Expr::Input(0),
        arguments: &[],
        inputs: &signature,
    };
    let predicate = Predicate {
        definition: &condition,
        inputs: &inputs,
    };
    let permitted = Definition {
        body: Expr::Constant(Value::boolean(true)),
        arguments: &[],
        inputs: &[],
    };
    let results = [Expr::Argument(0)];
    let operation = Operation {
        precondition: Predicate {
            definition: &permitted,
            inputs: &[],
        },
        effect: Effect {
            inputs: &[],
            outputs: &inputs,
            results: &results,
            arguments: &signature,
        },
    };
    let surface = BorrowedSurface {
        state: &state,
        transformer: &operation,
        axiom: &predicate,
        invariant: &predicate,
    };
    inspect(&surface);
    assert!(core::ptr::eq(
        surface.states().next().unwrap().location,
        core::ptr::from_ref(&actual_state)
    ));
}

//! A statically declared relay surface. No IR is executed by this example.

use core::sync::atomic::AtomicBool;
use lattice_ir::{Attribute, Definition, Effect, Expr, Operation, Predicate, Type, Value};

static POWER_STATE: AtomicBool = AtomicBool::new(false);

// The firmware supplies the location of its actual state. The IR only describes it.
static POWER: Attribute<&AtomicBool> = Attribute {
    ty: Type::Bool,
    location: &POWER_STATE,
};

// One shared mathematical definition: the new value is the invocation argument.
static ASSIGN: Definition<'static, 'static, &AtomicBool> = Definition {
    body: Expr::Argument(0),
    arguments: &[Type::Bool],
    inputs: &[],
};

static PERMITTED: Definition<'static, 'static, &AtomicBool> = Definition {
    body: Expr::Constant(Value::boolean(true)),
    arguments: &[],
    inputs: &[],
};

static SET_POWER: Operation<'static, 'static, &AtomicBool> = Operation {
    precondition: Predicate {
        definition: &PERMITTED,
        inputs: &[],
    },
    effect: Effect {
        inputs: &[],
        outputs: &[&POWER],
        results: &[Expr::Reference(&ASSIGN)],
        arguments: &[Type::Bool],
    },
};

fn main() {
    // A vertex enumerator or nucleus compiler can inspect this same definition.
    core::hint::black_box(&SET_POWER);
}

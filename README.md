# Lattice IR

Shared mathematical definitions for Lattice. A vertex firmware uses this crate
to describe its attributes and state-transforming operations. The nucleus uses
those definitions for its proof, synthesis, and compilation work.

The crate is `no_std`, has no dependencies, and uses no allocator or unsafe code.
Definitions borrow expressions, attributes, and signatures. They can be declared
statically in firmware. There is no interpreter, instruction tape, compiler,
solver, graph worker, retry engine, persistence layer, or transport implementation.

## Definitions

- `Integer`, `Type`, and `Value` describe Boolean and signed/unsigned 8-, 16-, 32-,
  and 64-bit scalar literals. Literal construction checks representable bounds.
  Boolean and integer literals can both be constructed in static definitions.
- `Unary` and `Binary` name the existing primitive operations and expose their
  operand/result type rules. They do not execute operations.
- `Expr` and `Definition` describe finite acyclic expressions with direct
  references, shared definitions, lazy Boolean control, arguments, and symbolic
  inputs. They require no generated node IDs or runtime registry.
- `Attribute<Location>` describes a typed state element and its correspondence
  to consumer-owned state. `Location` is supplied by the firmware or nucleus;
  the crate neither copies the current value nor accesses hardware.
- `Predicate`, `Effect`, and `Operation` describe permitted invocation and the
  joint transformation of affected attributes. Effect outputs are mathematical
  post-state, not return values that an executing vertex must send to a caller.
- `Surface` is the shared borrowing interface for enumerating an appliance's
  states, transformations, axioms, and invariants. Vertex implementations project
  their existing handles; nucleus can implement it over received definitions.
  Consumers see the same mathematical records without depending on the RTOS.

Surface axioms describe supplied facts/assumptions. Invariants describe properties
that admitted transitions must preserve; they are distinct from individual
operation preconditions. Both use ordinary argument-free Boolean predicates with
direct state inputs. The interface introduces no registry, copied definition
table, native execution, or fixed storage layout. Locations remain consumer-owned.

The [relay example](examples/relay.rs) declares a complete operation as static
data, referring to actual firmware storage. Firmware connects its hardware
callable to that operation; the IR definition supplies its mathematical meaning.

Whole-definition validation and interpretation belong to the nucleus. Public
expression records are declarations, not a claim of proof or executable authority.
Consumers must check signatures, input/output correspondence, permitted read
semantics, and acyclic references before admitting a definition. This crate
contains no Jarvis-specific proof restrictions or search limits.

## Retained arithmetic meaning

Arithmetic is defined at the declared integer width, independently of Rust's
overflow behavior. Addition, subtraction, and multiplication wrap. Signed
division rounds toward zero; division by zero produces all-one bits, and signed
minimum divided by minus one produces signed minimum. Remainder by zero produces
the dividend; signed minimum modulo minus one produces zero. Shift counts are
unsigned, with zero fill for oversized logical shifts and sign fill for
oversized arithmetic shifts. Conversions are explicit. Boolean control is lazy;
scalar binary operands have left-to-right order.

These meanings describe the IR. Their implementations and any cost analysis,
optimization, or proof machinery belong to the nucleus. Shared expression
structure does not authorize caching separate hardware observations.

## Extraction

Derived from `zacharytomlinson/jarvis`, branch `zmt/rewrite`, commit
`253b3ee4e5f261cc33759fb123666d8e8d3ee9e4`:

- `peripheral-graph/src/scalar/value.rs`: retained literal representation,
  primitive vocabulary, and typing; removed primitive execution methods and
  interpreter/compiler-specific errors.
- `peripheral-graph/src/scalar/mod.rs`: retained the borrowed expression and
  definition model. Boolean reads and typed observations share `Expr::Read`;
  Boolean literals use `Expr::Constant`. Source references replace lowered
  executable storage in effects.
- `peripheral-graph/src/operation.rs`: retained the precondition/effect
  association without native execution or owned graph state.

Jarvis's `Cell`-backed attributes are replaced by consumer-supplied locations.
Its owned `NodeId`/`EntryId` containers, JIR importer/encoder, allocator-based
builders, local graph executor, and Z3 implementation were not imported. The
borrowed definitions are not a frozen wire format or a deployed executable.
Representation and encoding can evolve with the nucleus implementation.

## Checks

```sh
cargo test --locked --offline
cargo check --locked --offline --examples
cargo check --locked --offline --target thumbv6m-none-eabi
cargo clippy --locked --offline --all-targets -- -D warnings
cargo fmt --check
```

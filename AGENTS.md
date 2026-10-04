# Lattice IR development guidance

This crate owns shared mathematical definitions used by vertex firmware and the
nucleus. Read README.md and preserve the extraction boundary.

Keep the core `no_std`, allocation-free, and independent of runtime storage,
device drivers, networking, and native callbacks. Use direct expression and
attribute references. Do not introduce NodeId registries or relational models.

Do not add an interpreter, solver, compiler, graph executor, retry machinery,
host persistence, or Jarvis-specific proof policies here. Those implementations
belong to the nucleus or the consuming kernel. Do not change sibling repositories
as a side effect of work in this crate.

Keep changes narrow and checks proportional to the change. Preserve one common
IR for state, effects, conditions, and constraints; avoid behavior-specific
opcodes and speculative abstractions.

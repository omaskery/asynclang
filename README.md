# Overview

In my embedded software work I've noticed that a lot of time is spent creating, in one form or another, state machines - manually structuring code to do a minimal amount of work and drop out to the main loop in order to not block execution.

This project is simply my foray into making a (predominantly toy) programming language that features await-style syntax but transpiles to C and thus usable on all platforms with a C compiler.

# Goals

1. Transpile 'asynclang' into C
2. Have await syntax which boils down to continuation-passing (callbacks)
3. Try to avoid forced dynamic memory allocations?
4. Simple generics?

# Status

This repository is pretty sparse, but progress so far includes:
- Parts of a hypothetical syntax exist, but are in no way final
- There's some code for representing parts of an AST in memory
- There's some code for generating a CFG from the AST

# To do:

- [ ] Start emitting some C output
- [ ] Do some type-checking
- [ ] Write a lexer + parser

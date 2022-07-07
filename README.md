# Rust Meta-Programming Lab

Experiments with meta-programming in Rust.

We explore a common topic in business applications - the need to model data and build user interfaces to display them.

We create a meta-language to hold this and generate the back-end (Rust) data structures and
associated meta-model for exporting to the client for creating sufficient (but perhaps not super elegant) user interfaces
for that data.

## Project Structure

### `metamodel`
This is the crate with the meta-model types.

### `metamodel_macros`
This is the Cargo crate with the meta-model macros. It must be in a separate crate since Rust procedural macros (`proc_macro`) are compiler extentions, so
they must be compile in a whole compile-unit before they can be used.

This kind of crate can only export the macro functions, hence the types used are in the crate mentioned above.

### `metamodel_test`
This crate contains tests for the meta-model.

### `metaprogramming_lab`
This crate contains examples of how to use the meta-model.


## Syn Library
The Syn library parses Rust TokenStreams into an AST.

Many of its features are turned off by default, and must be enabled via feature flags in the [`Cargo.toml]`(./metamodel_macros/Cargo.toml) file:

- `full` enables the Expr types
- `extra-traits` enables the Debug traits and more

The latter is very helpful. With the `Debug` trait defined in `extra-traits` you have a chance to print and figure out the AST that the macro is getting as input.


## Literature
Read more about Rust macros:

- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/introduction.html)
- [Introduction to Procedural Macros in Rust](https://tinkering.xyz/introduction-to-proc-macros/)

## License
See the [LICENSE](./LICENSE) file.

(C) 2022 by Martin Jul

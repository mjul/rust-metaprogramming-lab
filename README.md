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

## Approaches

### 1: Generate code from inline metamodel AST (`generate_data_structures!`)
The first approach.

The idea was to pass a meta-model AST instance to the macro inline and then pass
it to the code generator.

The problem is that the macro would have to parse the AST expression and instantiate
it at compile-time to pass it to the code generator since the code-gen works on a metamodel AST,
not the Rust syntax tree representation for building it.

This shows a limitation of Rust: it does not have the whole language available all the time, it works in discrete units
of *crates*. We can appreciate that it makes it much easier to build the compiler (*e.g.* reasoning about traits),
but it is a trade-off.

Trying to make the meta-model available at compile-time by making it a `const` expression
is also the reason why the AST enums take `&'static str` and not owned Strings.

It did give some insight into how complicated it is to work with `syn` and the Rust syntax tree.

See the [tests](./metamodel_test/src/lib.rs) and the [macro code](./metamodel_macros/src/lib.rs).

#### Directions for Future Studies
Perhaps instead of defining the meta-model AST inline as a parameter to the macro, the model
could be adapted a bit by placing it in a module known to the
meta-model macro code, so it could be referenced from there.

This would couple the macros tightly to the metamodel but since the model is project specific
that might be acceptable even if it couples the the macros tightly to not just the model AST but
the actual model instance for the project.


### 2: Tuple-based DSL (`generate_model_from_tuple!`)
This was the second attempt.

It is a tractable approach. However, the macro code is much more complicated since it has
to parse s-expression style Rust expressions given to the macro,
then generate the meta-model AST from that, and then send that to the
code generator.

The code generator, now residing in the `codegen` module, uses the
`quote` crate as the dual of `syn` to emit Rust code from the Rust AST.

This approach feels like too much work.


## Directions for Future Studies

- Consider adding a code-generation build step to the Cargo build as way to work around the
  under-powered macro features of Rust, see
  [The Cargo Book: Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)


## Literature
Read more about Rust macros:

- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/introduction.html)
- [Introduction to Procedural Macros in Rust](https://tinkering.xyz/introduction-to-proc-macros/)

Read more about Cargo build scripts:
- [The Cargo Book: Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)


## License
See the [LICENSE](./LICENSE) file.

(C) 2022 by Martin Jul

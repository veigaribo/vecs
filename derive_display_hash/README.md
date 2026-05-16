Provides the `DisplayHash` derive macro that implements the `Hash` trait of `Display`
types by consuming the output of the `fmt` method, without allocating.

The derive macro was originally something more complex, so that's why I decided to
make it a derive macro. It could easily become a `macro_rules!` right now.

In order to use the output of this macro, you need to:

- Enable the `formatting_options` unstable feature (`#![feature(formatting_options)]`);
- Have the companion `display_hash` crate available.

# `recrust`

This repo is a small experimental **RSX/JSX-like syntax** for Rust built as a **proc-macro**.

- **`recrust_macro`** provides the `rsx! { ... }` macro.
- **`recrust_ast`** parses `<tag ...>...</tag>`-style input (including nested RSX inside `{ ... }`) and rewrites it into normal Rust tokens.

The current expansion looks something like:

```rust
create_element(tag, vec![("prop_name", prop_value), ...])
```
# src-embed

`src-embed` is a tiny procedural attribute macro that embeds the original
Rust source of an annotated item into the compiled crate as a `pub const`
string. The generated constant is marked `#[doc(hidden)]` and the original
item is re-emitted unchanged so the macro preserves semantics while making the
textual source available at compile time or runtime.

## Motivation

Sometimes it's useful to keep the literal source of a type, function, trait,
or impl block available in the binary for tests, examples, debug output, or
documentation tooling. `src-embed` provides a simple, dependency-light way to
make that source accessible.

## Features

- Works on `struct`, `enum`, `fn`, `trait`, and `impl` items.
- Generates a `pub const __<NAME>_SOURCE__: &str` containing the source.
- Keeps the original item intact and hidden from the public docs.

Note: the embedded source includes the original attributes and doc
comments associated with the annotated item. This ensures `///` doc
comments (and other outer attributes) appear in the generated string.

## Usage

Add the crate as a dependency (path or registry as appropriate) and enable the
`proc-macro` feature in your `Cargo.toml` if using a local crate.

```toml
[dependencies]
src-embed = { path = ".." }
```

Then annotate an item:

```rust
use src_embed::src_embed;

#[src_embed]
pub struct Foo { pub x: u32 }

// You can now refer to `__FOO_SOURCE__` (it is `pub`, but marked
// `#[doc(hidden)]`).
```

## Example

Given:

```rust
#[src_embed]
pub fn example() -> &'static str { "hello" }
```

The macro will emit something like:

```rust
#[doc(hidden)]
pub const __EXAMPLE_SOURCE__: &str = "pub fn example() -> &'static str { \"hello\" }";

pub fn example() -> &'static str { "hello" }
```

## Notes

- The constant name is generated from the item's identifier in uppercase and
  wrapped between `__` and `_SOURCE__`. For certain items (like impls) the
  macro derives a representative name; when it cannot, it falls back to
  `ITEM` or `UNKNOWN`.

## License

This repository is provided under the terms of the included `LICENSE` file.

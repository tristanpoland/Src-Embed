//! src-embed — small helper to embed an item's source as a const string.
//!
//! This crate exposes a single attribute procedural macro, `src_embed`, which
//! captures the original source code of the annotated item and emits it as a
//! `pub const` string (hidden from generated documentation). The original item
//! is re-emitted unchanged so the macro is transparent to callers while still
//! providing access to the textual source at compile time.
//!
//! The macro is useful for tests, debugging, documentation generators, or any
//! scenario where you want the literal source of an item available at runtime
//! or in compiled artifacts.
//!
//! # Supported items
//! - `struct`, `enum`, `fn`, `trait`, and `impl` blocks
//!
//! # Example
//! ```rust
//! use src_embed::src_embed;
//!
//! #[src_embed]
//! pub struct Foo { pub x: u32 }
//!
//! // Expands to something like:
//! // #[doc(hidden)]
//! // pub const __FOO_SOURCE__: &str = "pub struct Foo { pub x: u32 }";
//! // pub struct Foo { pub x: u32 }
//! ```

use proc_macro::TokenStream;
use quote::quote;

/// Attribute macro that embeds the original source of the annotated item.
///
/// When applied to an item (for example a `struct`, `enum`, `fn`, `trait`, or
/// `impl` block) this macro produces a `pub const` string containing the
/// textual source of that item and re-emits the original item unchanged.
///
/// The generated constant name is formed from the item's identifier in
/// uppercase, wrapped between `__` and `_SOURCE__` (for example a `struct` named
/// `Foo` will produce `__FOO_SOURCE__`). The constant is marked
/// `#[doc(hidden)]` so it does not appear in normal documentation output.
///
/// # Notes
/// - If the macro cannot determine a sensible identifier (for example for
///   certain anonymous or complex items) it falls back to `ITEM` or
///   `UNKNOWN` in the generated constant name.
/// - The macro is intentionally conservative and re-emits the original item
///   so it does not alter semantics.
///
/// # Example
/// ```rust
/// use src_embed::src_embed;
///
/// #[src_embed]
/// pub fn example() -> &'static str { "hi" }
///
/// // The crate will also provide a constant like `__EXAMPLE_SOURCE__` with
/// // the textual source of the function.
/// ```
#[allow(unused)]
#[proc_macro_attribute]
pub fn src_embed(args: TokenStream, input: TokenStream) -> TokenStream {
    use syn::{parse_macro_input, Item, Type};

    // Parse the input - accepts any Rust item (trait, impl, struct, etc.)
    let input_parsed = parse_macro_input!(input as Item);

    // Preserve the original token stream text (this includes attributes
    // such as doc comments). We capture the raw input *before* parsing so
    // that the embedded string reflects the original source as written.
    let raw_source = input.to_string();
    let source_code = syn::LitStr::new(&raw_source, proc_macro2::Span::call_site());

    // Extract the name of the item to generate a unique const name
    let item_name = match &input_parsed {
        Item::Trait(trait_item) => trait_item.ident.to_string(),
        Item::Impl(impl_item) => {
            // For impl blocks, extract the type being implemented for
            if let Type::Path(type_path) = &*impl_item.self_ty {
                type_path
                    .path
                    .segments
                    .last()
                    .map(|seg| seg.ident.to_string())
                    .unwrap_or_else(|| "UNKNOWN".to_string())
            } else {
                "UNKNOWN".to_string()
            }
        }
        Item::Struct(struct_item) => struct_item.ident.to_string(),
        Item::Enum(enum_item) => enum_item.ident.to_string(),
        Item::Fn(fn_item) => fn_item.sig.ident.to_string(),
        _ => "ITEM".to_string(),
    };

    // Generate a const name: __ITEMNAME_SOURCE__
    let const_ident = syn::Ident::new(
        &format!("__{}_SOURCE__", item_name.to_uppercase()),
        proc_macro2::Span::call_site(),
    );

    // Generate the output: const definition + original item. We use the
    // captured `raw_source` as a `&'static str` literal so the embedded
    // constant contains the original source text (including doc comments).
    let expanded = quote! {
        #[doc(hidden)]
        pub const #const_ident: &str = #source_code;

        #input_parsed
    };

    TokenStream::from(expanded)
}
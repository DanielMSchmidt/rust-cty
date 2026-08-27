//! Derive macros for the `cty` crate's interop traits — the Rust analogue of
//! go-cty's `gocty` struct-tag reflection.
//!
//! **Everything the derives emit is a `todo!()` stub**, matching the rest of
//! the workspace: the derives exist so conformance tests can declare tagged
//! structs that compile; the generated decoding/encoding logic is Daniel's to
//! write. See `docs/api-mapping.md` for the attribute grammar.
//!
//! Field attribute grammar (mirroring Go's `cty:"name"` struct tags):
//!
//! - `#[cty(attr = "name")]` — the field maps to the object attribute `name`
//!   (go-cty: a `cty:"name"` tag). Fields without this attribute are ignored
//!   for object conversion, exactly as untagged Go fields are.
//! - No field attributes are used for tuple conversion: struct fields map to
//!   tuple elements positionally, as in gocty.
//! - `Option<T>` fields are the analogue of Go pointer fields (`None` ↔ null).
//! - Deriving on a single-field tuple struct (`struct MyString(String);`) is
//!   the analogue of a Go defined type (`type stringAlias string`), delegating
//!   to the inner type's conversion.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derives `cty::interop::IntoCty` (go-cty: `gocty.ToCtyValue` over a
/// reflected struct). Accepts `#[cty(attr = "name")]` field attributes.
#[proc_macro_derive(IntoCty, attributes(cty))]
pub fn derive_into_cty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::cty::interop::IntoCty for #name #ty_generics #where_clause {
            fn into_cty(self, ty: &::cty::Type) -> ::core::result::Result<::cty::Value, ::cty::Error> {
                let _ = ty;
                todo!()
            }
        }
    }
    .into()
}

/// Derives `cty::interop::FromCty` (go-cty: `gocty.FromCtyValue` into a
/// reflected struct). Accepts `#[cty(attr = "name")]` field attributes.
#[proc_macro_derive(FromCty, attributes(cty))]
pub fn derive_from_cty(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::cty::interop::FromCty for #name #ty_generics #where_clause {
            fn from_cty(value: &::cty::Value) -> ::core::result::Result<Self, ::cty::Error> {
                let _ = value;
                todo!()
            }
        }
    }
    .into()
}

/// Derives `cty::interop::CtyTyped` (go-cty: `gocty.ImpliedType` over a
/// reflected struct: an object type built from the `#[cty(attr = "…")]`
/// fields). Accepts `#[cty(attr = "name")]` field attributes.
#[proc_macro_derive(CtyTyped, attributes(cty))]
pub fn derive_cty_typed(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::cty::interop::CtyTyped for #name #ty_generics #where_clause {
            fn implied_type() -> ::core::result::Result<::cty::Type, ::cty::Error> {
                todo!()
            }
        }
    }
    .into()
}

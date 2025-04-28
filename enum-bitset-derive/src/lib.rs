//! Proc macros for the [enum-bitset](https://crates.io/crates/enum-bitset) crate.
//!
//! You do not need to depend directly on this crate, the macro is conveniently reexported by the [enum-bitset](https://crates.io/crates/enum-bitset) crate.


extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod derive;

#[proc_macro_derive(EnumBitset, attributes(bitset))]
pub fn enum_bitset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive::derive_enum_bitset(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

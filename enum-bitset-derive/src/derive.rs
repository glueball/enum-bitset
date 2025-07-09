use std::convert::TryInto;

use heck::ToSnakeCase;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::derive::config::EnumBitsetConfig;

mod config;
mod core_traits;
mod doc;
mod inherent;
mod iter;
mod mask;
mod ops;
mod serde;

pub fn derive_enum_bitset(input: DeriveInput) -> syn::Result<TokenStream2> {
    let config: EnumBitsetConfig = input.try_into()?;

    let inner_ty = &config.inner_type;
    let base_ty = &config.base_type;
    let name = &config.set_type;
    let iter = &config.iter_type;
    let base_vis = &config.base_vis;
    let inner_vis = &config.inner_vis;

    let module = format_ident!("__{}_enum_bitset", name.to_string().to_snake_case());

    let to_value_branches = config.base_to_value_branches();
    let core_traits = config.derive_core_traits();
    let impl_debug = config.derive_debug();
    let inherent = config.impl_inherent();
    let impl_iter = config.impl_iter();
    let impl_serde = config.impl_serde();
    let impl_ops = config.impl_ops();
    let doc = config.set_docs();

    Ok(quote! {
        #[doc(inline)]
        #[allow(unused_imports)]
        #base_vis use #module::{#name, #iter};

        #[doc(hidden)]
        mod #module {
            #![allow(unused_imports)]
            #![allow(dead_code)]

            use super::*;
            use ::core::{
                borrow::Borrow,
                clone::Clone,
                cmp::{Eq, PartialEq},
                convert::From,
                default::Default,
                fmt::{self, Debug, Display, Formatter, write},
                iter::{IntoIterator, Iterator, Extend, FromIterator},
                marker::PhantomData,
                option::Option,
            };

            const _: fn() = || {
                fn base_enum_for_bitsets_must_be_clonable<T: Clone + ?Sized>() {}
                base_enum_for_bitsets_must_be_clonable::<#base_ty>();
            };

            #doc
            #[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
            #[repr(transparent)]
            #inner_vis struct #name {
                // Invariant: only the N-th last bits may be 1 (where N is the number of variants)
                items: #inner_ty,
            }

            #core_traits
            #inherent
            #impl_debug
            #impl_iter
            #impl_serde
            #impl_ops

            #[inline]
            const fn base_to_value(value: &#base_ty) -> #inner_ty {
                return match value {
                    #(#to_value_branches),*
                }
            }
        }
    })
}

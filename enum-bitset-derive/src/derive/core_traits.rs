use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::LitStr;

use crate::derive::config::EnumBitsetConfig;

impl EnumBitsetConfig {
    pub fn derive_core_traits(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;

        let to_value_branches = self.base_to_value_branches();

        quote! {
            impl Default for #name {
                #[inline(always)]
                fn default() -> Self {
                    Self { items: 0 }
                }
            }

            impl From<#base_ty> for #name {
                #[inline(always)]
                fn from(item: #base_ty) -> Self {
                    (&item).into()
                }
            }

            impl From<&#base_ty> for #name {
                #[inline(always)]
                fn from(item: &#base_ty) -> Self {
                    let items = match item {
                        #(#to_value_branches),*
                    };

                    Self { items }
                }
            }
        }
    }

    pub fn derive_debug(&self) -> TokenStream2 {
        if !self.debug {
            return TokenStream2::new();
        }

        let name = &self.set_type;
        let base_ty = &self.base_type;
        let my_crate = &self.my_crate;

        let name_lit = LitStr::new(&name.to_string(), Span::call_site());

        quote! {
            use #my_crate::debug_impl::{NoDebug, DebugWrapper};

            impl Debug for #name {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    f.write_str(#name_lit)?;

                    if f.alternate() {
                        write!(f, " ({}) ", self.len())?;
                    }
                    else {
                        write!(f, "({})", self.len())?;
                    }

                    DebugWrapper::<#base_ty>(PhantomData).debug_entries(f, self.len(), self.iter())
                }
            }
        }
    }
}

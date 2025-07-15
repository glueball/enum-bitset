use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::derive::config::EnumBitsetConfig;

impl EnumBitsetConfig {
    pub fn base_impl(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let set_type = &self.set_type;
        let doc = format!(
            r#"Creates a new [`{set_type}`] from a [`{base_ty}`] value. Can be used in
               const contexts."#
        );

        quote! {
            #[doc = #doc]
            impl #base_ty {
                pub const fn as_bitset(&self) -> #set_type {
                    #set_type { items: base_to_value(self) }
                }
            }
        }
    }
}

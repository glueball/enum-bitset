use proc_macro2::TokenStream;
use quote::quote;

use crate::derive::config::EnumBitsetConfig;

impl EnumBitsetConfig {
    pub fn set_docs(&self) -> TokenStream {
        let name = &self.set_type;
        let base_ty = &self.base_type;
        let inner_ty = &self.inner_type;
        let len = self.variants.len();

        let doc1 = format!("A set of [`{base_ty}`] values, efficiently implemented as a bitfield.");
        let doc2 =
            format!("It is internally implemented as a single [`{inner_ty}`] integer value.");
        let doc3 = format!(
            "Only the last {len} (the number of variants in [`{base_ty}`]) may be non-zero."
        );
        let doc4 = format!(
            "It is guaranteed that the layout and ABI of a [`{name}`] is exactly the same of a [`{inner_ty}`]."
        );
        let doc5 =
            format!("This is true regardless of any `#[repr(..)]` attribute set on [`{base_ty}`].");

        quote! {
                #[doc = #doc1]
                ///
                #[doc = #doc2]
                /// The N-th least significant bit of the value is set to `1` if the N-th variant is present in the set, and `0` otherwise.
                ///
                /// # Representation
                #[doc = #doc4]
                #[doc = #doc5]
                ///
                ///
                /// # Invariant
                #[doc = #doc3]
                /// That is, the value of bits that do not correspond to an existing variant *must* always be set to 0.
                /// All the intrinsic methods will respect this invariant, but please take this into account when manipulating
                /// the state of the set via unsafe or using it in FFI.
        }
    }
}

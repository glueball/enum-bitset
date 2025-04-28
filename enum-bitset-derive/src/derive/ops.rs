use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::derive::config::EnumBitsetConfig;

impl EnumBitsetConfig {
    pub(crate) fn impl_ops(&self) -> TokenStream2 {
        let mut mandatory = self.impl_ops_mandatory();

        if self.base_add {
            mandatory.extend(self.impl_base_add());
        }
        mandatory
    }

    fn impl_ops_mandatory(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;

        quote! {
            use ::core::ops::*;

            impl Add<#name> for #name {
                type Output = Self;

                fn add(self, rhs: Self) -> Self {
                    Self { items: self.items | rhs.items}
                }
            }

            impl Add<&#name> for #name {
                type Output = Self;

                fn add(self, rhs: &Self) -> Self {
                    Self { items: self.items | rhs.items}
                }
            }

            impl Add<#base_ty> for #name {
                type Output = Self;

                fn add(mut self, rhs: #base_ty) -> Self {
                    Self { items: self.items | base_to_value(&rhs) }
                }
            }

            impl Add<&#base_ty> for #name {
                type Output = Self;

                fn add(mut self, rhs: &#base_ty) -> Self {
                    Self { items: self.items | base_to_value(rhs) }
                }
            }

            impl AddAssign<#name> for #name {
                fn add_assign(&mut self, rhs: Self) {
                    self.items |= rhs.items;
                }
            }

            impl AddAssign<&#name> for #name {
                fn add_assign(&mut self, rhs: &Self) {
                    self.items |= rhs.items;
                }
            }

            impl AddAssign<#base_ty> for #name {
                fn add_assign(&mut self, rhs: #base_ty) {
                    self.items |= base_to_value(&rhs);
                }
            }

            impl AddAssign<&#base_ty> for #name {
                fn add_assign(&mut self, rhs: &#base_ty) {
                    self.items |= base_to_value(rhs);
                }
            }

            impl Sub<#name> for #name {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    Self { items: self.items & !rhs.items }
                }
            }

            impl Sub<&#name> for #name {
                type Output = Self;

                fn sub(self, rhs: &Self) -> Self {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    Self { items: self.items & !rhs.items }
                }
            }

            impl Sub<#base_ty> for #name {
                type Output = Self;

                fn sub(self, rhs: #base_ty) -> Self {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    Self { items: self.items & !base_to_value(&rhs) }
                }
            }

            impl Sub<&#base_ty> for #name {
                type Output = Self;

                fn sub(self, rhs: &#base_ty) -> Self {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    Self { items: self.items & !base_to_value(rhs) }
                }
            }

            impl SubAssign<#name> for #name {
                fn sub_assign(&mut self, rhs: Self) {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    self.items &= !rhs.items;
                }
            }

            impl SubAssign<&#name> for #name {
                fn sub_assign(&mut self, rhs: &Self) {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    self.items &= !rhs.items;
                }
            }

            impl SubAssign<#base_ty> for #name {
                fn sub_assign(&mut self, rhs: #base_ty) {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    self.items &= !base_to_value(&rhs);
                }
            }

            impl SubAssign<&#base_ty> for #name {
                fn sub_assign(&mut self, rhs: &#base_ty) {
                    // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                    self.items &= !base_to_value(rhs);
                }
            }

            impl BitAnd<#name> for #name {
                type Output = Self;

                fn bitand(self, rhs: Self) -> Self {
                    Self { items: self.items & rhs.items }
                }
            }

            impl BitAnd<&#name> for #name {
                type Output = Self;

                fn bitand(self, rhs: &Self) -> Self {
                    Self { items: self.items & rhs.items }
                }
            }

            impl BitAndAssign<#name> for #name {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.items &= rhs.items;
                }
            }

            impl BitAndAssign<&#name> for #name {
                fn bitand_assign(&mut self, rhs: &Self) {
                    self.items &= rhs.items;
                }
            }

            impl BitOr<#name> for #name {
                type Output = Self;

                fn bitor(self, rhs: Self) -> Self {
                    Self { items: self.items | rhs.items }
                }
            }

            impl BitOr<&#name> for #name {
                type Output = Self;

                fn bitor(self, rhs: &Self) -> Self {
                    Self { items: self.items | rhs.items }
                }
            }

            impl BitOr<#base_ty> for #name {
                type Output = Self;

                fn bitor(mut self, rhs: #base_ty) -> Self {
                    Self { items: self.items | base_to_value(&rhs) }
                }
            }

            impl BitOr<&#base_ty> for #name {
                type Output = Self;

                fn bitor(mut self, rhs: &#base_ty) -> Self {
                    Self { items: self.items | base_to_value(rhs) }
                }
            }

            impl BitOrAssign<#name> for #name {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.items |= rhs.items;
                }
            }

            impl BitOrAssign<&#name> for #name {
                fn bitor_assign(&mut self, rhs: &Self) {
                    self.items |= rhs.items;
                }
            }

            impl BitXor<#name> for #name {
                type Output = Self;

                fn bitxor(self, rhs: Self) -> Self {
                    // No `Self::MASK &` necessary, since `self.items` and `rhs.items` must have invalid bits zeroed, 0^0 = 0.
                    Self { items: self.items ^ rhs.items }
                }
            }

            impl BitXor<&#name> for #name {
                type Output = Self;

                fn bitxor(self, rhs: &Self) -> Self {
                    // No `Self::MASK &` necessary, since `self.items` and `rhs.items` must have invalid bits zeroed, 0^0 = 0.
                    Self { items: self.items ^ rhs.items }
                }
            }

            impl BitXorAssign<#name> for #name {
                fn bitxor_assign(&mut self, rhs: Self) {
                    // No `Self::MASK &` necessary, since `self.items` and `rhs.items` must have invalid bits zeroed, 0^0 = 0.
                    self.items ^= rhs.items;
                }
            }

            impl BitXorAssign<&#name> for #name {
                fn bitxor_assign(&mut self, rhs: &Self) {
                    // No `Self::MASK &` necessary, since `self.items` and `rhs.items` must have invalid bits zeroed, 0^0 = 0.
                    self.items ^= rhs.items;
                }
            }

            impl Not for #name {
                type Output = Self;

                fn not(self) -> Self {
                    Self { items: (!self.items) & Self::MASK }
                }
            }
        }
    }


    fn impl_base_add(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;

        quote! {
            impl Add<#base_ty> for #base_ty {
                type Output = #name;

                fn add(self, rhs: #base_ty) -> #name {
                    #name { items: base_to_value(&self) | base_to_value(&rhs) }
                }
            }

            impl Add<&#base_ty> for #base_ty {
                type Output = #name;

                fn add(self, rhs: &#base_ty) -> #name {
                    #name { items: base_to_value(&self) | base_to_value(rhs) }
                }
            }

            impl Add<&#base_ty> for &#base_ty {
                type Output = #name;

                fn add(self, rhs: &#base_ty) -> #name {
                    #name { items: base_to_value(self) | base_to_value(rhs) }
                }
            }

            impl Add<#base_ty> for &#base_ty {
                type Output = #name;

                fn add(self, rhs: #base_ty) -> #name {
                    #name { items: base_to_value(self) | base_to_value(&rhs) }
                }
            }

            impl BitOr<#base_ty> for #base_ty {
                type Output = #name;

                fn bitor(self, rhs: #base_ty) -> #name {
                    #name { items: base_to_value(&self) | base_to_value(&rhs) }
                }
            }

            impl BitOr<&#base_ty> for #base_ty {
                type Output = #name;

                fn bitor(self, rhs: &#base_ty) -> #name {
                    #name { items: base_to_value(&self) | base_to_value(rhs) }
                }
            }

            impl BitOr<&#base_ty> for &#base_ty {
                type Output = #name;

                fn bitor(self, rhs: &#base_ty) -> #name {
                    #name { items: base_to_value(self) | base_to_value(rhs) }
                }
            }

            impl BitOr<#base_ty> for &#base_ty {
                type Output = #name;

                fn bitor(self, rhs: #base_ty) -> #name {
                    #name { items: base_to_value(self) | base_to_value(&rhs) }
                }
            }
        }
    }
}

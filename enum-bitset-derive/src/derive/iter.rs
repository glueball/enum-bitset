use proc_macro2::TokenStream as TokenStream2;

use crate::derive::config::EnumBitsetConfig;

impl EnumBitsetConfig {
    pub fn impl_iter(&self) -> TokenStream2 {
        let name = &self.set_type;
        let iter = &self.iter_type;
        let base_ty = &self.base_type;
        let inner_ty = &self.inner_type;
        let inner_vis = &self.inner_vis;

        let doc1 = format!("Iterator returned by the [`iter`]({name}::iter) method.");
        let doc2 = format!(
            "[`{iter}`] iterates over the variants of [`{base_ty}`] contained in a [`{name}`] instance."
        );

        quote::quote! {
            #[doc = #doc1]
            ///
            #[doc = #doc2]
            #[derive(Clone)]
            #inner_vis struct #iter {
                items: #inner_ty,
            }

            impl Iterator for #iter {
                type Item = #base_ty;

                fn next(&mut self) -> Option<#base_ty> {
                    if self.items == 0 {
                        return Option::None;
                    }

                    let index = self.items.trailing_zeros();

                    // Safety:
                    // the shift won't panic as it is guaranteed to be <= the size of the inner type.
                    self.items &= !(1 << index);

                    // Safety:
                    // the index op won't panic as long
                    // as the invariant of the items is satisfied
                    // (that is, that only the last N bits are 1).
                    Option::Some(#name::VARIANTS[index as usize].clone())
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let n = self.items.count_ones() as usize;
                    (n, Option::Some(n))
                }

                fn count(self) -> usize {
                    self.items.count_ones() as usize
                }
            }

            impl FromIterator<#base_ty> for #name {
                fn from_iter<T: IntoIterator<Item = #base_ty>>(iter: T) -> Self {
                    Self::from(iter)
                }
            }


            impl<'a> FromIterator<&'a #base_ty> for #name {
                fn from_iter<T: IntoIterator<Item = &'a #base_ty>>(iter: T) -> Self {
                    let items = iter.into_iter().fold(0, |acc, item| acc | base_to_value(item));

                    Self { items }
                }
            }

            impl Extend<#base_ty> for #name {
                fn extend<T: IntoIterator<Item = #base_ty>>(&mut self, iter: T) {
                    self.items |= iter.into_iter().fold(0, |acc, item| acc | base_to_value(&item));
                }
            }

            impl<'a> Extend<&'a #base_ty> for #name {
                fn extend<T: IntoIterator<Item = &'a #base_ty>>(&mut self, iter: T) {
                    self.items |= iter.into_iter().fold(0, |acc, item| acc | base_to_value(item));
                }
            }

            impl IntoIterator for #name {
                type Item = #base_ty;
                type IntoIter = #iter;

                fn into_iter(self) -> Self::IntoIter {
                    #iter { items: self.items }
                }
            }
        }
    }
}

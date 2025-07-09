use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::derive::config::EnumBitsetConfig;

impl EnumBitsetConfig {
    pub fn impl_inherent(&self) -> TokenStream2 {
        let name = &self.set_type;

        let items = [
            self.impl_mask(),
            self.impl_variants(),
            self.impl_new(),
            self.impl_from(),
            self.impl_from_slice(),
            self.impl_from_array(),
            self.impl_empty(),
            self.impl_all(),
            self.impl_is_empty(),
            self.impl_is_all(),
            self.impl_len(),
            self.impl_contains(),
            self.impl_contains_const(),
            self.impl_union(),
            self.impl_intersection(),
            self.impl_difference(),
            self.impl_symmetric_difference(),
            self.impl_complement(),
            self.impl_is_subset_of(),
            self.impl_is_superset_of(),
            self.impl_is_disjoint(),
            self.impl_is_complementary(),
            self.impl_insert(),
            self.impl_remove(),
            self.impl_insert_const(),
            self.impl_remove_const(),
            self.impl_iter_inherent(),
            self.impl_to_repr(),
            self.impl_from_repr(),
            self.impl_is_valid_repr(),
            self.impl_from_repr_unchecked(),
            self.impl_from_repr_masked(),
            self.impl_from_repr_discarded(),
            self.impl_collect(),
        ];

        quote! {
            impl #name {
                #(#items)*
            }
        }
    }


    fn impl_mask(&self) -> TokenStream2 {
        let inner_ty = &self.inner_type;
        let len = self.len();
        let mask = Self::mask(len);

        quote! {
            #[doc(hidden)]
            pub const MASK : #inner_ty = #mask;
        }
    }


    fn impl_variants(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let len = self.len();

        let array_items = self.variants.iter().map(move |variant| {
            let name = &variant.ident;
            quote! {#base_ty::#name}
        });

        quote!(
            // Holding the array of variants here allows us
            // to not rely on the representation of the value
            #[doc(hidden)]
            pub const VARIANTS: [#base_ty; #len] = [#(#array_items),*];
        )
    }


    fn impl_new(&self) -> TokenStream2 {
        let name = &self.set_type;

        let doc = format!("Creates a new empty [`{}`].", name);

        quote!(
            #[doc = #doc]
            pub const fn new() -> Self {
                Self { items: 0 }
            }
        )
    }


    fn impl_from(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;

        let doc = format!(
            r#"Creates a new [`{name}`] from an iterator that yields any borrowed type of 
              [`{base_ty}`] variants. Duplicated values are ignored."#
        );

        quote!(
               #[doc = #doc]
                pub fn from<T: IntoIterator<Item = I>, I: Borrow<#base_ty>>(iter: T) -> Self {
                    let items = iter.into_iter()
                                    .fold(0, |acc, item| acc | base_to_value(item.borrow()));

                    Self{ items }
                }
        )
    }


    fn impl_from_slice(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;
        let doc = format!(
            r#"Creates a new [`{name}`] from a slice of the variants of the set. Can be used in 
               const contexts."#
        );

        quote!(
                #[doc = #doc]
                pub const fn from_slice(slice: &[#base_ty]) -> Self {
                    let mut items = 0;
                    let mut index = 0;

                    while index < slice.len() {
                        items |= base_to_value(&slice[index]);
                        index += 1;
                    }

                    Self{ items }
                }
        )
    }


    fn impl_from_array(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;

        let doc = format!(
            r#"Creates a new [`{name}`] from an array of the variants of the set. Can be used in 
               const contexts."#
        );

        quote!(
                #[doc = #doc]
                pub const fn from_array<const N: usize>(array: [#base_ty; N]) -> Self {
                    let mut items = 0;
                    let mut index = 0;

                    while index < N {
                        items |= base_to_value(&array[index]);
                        index += 1;
                    }

                    Self{ items }
                }
        )
    }


    fn impl_empty(&self) -> TokenStream2 {
        let name = &self.set_type;
        let doc = format!("Creates a new empty [`{name}`].");

        quote!(
                #[doc = #doc]
                pub const fn empty() -> Self {
                    Self { items: 0 }
                }
        )
    }


    fn impl_all(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;
        let doc =
            format!("Creates a new [`{name}`] that contains all the variants of [`{base_ty}`].");

        quote!(
                #[doc = #doc]
                pub const fn all() -> Self {
                    Self { items: Self::MASK }
                }
        )
    }


    fn impl_is_empty(&self) -> TokenStream2 {
        quote!(
            /// Returns `true` if the set is empty.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.items == 0
            }
        )
    }


    fn impl_is_all(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let len = self.len();
        let doc = if len > 1 {
            format!(
                "Returns `true` if the set contains all {len} possible variants of [`{base_ty}`]."
            )
        } else {
            format!("Returns `true` if the set contains the only variant of [`{base_ty}`].")
        };

        quote!(
                #[doc = #doc]
                #[inline]
                pub const fn is_all(&self) -> bool {
                    self.items == Self::MASK
                }
        )
    }



    fn impl_len(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let doc = format!("Returns the number of variants of [`{base_ty}`] present in the set.");

        quote!(
                #[doc = #doc]
                #[inline]
                pub const fn len(&self) -> usize {
                    self.items.count_ones() as usize
                }
        )
    }


    fn impl_contains(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let doc = format!(
            r#"Returns `true` if the set contains the given variant. The variant can be specified by 
               any borrow of [`{base_ty}`]."#
        );

        quote!(
                #[doc = #doc]
                #[inline]
                pub fn contains<T: Borrow<#base_ty >>(&self, item: T) -> bool {
                    self.items & base_to_value(item.borrow()) != 0
                }
        )
    }


    fn impl_contains_const(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base_ty = &self.base_type;
        let doc = format!(
            r#"Returns `true` if the set contains the given variant. This version of 
               [`contains`]({name}::contains) can be used in const contexts, but does 
               not allow using a borrowed type of [`{base_ty}`]."#
        );

        quote!(
                #[doc = #doc]
                #[inline]
                pub fn contains_const(&self, item: &#base_ty) -> bool {
                    self.items & base_to_value(item) != 0
                }
        )
    }


    fn impl_union(&self) -> TokenStream2 {
        let name = &self.set_type;
        let doc = format!(
            r#"Creates a new [`{name}`] that contains all the variants that are in either `self` 
               or `other`, leaving both input sets unchanged."#
        );

        quote!(
                #[doc = #doc]
                #[inline]
                pub const fn union(&self, other: &Self) -> Self {
                    Self { items: self.items | other.items }
                }
        )
    }


    fn impl_intersection(&self) -> TokenStream2 {
        let name = &self.set_type;
        let doc = format!(
            r#"Creates a new [`{name}`] that contains all the variants that are in both `self` 
               and `other`, leaving both input sets unchanged."#
        );

        quote!(
                #[doc = #doc]
                #[inline]
                pub const fn intersection(&self, other: &Self) -> Self {
                    Self { items: self.items & other.items }
                }
        )
    }


    fn impl_difference(&self) -> TokenStream2 {
        let name = &self.set_type;
        let doc = format!(
            r#"Creates a new [`{name}`] that contains all the variants that are in `self` but not 
               in `other`, leaving both input sets unchanged."#
        );

        quote!(
                #[doc = #doc]
                #[inline]
                pub const fn difference(&self, other: &Self) -> Self {
                    Self { items: self.items & !other.items }
                }
        )
    }

    fn impl_symmetric_difference(&self) -> TokenStream2 {
        let name = &self.set_type;
        let doc = format!(
            r#"Creates a new [`{name}`] that contains all the variants that are in either `self` or 
               `other`, but not in both, leaving both input sets unchanged."#
        );

        quote!(
                #[doc = #doc]
                #[inline]
                pub const fn symmetric_difference(&self, other: &Self) -> Self {
                    Self { items: self.items ^ other.items }
                }
        )
    }



    fn impl_complement(&self) -> TokenStream2 {
        let doc = format!(
            "Creates a new [`{}`] that contains all the variants that are not in `self`, leaving the \
             input set unchanged.",
            self.set_type
        );

        quote!(
            #[doc = #doc]
            #[inline]
            pub const fn complement(&self) -> Self {
                Self { items: Self::MASK & !self.items }
            }
        )
    }


    fn impl_is_subset_of(&self) -> TokenStream2 {
        quote!(
            /// Returns `true` if `self` is a subset of `other`. That is, if `self` contains all
            /// the items that exist in `other`.
            #[inline]
            pub fn is_subset_of(&self, other: &Self) -> bool {
                (self.items & other.items) == self.items
            }
        )
    }

    fn impl_is_superset_of(&self) -> TokenStream2 {
        quote!(
            /// Returns `true` if `self` is a superset of `other`. That is, if `other` contains all
            /// the items that exist in `self`.
            #[inline]
            pub fn is_superset_of(&self, other: &Self) -> bool {
                (self.items & other.items) == other.items
            }
        )
    }


    fn impl_is_disjoint(&self) -> TokenStream2 {
        quote!(
            /// Returns `true` if `self` has no elements in common with `other`.
            #[inline]
            pub const fn is_disjoint(&self, other: &Self) -> bool {
                (self.items & other.items) == 0
            }
        )
    }

    fn impl_is_complementary(&self) -> TokenStream2 {
        quote!(
            /// Returns `true` if `self` and `other` are complementary sets.
            /// Two sets are complementary if their union contains all variants and their intersection is empty.
            #[inline]
            pub const fn is_complementary(&self, other: &Self) -> bool {
                self.is_disjoint(other) && (self.items | other.items) == Self::MASK
            }
        )
    }


    fn impl_insert(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let doc = format!(
            "Inserts a variant into the set. The variant can be specified by any borrow of [`{}`].",
            self.base_type
        );

        quote!(
            #[doc = #doc]
            #[inline]
            pub fn insert<T: Borrow<#base_ty >>(&mut self, item: T) {
                self.insert_const(item.borrow());
            }
        )
    }


    fn impl_remove(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let doc = format!(
            "Removes a variant from the set. The variant can be specified by any borrow of [`{}`].",
            self.base_type
        );

        quote!(
            #[doc = #doc]
            #[inline]
            pub fn remove<T: Borrow<#base_ty >>(&mut self, item: T) {
                self.remove_const(item.borrow());
            }
        )
    }

    fn impl_insert_const(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let name = &self.set_type;
        let doc = format!(
            r#"Inserts a variant into the set. This version of [`insert`]({name}::insert) can be 
               used in const contexts, but does not allow using a borrowed type of [`{}`]."#,
            self.base_type
        );

        quote!(
            #[doc = #doc]
            #[inline]
            pub const fn insert_const(&mut self, item: &#base_ty) {
                self.items |= base_to_value(item);
            }
        )
    }

    fn impl_remove_const(&self) -> TokenStream2 {
        let base_ty = &self.base_type;
        let doc = format!(
            r#"Removes a variant from the set. This version of [`remove`]({}::remove) can be used in 
               const contexts, but does not allow using a borrowed type of [`{}`]."#,
            self.set_type, self.base_type
        );

        quote!(
            #[doc = #doc]
            #[inline]
            pub const fn remove_const(&mut self, item: &#base_ty) {
                // No `Self::MASK &` necessary, since `self.items` must have invalid bits zeroed
                self.items &= !base_to_value(item);
            }
        )
    }

    fn impl_iter_inherent(&self) -> TokenStream2 {
        let iter = &self.iter_type;
        let doc = format!(
            r#"Returns an iterator over the variants of [`{}`] contained in the set. Variants are 
               yielded in the order in which they appear in the definition of [`{}`], regardless 
               of insertion order."#,
            self.base_type, self.base_type
        );

        quote!(
            #[doc = #doc]
            #[inline]
            pub const fn iter(&self) -> #iter {
                #iter { items: self.items }
            }
        )
    }


    fn impl_to_repr(&self) -> TokenStream2 {
        let inner_ty = &self.inner_type;
        let base_ty = &self.base_type;
        let doc = format!(
            r#"Returns the integer representation of the set as a bitset. The N-th variant of 
               [`{base_ty}`] corresponds to the N-th least significative bit of the integer."#
        );

        quote!(
            #[doc = #doc]
            pub const fn to_repr(&self) -> #inner_ty {
                self.items
            }
        )
    }

    fn impl_from_repr(&self) -> TokenStream2 {
        let name = &self.set_type;
        let inner_ty = &self.inner_type;
        let base_ty = &self.base_type;
        let doc1 = format!(
            r#"Creates a new [`{name}`] from an integer interpreted as a bitset. The N-th variant 
               of [`{base_ty}`] corresponds to the N-th least significative bit of the integer."#
        );
        let doc2 = format!(
            r#"Returns `None` if the integer is not a valid bitset representation of the set 
               (i.e, a bit that does not correspond to any variant of [`{base_ty}`] is set to 1.)."#
        );

        quote!(
            #[doc = #doc1]
            ///
            #[doc = #doc2]
            pub const fn from_repr(repr: #inner_ty) -> Option<Self> {
                let items = repr & Self::MASK;
                if items == repr {
                    Some(Self { items : items & Self::MASK })
                } else {
                    None
                }
            }
        )
    }

    fn impl_is_valid_repr(&self) -> TokenStream2 {
        let name = &self.set_type;
        let inner_ty = &self.inner_type;
        let doc = format!(
            "Checks whether the provided value is a valid representation for a [`{name}`]."
        );

        quote!(
            #[doc = #doc]
            pub const fn is_valid_repr(repr: #inner_ty) -> bool {
                (repr & Self::MASK) == repr
            }
        )
    }

    fn impl_from_repr_unchecked(&self) -> TokenStream2 {
        let name = &self.set_type;
        let inner_ty = &self.inner_type;
        let base_ty = &self.base_type;
        let doc1 = format!(
            r#"Creates a new [`{name}`] from an integer interpreted as a bitset. The N-th variant 
               of [`{base_ty}`] corresponds to the N-th least significative bit of the integer."#
        );
        let doc2 = format!(
            r#"The integer must be a valid bitset representation of the set, but the invariant that 
               bits that do not correspond to a variant of [`{base_ty}`] is not checked, so must be 
               guaranteed by the calling code."#
        );

        let doc3 = format!(
            r#"Note the [`from_repr_masked`]({base_ty}::from_repr_masked) method that silently
               discards invalid bits at the cost of a (cheap) single bit-wise and operation."#
        );

        let doc4 = format!(
            r#"Also the [`from_repr_discarded`]({base_ty}::from_repr_discarded) method that
               returns the discarded bits alongide the new set instance (where those bits are
               safely masked out). This can be useful as an error hanlding mechanism."#
        );

        quote!(
            #[doc = #doc1]
            ///
            /// # Safety
            #[doc = #doc2]
            ///
            /// It is undefined behavior to pass an invalid value. Methods and iterators obtained
            /// from the generated set might exhibit incorrect behavior, including possible panics.
            ///
            /// # Safe alternatives
            #[doc = #doc3]
            ///
            #[doc = #doc4]
            pub const unsafe fn from_repr_unchecked(repr: #inner_ty) -> Self {
                Self { items : repr }
            }
        )
    }

    fn impl_from_repr_masked(&self) -> TokenStream2 {
        let name = &self.set_type;
        let inner_ty = &self.inner_type;
        let base_ty = &self.base_type;
        let doc1 = format!(
            r#"Creates a new [`{name}`] from an integer interpreted as a bitset, silently discarding invalid bits.
               The N-th variant of [`{base_ty}`] corresponds to the N-th least significative bit of the integer."#
        );
        quote!(
            #[doc = #doc1]
            ///
            /// If the provided `repr` has bits that do not map to a variant of the original enum
            /// set to one, they are masked out and ignored to uphold the bitset integrity
            /// invariant.
            pub const unsafe fn from_repr_masked(repr: #inner_ty) -> Self {
                Self { items : repr & Self::MASK }
            }
        )
    }

    fn impl_from_repr_discarded(&self) -> TokenStream2 {
        let name = &self.set_type;
        let inner_ty = &self.inner_type;
        let base_ty = &self.base_type;
        let doc1 = format!(
            r#"Creates a new [`{name}`] from an integer interpreted as a bitset, discarding invalid bits.
               Returns the created set and an integer where only the discarded bits are set to 1.
               The N-th variant of [`{base_ty}`] corresponds to the N-th least significative bit of the integer."#
        );
        quote!(
            #[doc = #doc1]
            ///
            /// If the provided `repr` has bits that do not map to a variant of the original enum
            /// set to one, they are masked out to uphold the bitset integrity invariant.
            /// The discarded bits that were set are returned as the second member of the tuple.
            pub const unsafe fn from_repr_discarded(repr: #inner_ty) -> (Self, #inner_ty) {
                (Self { items : repr & Self::MASK }, repr & !Self::MASK)
            }
        )
    }


    fn impl_collect(&self) -> TokenStream2 {
        let base_ty = &self.base_type;

        let doc1 = format!(
            "Collects the values present in the set into any value that can be formed by an iterator of [`{base_ty}`]."
        );
        let doc2 = format!("That is, any value that implements `FromIterator<`{base_ty}`>.");

        quote! {
            #[doc = #doc1]
            #[doc = #doc2]
            pub fn collect<B: FromIterator<#base_ty>>(&self) -> B {
                self.iter().collect()
            }
        }
    }
}

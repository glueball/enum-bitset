use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{LitStr, Path, parse_str};

use crate::derive::config::EnumBitsetConfig;

pub struct SerdeConfig {
    pub ser: bool,
    pub de: bool,
    pub serde_crate: Path,
}

impl Default for SerdeConfig {
    fn default() -> Self {
        SerdeConfig {
            ser: true,
            de: true,
            serde_crate: parse_str("::serde").expect("::serde is a valid path"),
        }
    }
}

#[allow(dead_code)]
impl EnumBitsetConfig {
    #[cfg(feature = "serde")]
    pub fn impl_serde(&self) -> TokenStream2 {
        self.impl_serde_list()
    }

    #[cfg(not(feature = "serde"))]
    pub fn impl_serde(&self) -> TokenStream2 {
        TokenStream2::new()
    }

    #[allow(dead_code)]
    fn impl_serde_list(&self) -> TokenStream2 {
        let mut output = TokenStream2::new();

        if self.serde.ser {
            output.extend(self.impl_serde_list_ser());
        }

        if self.serde.de {
            output.extend(self.impl_serde_list_de());
        }

        output
    }

    #[allow(dead_code)]
    fn impl_serde_list_ser(&self) -> TokenStream2 {
        let name = &self.set_type;
        let serde = &self.serde.serde_crate;

        quote! {
            use #serde::{Serialize, Serializer, ser::{SerializeSeq}};

            const _: fn() = || {
                fn base_enum_for_bitsets_must_be_serializable<T: Serialize>() {}
                base_enum_for_bitsets_must_be_serializable::<#name>();
            };

            impl Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let mut seq = serializer.serialize_seq(Some(self.len()))?;

                    for variant in self.iter() {
                        seq.serialize_element(&variant)?;
                    }

                    seq.end()
                }
            }
        }
    }

    #[allow(dead_code)]
    fn impl_serde_list_de(&self) -> TokenStream2 {
        let name = &self.set_type;
        let base = &self.base_type;
        let serde = &self.serde.serde_crate;

        let expect = LitStr::new(&format!("a list {name} of variants"), name.span());

        quote! {
            use #serde::{Deserialize, Deserializer, de::{SeqAccess, Visitor}};

            const _: fn() = || {
                fn base_enum_for_bitsets_must_be_deserializable<T: for<'de> Deserialize<'de>>() {}
                base_enum_for_bitsets_must_be_deserializable::<#name>();
            };

            impl<'de> Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct VisitList;

                    impl<'de> Visitor<'de> for VisitList {
                        type Value = #name;

                        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                            formatter.write_str(#expect)
                        }

                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                        where
                            A: serde::de::SeqAccess<'de>,
                        {
                            let mut result = #name::new();
                            while let Some(variant) = seq.next_element::<#base>()? {
                                result.insert(variant);
                            }

                            Ok(result)
                        }
                    }

                    deserializer.deserialize_seq(VisitList)
                }
            }
        }
    }
}

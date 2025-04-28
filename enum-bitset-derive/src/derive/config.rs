use std::convert::TryFrom;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DeriveInput, Error, Fields, Lit, Path, Result, Variant, Visibility,
    meta::ParseNestedMeta, parse_str, spanned::Spanned,
};

use crate::derive::serde::SerdeConfig;

mod vis;

pub struct EnumBitsetConfig {
    pub base_type: Ident,
    pub set_type: Ident,
    pub inner_type: Ident,
    pub iter_type: Ident,
    pub debug: bool,
    pub variants: Vec<Variant>,
    pub base_vis: Visibility,
    pub inner_vis: Visibility,
    pub base_add: bool,
    pub my_crate: Path,
    pub serde: SerdeConfig,
}

static INVALID_SERDE_MSG: &str = "Invalid value for serde. Valid values are: `true`, `false`, \"de\", \"ser\", \"both\" (same as `true`), and \"none\" (same as `false`).";
static INVALID_ATTR_MSG: &str =
    "Invalid attribute value. Valid values are: `serde`, `serde_crate`, `no_debug`, and `crate`.";
static ONLY_ENUM_MSG: &str = "EnumBitset can only be derived for enums";
static NO_VARIANTS: &str = "EnumBitset cannot be derived for enums with no variants";
static ALL_UNIT_MSG: &str = "EnumBitset can only be derived for enums with all unit variants.";
static TOO_MANY_VARIANTS_MSG: &str = "Too many variants! At most 128 are supported.";
const INVALID_REPR_MSG: &str = "Invalid bitset representation: must be a primitive unsigned integer (u8, u16, u32, u64, u128).";



impl TryFrom<DeriveInput> for EnumBitsetConfig {
    type Error = Error;

    fn try_from(input: DeriveInput) -> Result<Self> {
        let data = match input.data {
            Data::Enum(data) => data,
            _ => return Err(Error::new(input.span(), ONLY_ENUM_MSG)),
        };

        if data.variants.is_empty() {
            return Err(Error::new(data.variants.span(), NO_VARIANTS));
        }

        let set_type = format_ident!("{}Set", input.ident);
        let mut config = Self {
            iter_type: format_ident!("{set_type}SetIter"),
            set_type,
            inner_type: Self::inner_type(data.variants.len())?,
            variants: Self::parse_variants(data.variants)?,
            inner_vis: vis::compute_visibility(&input.vis),
            base_vis: input.vis,
            base_type: input.ident,
            my_crate: parse_str("::enum_bitset")?,
            serde: SerdeConfig::default(),
            debug: true,
            base_add: true,
        };

        config.parse_attrs(input.attrs)?;

        Ok(config)
    }
}


impl EnumBitsetConfig {
    pub(crate) fn len(&self) -> usize {
        self.variants.len()
    }

    pub(crate) fn base_to_value_branches(&self) -> impl Iterator<Item = TokenStream2> + Clone + '_ {
        self.variants
            .iter()
            .enumerate()
            .map(move |(index, variant)| {
                let name = &variant.ident;
                let base = &self.base_type;

                quote! {#base::#name => const { 1 << #index }}
            })
    }

    fn parse_variants(variants: impl IntoIterator<Item = Variant>) -> Result<Vec<Variant>> {
        variants
            .into_iter()
            .map(|variant| match variant.fields {
                Fields::Unit => Ok(variant),
                _ => Err(Error::new(variant.span(), ALL_UNIT_MSG)),
            })
            .collect()
    }

    fn parse_attrs(&mut self, attrs: Vec<Attribute>) -> Result<()> {
        attrs
            .into_iter()
            .filter(|attr| attr.path().is_ident("bitset"))
            .try_for_each(|attr| -> Result<()> {
                attr.parse_nested_meta(|meta| self.parse_attr(meta))
            })
    }

    fn parse_attr(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("name") {
            self.set_type = meta.value()?.parse()?;
            self.iter_type = format_ident!("{}SetIter", self.set_type);
            return Ok(());
        };

        if meta.path.is_ident("serde") {
            return self.parse_serde_attr(&meta);
        };

        if meta.path.is_ident("serde_crate") {
            self.serde.serde_crate = meta.value()?.parse()?;
            return Ok(());
        }

        if meta.path.is_ident("no_debug") {
            self.debug = false;
            return Ok(());
        }

        if meta.path.is_ident("crate") {
            self.my_crate = meta.value()?.parse()?;
            return Ok(());
        }

        if meta.path.is_ident("repr") {
            self.inner_type = self.parse_repr_attr(&meta)?;
            return Ok(());
        }

        if meta.path.is_ident("no_base_ops") {
            self.base_add = false;
            return Ok(());
        }

        Err(Error::new(meta.input.span(), INVALID_ATTR_MSG))
    }

    fn parse_serde_attr(&mut self, meta: &ParseNestedMeta) -> Result<()> {
        if meta.input.is_empty() {
            self.serde.de = true;
            self.serde.ser = true;
            return Ok(());
        }

        match meta.value()?.parse::<Lit>()? {
            Lit::Bool(value) => {
                self.serde.de = value.value;
                self.serde.ser = value.value;
                Ok(())
            }
            Lit::Str(value) => match value.value().as_str() {
                "de" | "deserialize" => {
                    self.serde.de = true;
                    self.serde.ser = false;
                    Ok(())
                }
                "ser" | "serialize" => {
                    self.serde.ser = true;
                    self.serde.de = false;
                    Ok(())
                }
                "both" => {
                    self.serde.de = true;
                    self.serde.ser = true;
                    Ok(())
                }
                "none" => {
                    self.serde.de = false;
                    self.serde.ser = false;
                    Ok(())
                }
                _ => Err(Error::new(meta.input.span(), INVALID_SERDE_MSG)),
            },
            _ => Err(Error::new(meta.input.span(), INVALID_SERDE_MSG)),
        }
    }

    fn inner_type(n_variants: usize) -> Result<Ident> {
        let len = if n_variants <= 8 {
            8
        } else if n_variants <= 16 {
            16
        } else if n_variants <= 32 {
            32
        } else if n_variants <= 64 {
            64
        } else if n_variants <= 128 {
            128
        } else {
            return Err(Error::new(Span::call_site(), TOO_MANY_VARIANTS_MSG));
        };

        Ok(format_ident!("u{len}"))
    }

    fn parse_repr_attr(&mut self, meta: &ParseNestedMeta) -> Result<Ident> {
        let ty: Path = meta
            .value()?
            .parse()
            .map_err(|_| meta.error(INVALID_REPR_MSG))?;
        let ty = ty.require_ident()?.to_string();

        if !ty.starts_with("u") {
            return Err(meta.error(INVALID_REPR_MSG));
        }

        let n: usize = ty[1..]
            .parse()
            .map_err(|_| meta.error(INVALID_REPR_MSG))?;

        if ![8, 16, 32, 64, 128].contains(&n) {
            return Err(meta.error(INVALID_REPR_MSG));
        }

        if n < self.variants.len() {
            return Err(meta.error(format!(
                "Invalid bitset representation: {} has {} variants, but the requested bitset representation is only {n} bits wide.",
                self.base_type,
                self.variants.len(),
            )));
        }

        Ok(format_ident!("u{n}"))
    }
}

use syn::LitInt;

use crate::derive::config::EnumBitsetConfig;

impl EnumBitsetConfig {
    // Static for easier testing
    pub(crate) fn mask(len: usize) -> LitInt {
        let ones = "1".repeat(len);
        let bin = format!("0b{ones}");
        LitInt::new(&bin, proc_macro2::Span::call_site())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_3() {
        let mask = EnumBitsetConfig::mask(3);
        assert_eq!(mask.to_string(), "0b111");
    }

    #[test]
    fn test_mask_10() {
        let mask = EnumBitsetConfig::mask(10);
        assert_eq!(mask.to_string(), "0b1111111111");
    }

    #[test]
    #[should_panic]
    fn test_mask_0() {
        let mask = EnumBitsetConfig::mask(0);
        assert_eq!(mask.to_string(), "0b");
    }
}

//! Example to demonstrate the `EnumBitset` derive macro.
//!
use enum_bitset_derive::EnumBitset;

/// Example enum to demonstrate the `EnumBitset` derive macro.
#[derive(EnumBitset, Clone, Copy, PartialEq)]
#[bitset(crate = crate)]
pub enum ProgrammerState {
    Awake,
    CoffeeAcquired,
    CompilerFight,
    MergeConflict,
    ZoneAchieved,
    BuildingCastlesInTheCloud,
    TimeToLeave,
    SideProject,
    Sleeping,
}


#[cfg(all(feature = "serde", any(doc, test)))]
mod serde_impl {
    // "Implement" serde traits manually, so we don't need to add the `derive` feature in our serde dependence to compile docs.
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::ProgrammerState;


    impl Serialize for ProgrammerState {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            unimplemented!()
        }
    }


    #[automatically_derived]
    impl<'de> Deserialize<'de> for ProgrammerState {
        fn deserialize<D>(_: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            unimplemented!()
        }
    }
}

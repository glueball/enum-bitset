#![cfg(feature = "serde")]

use std::iter::FromIterator;

use assert2::check;
use enum_bitset::EnumBitset;
use serde::{Deserialize, Serialize};
use serde_json::from_str;



#[derive(EnumBitset, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[bitset(serde = "de", serde_crate = serde)]
enum SerdeConfigTest {
    A,
    B,
    C,
}



#[test]
fn deserialize_empty() {
    let json = "[]";
    let empty = SerdeConfigTestSet::new();
    check!(from_str::<SerdeConfigTestSet>(json).unwrap() == empty);
}

#[test]
fn deserialize_all() {
    let json = "[\"A\",\"B\",\"C\"]";
    let all = SerdeConfigTestSet::all();
    check!(from_str::<SerdeConfigTestSet>(json).unwrap() == all);
}

#[test]
fn deserialize_a() {
    let json = "[\"A\"]";
    let a = SerdeConfigTestSet::from_iter([SerdeConfigTest::A]);
    check!(from_str::<SerdeConfigTestSet>(json).unwrap() == a);
}


#[test]
fn assert_no_serialize() {
    // Adapted from the "impls" crate: https://github.com/nvzqz/impls/blob/master/src/lib.rs
    trait DoesNotImpl {
        const IMPLS: bool = false;
    }

    impl<T: ?Sized> DoesNotImpl for T {}

    struct Wrapper<T: ?Sized>(T);

    impl<T: ?Sized + Serialize> Wrapper<T> {
        const IMPLS: bool = true;
    }

    check!(Wrapper::<SerdeConfigTest>::IMPLS == true);
    check!(Wrapper::<SerdeConfigTestSet>::IMPLS == false);
}

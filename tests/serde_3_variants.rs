#![cfg(feature = "serde")]

use core::iter::FromIterator;

use assert2::check;
use enum_bitset::EnumBitset;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};


#[derive(EnumBitset, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum TreeVariants {
    A,
    B,
    C,
}


#[test]
fn serialize_empty() {
    let json = "[]";
    let empty = TreeVariantsSet::new();
    check!(to_string(&empty).unwrap() == json);
}

#[test]
fn deserialize_empty() {
    let json = "[]";
    let empty = TreeVariantsSet::new();
    check!(from_str::<TreeVariantsSet>(json).unwrap() == empty);
}

#[test]
fn serialize_all() {
    let json = "[\"A\",\"B\",\"C\"]";
    let all = TreeVariantsSet::all();
    check!(to_string(&all).unwrap() == json);
}

#[test]
fn deserialize_all() {
    let json = "[\"A\",\"B\",\"C\"]";
    let all = TreeVariantsSet::all();
    check!(from_str::<TreeVariantsSet>(json).unwrap() == all);
}

#[test]
fn deserialize_a() {
    let json = "[\"A\"]";
    let a = TreeVariantsSet::from_iter([TreeVariants::A]);
    check!(from_str::<TreeVariantsSet>(json).unwrap() == a);
}

#[test]
fn serialize_a() {
    let json = "[\"A\"]";
    let a = TreeVariantsSet::from_iter([TreeVariants::A]);
    check!(to_string(&a).unwrap() == json);
}

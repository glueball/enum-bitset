use core::fmt::Debug;

use assert2::check;
use enum_bitset::EnumBitset;



// Adapted from the "impls" crate: https://github.com/nvzqz/impls/blob/master/src/lib.rs
trait DoesNotImpl {
    const IMPLS: bool = false;
}

impl<T: ?Sized> DoesNotImpl for T {}

struct Wrapper<T: ?Sized>(T);

impl<T: ?Sized + Debug> Wrapper<T> {
    const IMPLS: bool = true;
}


#[derive(EnumBitset, Clone, Debug)]
#[bitset(serde = false, no_debug)]
pub enum WithDebug {
    A,
    B,
}

#[test]
fn assert_no_debug() {
    check!(Wrapper::<WithDebug>::IMPLS == true);
    check!(Wrapper::<WithDebugSet>::IMPLS == false);
}



#[derive(EnumBitset, Clone)]
#[bitset(serde = false)]
enum WithoutDebug {
    A,
    B,
}

#[test]
fn assert_no_debug_set() {
    check!(Wrapper::<WithoutDebug>::IMPLS == false);
    check!(Wrapper::<WithoutDebugSet>::IMPLS == true);
}

#[test]
fn custom_debug_impl_empty() {
    let set = WithoutDebugSet::empty();
    let dbg = format!("{set:?}");
    assert_eq!(dbg, "WithoutDebugSet(0){}");
}

#[test]
fn custom_debug_impl_2_items() {
    let set = WithoutDebugSet::from([WithoutDebug::A, WithoutDebug::B]);
    let dbg = format!("{set:?}");
    assert_eq!(dbg, "WithoutDebugSet(2){/* 2 items */}");
}

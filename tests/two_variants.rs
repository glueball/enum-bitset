use assert2::check;
use enum_bitset::EnumBitset;

#[derive(EnumBitset, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum TwoVariants {
    A,
    B,
}

#[test]
fn default_is_empty() {
    let default = TwoVariantsSet::default();
    assert!(default.is_empty());
}

#[test]
fn mask() {
    check!(TwoVariantsSet::MASK == 3);
}

#[test]
fn variants() {
    check!(TwoVariantsSet::VARIANTS == [TwoVariants::A, TwoVariants::B]);
}

#[test]
fn insert_a() {
    let mut set = TwoVariantsSet::new();
    set.insert(TwoVariants::A);

    check!(set == TwoVariants::A.into());
    check!(set.len() == 1);

    assert!(set.contains(TwoVariants::A));
    assert!(set.contains(&TwoVariants::A));
    assert!(!set.contains(TwoVariants::B));
    assert!(!set.contains(&TwoVariants::B));
}


#[test]
fn iter_empty() {
    let set = TwoVariantsSet::new();

    check!(set.iter().next().is_none());
}


#[test]
fn iter_b() {
    let mut set = TwoVariantsSet::new();
    set.insert(TwoVariants::B);

    let mut iter = set.iter();
    check!(iter.next() == Some(TwoVariants::B));
    check!(iter.next().is_none());
}


#[test]
fn iter_both() {
    let mut set = TwoVariantsSet::new();
    set.insert(TwoVariants::B);
    set.insert(TwoVariants::A);

    let mut iter = set.iter();
    check!(iter.next() == Some(TwoVariants::A));
    check!(iter.next() == Some(TwoVariants::B));
    check!(iter.next().is_none());
}


#[test]
fn debug_empty() {
    let set = TwoVariantsSet::new();
    check!(format!("{:?}", set) == "TwoVariantsSet(0){}");
}


#[test]
fn debug_a() {
    let set: TwoVariantsSet = TwoVariants::A.into();
    check!(format!("{:?}", set) == "TwoVariantsSet(1){A}");
}


#[test]
fn debug_ab() {
    let set = TwoVariantsSet::from([TwoVariants::A, TwoVariants::B]);
    check!(format!("{:?}", set) == "TwoVariantsSet(2){A, B}");
}



#[test]
fn debug_ab_pretty() {
    let set = TwoVariantsSet::from([TwoVariants::A, TwoVariants::B]);

    let expected = r#"TwoVariantsSet (2) {
    A,
    B,
}"#;

    check!(format!("{:#?}", set) == expected);
}

#[test]
fn debug_empty_pretty() {
    let set = TwoVariantsSet::empty();

    let expected = r#"TwoVariantsSet (0) {}"#;

    check!(format!("{:#?}", set) == expected);
}

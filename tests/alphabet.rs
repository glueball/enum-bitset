use assert2::check;
use enum_bitset::EnumBitset;

mod no_debug;

#[derive(EnumBitset, Hash, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}


#[test]
fn default_is_empty() {
    let default = AlphabetSet::default();
    assert!(default.is_empty());
}

#[test]
fn alphabet_set() {
    let mut set = AlphabetSet::new();
    set.insert(Alphabet::A);

    check!(set == Alphabet::A.into());
    check!(set.len() == 1);
    check!(set.contains(Alphabet::A));
    check!(set.contains(&Alphabet::A));
    check!(!set.contains(Alphabet::B));
    check!(!set.contains(&Alphabet::B));
}


#[test]
fn alphabet_mask() {
    check!(AlphabetSet::MASK == 0b11111111111111111111111111);
}

#[test]
fn iter_empty() {
    let set = AlphabetSet::new();

    check!(set.iter().next().is_none());
}


#[test]
fn iter_b() {
    let mut set = AlphabetSet::new();
    set.insert(Alphabet::B);

    let mut iter = set.iter();
    check!(iter.next() == Some(Alphabet::B));
    check!(iter.next().is_none());
}


static VOWELS: AlphabetSet = AlphabetSet::from_array([
    Alphabet::A,
    Alphabet::E,
    Alphabet::I,
    Alphabet::O,
    Alphabet::U,
]);

#[test]
fn vowels_len() {
    check!(VOWELS.len() == 5);
}

#[test]
fn vowels_iter() {
    let mut iter = VOWELS.iter();
    check!(iter.next() == Some(Alphabet::A));
    check!(iter.next() == Some(Alphabet::E));
    check!(iter.next() == Some(Alphabet::I));
    check!(iter.next() == Some(Alphabet::O));
    check!(iter.next() == Some(Alphabet::U));
    check!(iter.next().is_none());
}


#[test]
fn vowels_debug() {
    let expected = "AlphabetSet(5){A, E, I, O, U}";
    check!(format!("{:?}", VOWELS) == expected);
}


#[test]
fn vowels_collect_to_hash_set() {
    use std::collections::HashSet;

    let hash_set = VOWELS.iter().collect::<HashSet<_>>();
    check!(hash_set.len() == 5);

    let expected = HashSet::from([
        Alphabet::A,
        Alphabet::E,
        Alphabet::I,
        Alphabet::O,
        Alphabet::U,
    ]);
    check!(expected == hash_set);
}

#[test]
fn z() {
    let set = AlphabetSet::from([Alphabet::Z]);
    check!(set.len() == 1);

    let mut iter = set.iter();
    check!(iter.next() == Some(Alphabet::Z));
    check!(iter.next().is_none());
}

#[test]
fn vowels_from_iter_of_refs() {
    let set = AlphabetSet::from_iter([
        &Alphabet::A,
        &Alphabet::E,
        &Alphabet::I,
        &Alphabet::O,
        &Alphabet::U,
    ]);
    check!(set.len() == 5);
    check!(set.contains(Alphabet::A));
    check!(set.contains(Alphabet::E));
    check!(set.contains(Alphabet::I));
    check!(set.contains(Alphabet::O));
    check!(set.contains(Alphabet::U));
    check!(!set.contains(Alphabet::B));
    check!(set == VOWELS);
}

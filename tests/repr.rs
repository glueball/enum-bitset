use assert2::check;
use enum_bitset::EnumBitset;

#[derive(EnumBitset, Debug, Copy, Clone)]
#[bitset(serde = false, repr = u16)]
enum CodeComment {
    TodoSinceFirstCommit,
    BlackMagic,
    LateNightRambling,
}


#[test]
fn layout() {
    use std::alloc::Layout;

    assert_eq!(Layout::new::<CodeCommentSet>(), Layout::new::<u16>());
}

#[test]
fn from_repr() {
    check!(CodeCommentSet::from_repr(0).unwrap() == CodeCommentSet::empty());
    check!(
        CodeCommentSet::from_repr(1).unwrap()
            == CodeCommentSet::from([CodeComment::TodoSinceFirstCommit])
    );
    check!(
        CodeCommentSet::from_repr(2).unwrap() == CodeCommentSet::from([CodeComment::BlackMagic])
    );
    check!(
        CodeCommentSet::from_repr(3).unwrap()
            == CodeCommentSet::from([CodeComment::TodoSinceFirstCommit, CodeComment::BlackMagic])
    );
    check!(
        CodeCommentSet::from_repr(4).unwrap()
            == CodeCommentSet::from([CodeComment::LateNightRambling])
    );
    check!(
        CodeCommentSet::from_repr(5).unwrap()
            == CodeCommentSet::from([
                CodeComment::TodoSinceFirstCommit,
                CodeComment::LateNightRambling
            ])
    );
    check!(
        CodeCommentSet::from_repr(6).unwrap()
            == CodeCommentSet::from([CodeComment::LateNightRambling, CodeComment::BlackMagic])
    );
    check!(
        CodeCommentSet::from_repr(7).unwrap()
            == CodeCommentSet::from([
                CodeComment::TodoSinceFirstCommit,
                CodeComment::LateNightRambling,
                CodeComment::BlackMagic
            ])
    );
    check!(CodeCommentSet::from_repr(8) == None);
    check!(CodeCommentSet::from_repr(9) == None);
    check!(CodeCommentSet::from_repr(10000) == None);
}

#[test]
fn from_is_valid_repr() {
    check!(CodeCommentSet::is_valid_repr(0));
    check!(CodeCommentSet::is_valid_repr(1));
    check!(CodeCommentSet::is_valid_repr(2));
    check!(CodeCommentSet::is_valid_repr(3));
    check!(CodeCommentSet::is_valid_repr(4));
    check!(CodeCommentSet::is_valid_repr(5));
    check!(CodeCommentSet::is_valid_repr(6));
    check!(CodeCommentSet::is_valid_repr(7));
    check!(!CodeCommentSet::is_valid_repr(8));
    check!(!CodeCommentSet::is_valid_repr(9));
    check!(!CodeCommentSet::is_valid_repr(10000));
}

#[test]
fn from_from_repr_unchecked() {
    let set = unsafe { CodeCommentSet::from_repr_unchecked(256) };

    check!(!set.contains(CodeComment::TodoSinceFirstCommit));
    check!(!set.contains(CodeComment::BlackMagic));
    check!(!set.contains(CodeComment::LateNightRambling));

    // This is UB, so results might not be what you expect. No variant, but not empty ?!
    check!(set.len() == 1);
    check!(!set.is_empty());
}

#[test]
fn to_repr()
{
    check!(CodeCommentSet::empty().to_repr() == 0);
    check!(CodeCommentSet::from([CodeComment::TodoSinceFirstCommit]).to_repr() == 1);
    check!(CodeCommentSet::from([CodeComment::BlackMagic]).to_repr() == 2);
    check!(CodeCommentSet::from([CodeComment::TodoSinceFirstCommit, CodeComment::BlackMagic]).to_repr() == 3);
    check!(CodeCommentSet::from([CodeComment::LateNightRambling]).to_repr() == 4);
    check!(CodeCommentSet::from([CodeComment::TodoSinceFirstCommit, CodeComment::LateNightRambling]).to_repr() == 5);
    check!(CodeCommentSet::from([CodeComment::LateNightRambling, CodeComment::BlackMagic]).to_repr() == 6);
    check!(CodeCommentSet::from([CodeComment::TodoSinceFirstCommit, CodeComment::LateNightRambling, CodeComment::BlackMagic]).to_repr() == 7);
}

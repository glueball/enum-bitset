use assert2::check;
use enum_bitset_derive::EnumBitset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, EnumBitset, Clone, Copy, Debug, PartialEq, Eq)]
enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl MonthSet {
    const SHORT_MONTHS: MonthSet =
        MonthSet::from_array([Month::Feb, Month::Apr, Month::Jun, Month::Sep, Month::Nov]);

    const LONG_MONTHS: MonthSet = MonthSet::from_array([
        Month::Jan,
        Month::Mar,
        Month::May,
        Month::Jul,
        Month::Aug,
        Month::Oct,
        Month::Dec,
    ]);

    const ODD_MONTHS: MonthSet = MonthSet::from_array([
        Month::Jan,
        Month::Mar,
        Month::May,
        Month::Jul,
        Month::Sep,
        Month::Nov,
    ]);
    const EVEN_MONTHS: MonthSet = MonthSet::from_array([
        Month::Feb,
        Month::Apr,
        Month::Jun,
        Month::Aug,
        Month::Oct,
        Month::Dec,
    ]);
}

#[test]
fn with_add_short() {
    let short = Month::Feb + Month::Apr + Month::Jun + Month::Sep + Month::Nov;
    check!(short == MonthSet::SHORT_MONTHS);
}

#[test]
fn with_add_odd() {
    let odd = Month::Jan + Month::Mar + Month::May + Month::Jul + Month::Sep + Month::Nov;
    check!(odd == MonthSet::ODD_MONTHS);
}

#[test]
fn with_add_even() {
    let even = Month::Feb + Month::Apr + Month::Jun + Month::Aug + Month::Oct + Month::Dec;
    check!(even == MonthSet::EVEN_MONTHS);
}

#[test]
fn with_add_long() {
    let long =
        Month::Jan + Month::Mar + Month::May + Month::Jul + Month::Aug + Month::Oct + Month::Dec;
    check!(long == MonthSet::LONG_MONTHS);
}

#[test]
fn with_or_short() {
    let short = Month::Feb | Month::Apr | Month::Jun | Month::Sep | Month::Nov;
    check!(short == MonthSet::SHORT_MONTHS);
}

#[test]
fn with_or_long() {
    let long =
        Month::Jan | Month::Mar | Month::May | Month::Jul | Month::Aug | Month::Oct | Month::Dec;
    check!(long == MonthSet::LONG_MONTHS);
}

#[test]
fn with_or_odd() {
    let odd = Month::Jan | Month::Mar | Month::May | Month::Jul | Month::Sep | Month::Nov;
    check!(odd == MonthSet::ODD_MONTHS);
}

#[test]
fn with_or_even() {
    let even = Month::Feb | Month::Apr | Month::Jun | Month::Aug | Month::Oct | Month::Dec;
    check!(even == MonthSet::EVEN_MONTHS);
}

#[test]
fn add_to_all() {
    let all = MonthSet::SHORT_MONTHS + MonthSet::LONG_MONTHS;
    check!(all == MonthSet::all());

    let all = MonthSet::EVEN_MONTHS + MonthSet::ODD_MONTHS;
    check!(all == MonthSet::all());
}

#[test]
fn or_to_all() {
    let all = MonthSet::SHORT_MONTHS | MonthSet::LONG_MONTHS;
    check!(all == MonthSet::all());

    let all = MonthSet::EVEN_MONTHS | MonthSet::ODD_MONTHS;
    check!(all == MonthSet::all());
}

#[test]
fn sub_remove_1() {
    let set = MonthSet::SHORT_MONTHS - Month::Feb;
    check!(set.len() == 4);
    check!(set == MonthSet::from_array([Month::Apr, Month::Jun, Month::Sep, Month::Nov]));
}

#[test]
fn sub_remove_2() {
    let set = MonthSet::SHORT_MONTHS - Month::Apr - Month::Sep;
    check!(set.len() == 3);
    check!(set == MonthSet::from_array([Month::Feb, Month::Jun, Month::Nov]));
}

#[test]
fn sub_unexisting() {
    let set = MonthSet::SHORT_MONTHS - Month::Aug;
    check!(set.len() == 5);
    check!(set == MonthSet::SHORT_MONTHS);
}

#[test]
fn add_existing() {
    let set = MonthSet::SHORT_MONTHS + Month::Feb;
    check!(set.len() == 5);
    check!(set == MonthSet::SHORT_MONTHS);
}

#[test]
fn add_unexisting() {
    let set = MonthSet::SHORT_MONTHS + Month::Aug;
    check!(set.len() == 6);
    check!(
        set == MonthSet::from_array([
            Month::Feb,
            Month::Apr,
            Month::Jun,
            Month::Sep,
            Month::Nov,
            Month::Aug
        ])
    );
}

#[test]
fn symmetric_diff() {
    let set = MonthSet::SHORT_MONTHS ^ MonthSet::LONG_MONTHS;
    check!(set.is_all());
}

#[test]
fn symmetric_diff_empty() {
    let set = MonthSet::SHORT_MONTHS ^ MonthSet::SHORT_MONTHS;
    check!(set.is_empty());
}

#[test]
fn short_but_not_odd() {
    let set = MonthSet::SHORT_MONTHS - MonthSet::ODD_MONTHS;
    check!(set == MonthSet::from([Month::Feb, Month::Apr, Month::Jun,]));
}

#[test]
fn short_or_odd_not_both() {
    let set = MonthSet::SHORT_MONTHS ^ MonthSet::ODD_MONTHS;
    check!(
        set == MonthSet::from([
            Month::Jan,
            Month::Feb,
            Month::Mar,
            Month::Apr,
            Month::May,
            Month::Jun,
            Month::Jul,
        ])
    );
}
#[test]
fn short_and_odd() {
    let set = MonthSet::SHORT_MONTHS & MonthSet::ODD_MONTHS;
    check!(set == MonthSet::from([Month::Sep, Month::Nov]));
}

#[test]
fn and_empty() {
    let set = MonthSet::SHORT_MONTHS & MonthSet::LONG_MONTHS;
    check!(set.is_empty());
}

#[test]
fn ops_with_empty() {
    let empty = MonthSet::empty();
    check!((empty | MonthSet::SHORT_MONTHS) == MonthSet::SHORT_MONTHS);
    check!((empty & MonthSet::SHORT_MONTHS).is_empty());
    check!((empty + MonthSet::SHORT_MONTHS) == MonthSet::SHORT_MONTHS);
    check!((MonthSet::SHORT_MONTHS - empty) == MonthSet::SHORT_MONTHS);
}

#[test]
fn ops_with_full() {
    let all = MonthSet::all();
    check!((all | MonthSet::SHORT_MONTHS).is_all());
    check!((all & MonthSet::SHORT_MONTHS) == MonthSet::SHORT_MONTHS);
    check!((all + MonthSet::SHORT_MONTHS).is_all());
    check!((all - MonthSet::SHORT_MONTHS) == (MonthSet::LONG_MONTHS));
}

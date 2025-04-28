use assert2::check;
use enum_bitset_derive::EnumBitset;
use serde::{Deserialize, Serialize};

#[derive(EnumBitset, Serialize, Deserialize, Clone, Copy, Debug)]
enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl WeekdaySet {
    const WEEKEND: WeekdaySet = WeekdaySet::from_array([Weekday::Sat, Weekday::Sun]);
    const WORKDAY: WeekdaySet = Self::WEEKEND.complement();
}

impl Weekday {
    fn is_weekend(&self) -> bool {
        WeekdaySet::WEEKEND.contains(*self)
    }

    fn is_workday(&self) -> bool {
        WeekdaySet::WORKDAY.contains(*self)
    }
}

#[test]
fn weekend() {
    assert_eq!(
        WeekdaySet::WEEKEND,
        WeekdaySet::from_array([Weekday::Sat, Weekday::Sun])
    );
}

#[test]
fn workdays() {
    assert_eq!(
        WeekdaySet::WORKDAY,
        WeekdaySet::from([
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri
        ])
    );
}

#[test]
fn is_weekend() {
    assert!(!Weekday::Mon.is_weekend());
    assert!(!Weekday::Tue.is_weekend());
    assert!(!Weekday::Wed.is_weekend());
    assert!(!Weekday::Thu.is_weekend());
    assert!(!Weekday::Fri.is_weekend());
    assert!(Weekday::Sat.is_weekend());
    assert!(Weekday::Sun.is_weekend());
}


#[test]
fn is_weekday() {
    check!(Weekday::Mon.is_workday());
    check!(Weekday::Tue.is_workday());
    check!(Weekday::Wed.is_workday());
    check!(Weekday::Thu.is_workday());
    check!(Weekday::Fri.is_workday());
    check!(!Weekday::Sat.is_workday());
    check!(!Weekday::Sun.is_workday());
}

#[test]
fn is_complementary() {
    check!(WeekdaySet::WORKDAY.is_complementary(&WeekdaySet::WEEKEND));
    check!(WeekdaySet::WEEKEND.is_complementary(&WeekdaySet::WORKDAY));
}

#[test]
fn union_complements() {
    check!(WeekdaySet::WORKDAY.union(&WeekdaySet::WEEKEND) == WeekdaySet::all());
    check!(WeekdaySet::WEEKEND.union(&WeekdaySet::WORKDAY) == WeekdaySet::all());
}

#[test]
fn intersection_complements() {
    check!(WeekdaySet::WORKDAY.intersection(&WeekdaySet::WEEKEND) == WeekdaySet::empty());
    check!(WeekdaySet::WEEKEND.intersection(&WeekdaySet::WORKDAY) == WeekdaySet::empty());
}

#[test]
fn union_random_sets() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Fri]);
    check!(
        set1.union(&set2)
            == WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Fri])
    );
    check!(
        set2.union(&set1)
            == WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Fri])
    );
}

#[test]
fn intersection_random_sets() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Fri]);
    check!(set1.intersection(&set2) == WeekdaySet::from([Weekday::Mon, Weekday::Fri]));
    check!(set2.intersection(&set1) == WeekdaySet::from([Weekday::Mon, Weekday::Fri]));
}

#[test]
fn union_empty() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::empty();
    check!(set1.union(&set2) == set1);
    check!(set2.union(&set1) == set1);
}

#[test]
fn intersection_empty() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::empty();
    check!(set1.intersection(&set2) == WeekdaySet::empty());
    check!(set2.intersection(&set1) == WeekdaySet::empty());
}

#[test]
fn union_with_all() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::all();
    check!(set1.union(&set2) == WeekdaySet::all());
    check!(set2.union(&set1) == WeekdaySet::all());
}

#[test]
fn intersection_with_all() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::all();
    check!(set1.intersection(&set2) == set1);
    check!(set2.intersection(&set1) == set1);
}

#[test]
fn difference_complements() {
    check!(WeekdaySet::WORKDAY.difference(&WeekdaySet::WEEKEND) == WeekdaySet::WORKDAY);
    check!(WeekdaySet::WEEKEND.difference(&WeekdaySet::WORKDAY) == WeekdaySet::WEEKEND);
}

#[test]
fn difference_random_sets() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Fri]);
    check!(set1.difference(&set2) == WeekdaySet::from([Weekday::Wed]));
    check!(set2.difference(&set1) == WeekdaySet::from([Weekday::Tue]));
}

#[test]
fn difference_empty() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::empty();
    check!(set1.difference(&set2) == set1);
    check!(set2.difference(&set1) == WeekdaySet::empty());
}

#[test]
fn difference_with_all() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::all();
    check!(set1.difference(&set2) == WeekdaySet::empty());
    check!(
        set2.difference(&set1)
            == WeekdaySet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat, Weekday::Sun])
    );
}

#[test]
fn symmetric_difference_complements() {
    check!(WeekdaySet::WORKDAY.symmetric_difference(&WeekdaySet::WEEKEND) == WeekdaySet::all());
    check!(WeekdaySet::WEEKEND.symmetric_difference(&WeekdaySet::WORKDAY) == WeekdaySet::all());
}

#[test]
fn symmetric_difference_random_sets() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Fri]);
    check!(set1.symmetric_difference(&set2) == WeekdaySet::from([Weekday::Tue, Weekday::Wed]));
    check!(set2.symmetric_difference(&set1) == WeekdaySet::from([Weekday::Tue, Weekday::Wed]));
}

#[test]
fn symmetric_difference_empty() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::empty();
    check!(set1.symmetric_difference(&set2) == set1);
    check!(set2.symmetric_difference(&set1) == set1);
}

#[test]
fn symmetric_difference_with_all() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::all();
    check!(
        set1.symmetric_difference(&set2)
            == WeekdaySet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat, Weekday::Sun])
    );
    check!(
        set2.symmetric_difference(&set1)
            == WeekdaySet::from([Weekday::Tue, Weekday::Thu, Weekday::Sat, Weekday::Sun])
    );
    check!(set1.symmetric_difference(&set2) == set1.complement());
}

#[test]
fn is_subset_of_complements() {
    check!(WeekdaySet::WEEKEND.is_subset_of(&WeekdaySet::all()));
    check!(!WeekdaySet::WEEKEND.is_subset_of(&WeekdaySet::WORKDAY));
    check!(!WeekdaySet::WORKDAY.is_subset_of(&WeekdaySet::WEEKEND));
}

#[test]
fn is_subset_of_random_sets_1() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Fri]);
    check!(set1.is_subset_of(&set2));
    check!(!set2.is_subset_of(&set1));
}

#[test]
fn is_subset_of_random_sets_2() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set3 = WeekdaySet::from([Weekday::Tue, Weekday::Thu]);

    check!(set1.is_subset_of(&set2));
    check!(!set2.is_subset_of(&set1));
    check!(!set1.is_subset_of(&set3));
    check!(set1.is_subset_of(&WeekdaySet::all()));
    check!(WeekdaySet::empty().is_subset_of(&set1));
}

#[test]
fn is_subset_of_empty() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::empty();
    check!(!set1.is_subset_of(&set2));
    check!(set2.is_subset_of(&set1));
}

#[test]
fn is_subset_of_all() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::all();
    check!(set1.is_subset_of(&set2));
    check!(!set2.is_subset_of(&set1));
}

#[test]
fn is_superset_of_complements() {
    check!(WeekdaySet::all().is_superset_of(&WeekdaySet::WEEKEND));
    check!(!WeekdaySet::WORKDAY.is_superset_of(&WeekdaySet::WEEKEND));
    check!(!WeekdaySet::WEEKEND.is_superset_of(&WeekdaySet::WORKDAY));
}

#[test]
fn is_superset_of_random_sets_1() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Fri]);
    check!(!set1.is_superset_of(&set2));
    check!(set2.is_superset_of(&set1));
}

#[test]
fn is_superset_of_random_sets_2() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    let set2 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set3 = WeekdaySet::from([Weekday::Tue, Weekday::Thu]);

    check!(!set1.is_superset_of(&set2));
    check!(set2.is_superset_of(&set1));
    check!(!set1.is_superset_of(&set3));
    check!(WeekdaySet::all().is_superset_of(&set1));
    check!(set1.is_superset_of(&WeekdaySet::empty()));
}

#[test]
fn is_superset_of_empty() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::empty();
    check!(set1.is_superset_of(&set2));
    check!(!set2.is_superset_of(&set1));
}

#[test]
fn is_superset_of_all() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::all();
    check!(!set1.is_superset_of(&set2));
    check!(set2.is_superset_of(&set1));
}

#[test]
fn is_disjoint_complements() {
    check!(WeekdaySet::WEEKEND.is_disjoint(&WeekdaySet::WORKDAY));
    check!(WeekdaySet::WORKDAY.is_disjoint(&WeekdaySet::WEEKEND));
}

#[test]
fn is_disjoint_random_sets_1() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    let set2 = WeekdaySet::from([Weekday::Tue, Weekday::Thu]);
    check!(set1.is_disjoint(&set2));
    check!(set2.is_disjoint(&set1));

    let set3 = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    let set4 = WeekdaySet::from([Weekday::Mon, Weekday::Thu]);
    check!(!set3.is_disjoint(&set4));
    check!(!set4.is_disjoint(&set3));
}

#[test]
fn is_disjoint_random_sets_2() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    let set2 = WeekdaySet::from([Weekday::Tue, Weekday::Thu]);
    let set3 = WeekdaySet::from([Weekday::Mon, Weekday::Fri]);

    check!(set1.is_disjoint(&set2));
    check!(set2.is_disjoint(&set1));
    check!(!set1.is_disjoint(&set3));
    check!(WeekdaySet::empty().is_disjoint(&set1));
    check!(!WeekdaySet::all().is_disjoint(&set1));
}



#[test]
fn self_operations() {
    let set = WeekdaySet::from([Weekday::Mon, Weekday::Wed]);
    check!(set.union(&set) == set);
    check!(set.intersection(&set) == set);
    check!(set.difference(&set) == WeekdaySet::empty());
    check!(set.symmetric_difference(&set) == WeekdaySet::empty());
}

#[test]
fn is_disjoint_empty() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::empty();
    check!(set1.is_disjoint(&set2));
    check!(set2.is_disjoint(&set1));
}

#[test]
fn is_disjoint_all() {
    let set1 = WeekdaySet::from([Weekday::Mon, Weekday::Wed, Weekday::Fri]);
    let set2 = WeekdaySet::all();
    check!(!set1.is_disjoint(&set2));
    check!(!set2.is_disjoint(&set1));
}


#[test]
fn empty_with_empty_operations() {
    let empty1 = WeekdaySet::empty();
    let empty2 = WeekdaySet::empty();

    check!(empty1.union(&empty2) == WeekdaySet::empty());
    check!(empty1.intersection(&empty2) == WeekdaySet::empty());
    check!(empty1.difference(&empty2) == WeekdaySet::empty());
    check!(empty1.symmetric_difference(&empty2) == WeekdaySet::empty());
    check!(empty1.is_subset_of(&empty2));
    check!(empty1.is_superset_of(&empty2));
    check!(empty1.is_disjoint(&empty2));
}

#[test]
fn all_with_all_operations() {
    let all1 = WeekdaySet::all();
    let all2 = WeekdaySet::all();

    check!(all1.union(&all2) == WeekdaySet::all());
    check!(all1.intersection(&all2) == WeekdaySet::all());
    check!(all1.difference(&all2) == WeekdaySet::empty());
    check!(all1.symmetric_difference(&all2) == WeekdaySet::empty());
    check!(all1.is_subset_of(&all2));
    check!(all1.is_superset_of(&all2));
    check!(!all1.is_disjoint(&all2));
}

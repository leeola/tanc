use std::{
    cmp::Reverse,
    collections::{btree_map, BTreeMap},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub line: usize,
    pub char: usize,
}
impl Pos {
    const MAX: Pos = Pos {
        line: usize::MAX,
        char: usize::MAX,
    };
    pub fn next_char(&self) -> Pos {
        Self {
            line: self.line,
            char: self.char + 1,
        }
    }
}
impl From<(usize, usize)> for Pos {
    fn from((line, char): (usize, usize)) -> Self {
        Self { line, char }
    }
}
// 2,5 2,9
// 1,5 1,9
// 1,0 2.1
// 0,7
// 0,5 0,6
// 0,0 2,0

// 1,7
// 1,5 1,6
// 1,0 2,0
// 0,5 0,9
// 0,1 3,0
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosRange {
    start_incl: Pos,
    end_excl: Pos,
}
impl From<(usize, usize, usize)> for PosRange {
    fn from((line, char_start_incl, char_end_excl): (usize, usize, usize)) -> Self {
        Self {
            start_incl: (line, char_start_incl).into(),
            end_excl: (line, char_end_excl).into(),
        }
    }
}
impl From<(usize, usize, usize, usize)> for PosRange {
    fn from(
        (line_start_incl, char_start_incl, line_end_excl, char_end_excl): (
            usize,
            usize,
            usize,
            usize,
        ),
    ) -> Self {
        Self {
            start_incl: (line_start_incl, char_start_incl).into(),
            end_excl: (line_end_excl, char_end_excl).into(),
        }
    }
}
/// The ending position of a `PosRange`.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EndExclPos(pub Pos);
impl From<PosRange> for EndExclPos {
    fn from(range: PosRange) -> Self {
        Self(range.end_excl)
    }
}
/// The starting position of a `PosRange`.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StartInclPos(pub Pos);
impl From<PosRange> for StartInclPos {
    fn from(range: PosRange) -> Self {
        Self(range.start_incl)
    }
}
#[derive(Debug, Default, Clone)]
struct Entry<T> {
    pub start_incl: StartInclPos,
    pub value: T,
    pub children: EntryMap<T>,
}
type EntryMap<T> = BTreeMap<EndExclPos, Entry<T>>;
/// NIT: This is a pretty naive implementation and could/should be improved at some point. However
/// it's simple and correct, so good for a first pass.
#[derive(Debug, Default, Clone)]
pub struct PosIndex<T>(EntryMap<T>);
impl<T> PosIndex<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn insert(&mut self, range: impl Into<PosRange>, value: T) {
        let range = range.into();
        Self::recur_insert_into_entry(&mut self.0, range, value);
    }
    fn recur_insert_into_entry(entry_map: &mut EntryMap<T>, range: PosRange, value: T) {
        // NIT: This works around missing upper/lower bound features in stdlib[1]. By storing with
        // the end, we can grab any point from a range and use next to grab the nearest bound.
        // See also `Self::get`
        //
        // [1]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.lower_bound
        if let Some((_, entry)) = Self::get_entry_mut(entry_map, range) {
            Self::recur_insert_into_entry(&mut entry.children, range, value);
        } else {
            let prev = entry_map.insert(
                range.into(),
                Entry {
                    start_incl: range.into(),
                    value,
                    children: Default::default(),
                },
            );
            debug_assert!(prev.is_none());
        }
    }
    fn get_entry_mut(
        entry_map: &mut EntryMap<T>,
        range: PosRange,
    ) -> Option<(&EndExclPos, &mut Entry<T>)> {
        entry_map
            .range_mut(EndExclPos::from(range)..)
            .filter(|(_, entry)| entry.start_incl <= range.into())
            .next()
    }
    // NIT: This uses Pos, the above uses PosRange. Could prob just make them `P: Into<StartInclPos>
    // + P: Into<EndExclPos>`.
    fn recur_get_entry(entry_map: &EntryMap<T>, pos: Pos) -> Option<(&EndExclPos, &Entry<T>)> {
        let (end_pos, entry) = entry_map
            .range(EndExclPos(pos.next_char())..)
            .filter(|(_, entry)| entry.start_incl <= StartInclPos(pos))
            .next()?;
        // Check the children for a match. If one is found, we choose the inner layer (them) over
        // the outer layer (this fn).
        if let Some(child_match) = Self::recur_get_entry(&entry.children, pos) {
            return Some(child_match);
        } else {
            return Some((end_pos, entry));
        }
    }
    pub fn get(&self, &pos: &Pos) -> Option<&T> {
        Self::recur_get_entry(&self.0, pos).map(|(_, entry)| &entry.value)
        // // NIT: Use upper/lower bound when they become available. For now we have to fake this by
        // // storing the end pos, and ranging onto the next end.
        // dbg!(&self, &self.0.len(), pos);
        // // NIT: This could be improved by perhaps binary searching to start from a given index,
        // but // i think more likely this should use a BTreeMap and use a reversed key type
        // to // sort the values, but then during lookup i.. think it would need to use
        // custom // comparison logic. Ie if the keys were `Reverse<T>`, it might be
        // possible to the use a // custom borrow type to compare non-reversed keys... i
        // think? //
        // // So this impl is just an easy, obvious, correct, first impl.
        // self.0
        //     .iter()
        //     .filter(|(range, _)| range.start_incl <= pos && pos <= range.end_excl)
        //     .take_while(|(range, _)| pos < range.end_excl)
        //     .filter(|(range, _)| pos < range.end_excl)
        //     .inspect(|elm| {
        //         dbg!(elm);
        //     })
        //     .last()
        //     .map(|(_, v)| v)
    }
}
#[test]
fn get_non_overlapping_single_line() {
    let mut pi = PosIndex::new();
    pi.insert((0, 0, 5), "a");
    assert_eq!(pi.get(&(0, 0).into()), Some(&"a"));
    assert_eq!(pi.get(&(0, 4).into()), Some(&"a"));
    assert_eq!(pi.get(&(0, 5).into()), None);
    pi.insert((0, 5, 10), "b");
    assert_eq!(pi.get(&(0, 4).into()), Some(&"a"));
    assert_eq!(pi.get(&(0, 5).into()), Some(&"b"));
}
#[test]
fn get_non_overlapping_multi_line() {
    let mut pi = PosIndex::new();
    pi.insert((0, 0, 3, 5), "a");
    assert_eq!(pi.get(&(0, 0).into()), Some(&"a"));
    assert_eq!(pi.get(&(1, 10).into()), Some(&"a"));
    assert_eq!(pi.get(&(3, 5).into()), None);
    pi.insert((3, 5, 3, 10), "b");
    assert_eq!(pi.get(&(3, 4).into()), Some(&"a"));
    assert_eq!(pi.get(&(3, 5).into()), Some(&"b"));
}
#[test]
fn get_overlapping_single_line() {
    let mut pi = PosIndex::new();
    pi.insert((0, 0, 9), "a");
    pi.insert((0, 3, 5), "b");
    assert_eq!(pi.get(&(0, 2).into()), Some(&"a"));
    assert_eq!(pi.get(&(0, 5).into()), Some(&"a"));
    assert_eq!(pi.get(&(0, 3).into()), Some(&"b"));
    assert_eq!(pi.get(&(0, 4).into()), Some(&"b"));
}
// #[test]
// fn get_multi_line_overlapping_single_line() {
//     let mut pi = PosIndex::new();
//     pi.insert((0, 0, 3, 5), "a");
//     assert_eq!(pi.get(&(0, 0).into()), Some(&"a"));
//     assert_eq!(pi.get(&(1, 10).into()), Some(&"a"));
//     assert_eq!(pi.get(&(3, 5).into()), None);
//     pi.insert((1, 5, 1, 10), "b");
//     assert_eq!(pi.get(&(0, 5).into()), Some(&"a"));
//     assert_eq!(pi.get(&(1, 4).into()), Some(&"a"));
//     assert_eq!(pi.get(&(1, 5).into()), Some(&"b"));
//     assert_eq!(pi.get(&(1, 10).into()), Some(&"b"));
//     assert_eq!(pi.get(&(1, 11).into()), Some(&"a"));
//     assert_eq!(pi.get(&(3, 4).into()), Some(&"a"));
// }

use std::{cmp::Reverse, collections::BTreeMap};

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
}
impl From<(usize, usize)> for Pos {
    fn from((line, char): (usize, usize)) -> Self {
        Self { line, char }
    }
}
// 2,5 2,9
// 1,5 1,9
// 1,0 2.1
// 0,5
// 0,5 0,9
// 0,5 0,7
// 0,0 2,0
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Key {
    end_excl: Pos,
    start_incl: Reverse<Pos>,
}
impl From<PosRange> for Key {
    fn from(
        PosRange {
            start_incl,
            end_excl,
        }: PosRange,
    ) -> Self {
        Self {
            start_incl: Reverse(start_incl),
            end_excl: end_excl,
        }
    }
}
impl From<Pos> for Key {
    fn from(pos: Pos) -> Self {
        Key {
            end_excl: Default::default(),
            start_incl: Reverse(pos),
        }
    }
}
#[test]
fn key_ord() {
    // assert!(
    //     Key::from(PosRange::from((1, 0, 2, 0))) < Key::from(PosRange::from((0, 0, 1, 0))),
    //     "keys should be reverse"
    // );
    // assert!(
    //     Key::from(PosRange::from((0, 0, 1, 0))) < Key::from(PosRange::from((0, 5, 0, 10))),
    //     "keys should sort end before start",
    // );
}
#[derive(Debug, Clone)]
pub struct Builder<T>(Vec<(PosRange, T)>);
impl<T> Builder<T>
where
    T: std::fmt::Debug,
{
    pub fn insert(&mut self, range: impl Into<PosRange>, t: T) {
        self.0.push((range.into(), t));
    }
    pub fn with(mut self, range: impl Into<PosRange>, t: T) -> Self {
        self.insert(range, t);
        self
    }
    pub fn build(mut self) -> PosIndex<T> {
        self.0.sort_unstable_by(|(a, _), (b, _)| a.cmp(&b));
        PosIndex(self.0)
    }
}
impl<T> Default for Builder<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
#[derive(Debug, Default, Clone)]
pub struct PosIndex<T>(Vec<(PosRange, T)>);
impl<T> PosIndex<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Builder<T> {
        Builder::default()
    }
    pub fn get(&self, &pos: &Pos) -> Option<&T> {
        self.0.iter().for_each(|v| {
            dbg!(&v);
        });
        dbg!(&self.0);
        // NIT: This could be improved by perhaps binary searching, but i think more likely this
        // should use a BTreeMap and use a reversed key type to sort the values, but then
        // during lookup i.. think it would need to use custom comparison logic. So this
        // impl is just an easy, obvious, first impl.
        self.0
            .iter()
            .filter(|(range, _)| range.start_incl <= pos)
            .take_while(|(range, _)| pos < range.end_excl)
            .last()
            .map(|(_, v)| v)
    }
}
#[test]
fn get_non_overlapping_single_line() {
    let b = PosIndex::new().with((0, 0, 5), "a");
    let pi = b.clone().build();
    assert_eq!(pi.get(&(0, 0).into()), Some(&"a"));
    assert_eq!(pi.get(&(0, 5).into()), None);
    let pi = b.clone().with((0, 5, 10), "b").build();
    assert_eq!(pi.get(&(0, 4).into()), Some(&"a"));
    assert_eq!(pi.get(&(0, 5).into()), Some(&"b"));
}
// #[test]
// fn get_non_overlapping_multi_line() {
//     let mut pi = PosIndex::new();
//     pi.insert((0, 0, 3, 5), "a");
//     assert_eq!(pi.get(&(0, 0).into()), Some(&"a"));
//     assert_eq!(pi.get(&(1, 10).into()), Some(&"a"));
//     assert_eq!(pi.get(&(3, 5).into()), None);
//     pi.insert((3, 5, 3, 10), "b");
//     assert_eq!(pi.get(&(3, 4).into()), Some(&"a"));
//     assert_eq!(pi.get(&(3, 5).into()), Some(&"b"));
// }
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

use core::fmt;
use std::{fmt::Display, str::FromStr};

use compact_str::CompactString;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Remote {
    GitRepo {
        host: CompactString,
        user: CompactString,
        repo: CompactString,
    },
}

// NIT: Might be useful to store this in a single string, rather than parsing. I suspect i'll
// refactor this entirely once i know how AST lookup will peform on requests.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path {
    pub remote: Option<Remote>,
    pub commit: Option<CompactString>,
    pub file_path: Option<CompactString>,
    pub syntax_path: Option<CompactString>,
}
impl FromStr for Path {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Default::default())
    }
}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
macro_rules! assert_parse_and_display_eq {
    ($str:expr, $expect:expr) => {
        let expect = $expect;
        let parsed: Result<Path, _> = $str.parse();
        assert_eq!(parsed.as_ref(), Ok(&expect));
        assert_eq!($str, format!("{expect}"));
    };
}

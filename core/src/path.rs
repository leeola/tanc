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

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path {
    pub remote: Option<Remote>,
    pub commit: Option<CompactString>,
    pub file_path: Option<CompactString>,
    pub syntax_path: SyntaxPath,
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

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyntaxPath {
    #[default]
    Root,
    Type(CompactString),
    Segments(Vec<CompactString>),
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
#[test]
fn path_io() {
    // # foo
    // <github.com:foo/bar[]>
    // <foo/bar#baz.bang>
    assert_parse_and_display_eq!(
        "foo",
        Path {
            syntax_path: SyntaxPath::Type("foo".into()),
            ..Default::default()
        }
    );
    assert_parse_and_display_eq!(
        "/foo/bar#foo",
        Path {
            file_path: Some("/foo/bar"),
            syntax_path: SyntaxPath::Type("foo".into()),
            ..Default::default()
        }
    );
    assert_parse_and_display_eq!(
        "github.com:foo/bar:foo/bar#foo",
        Path {
            remote: Some(Remote::GitRepo {
                host: "github.com".into(),
                user: "foo".into(),
                repo: "bar".into()
            }),
            file_path: Some("/foo/bar"),
            syntax_path: SyntaxPath::Type("foo".into()),
            ..Default::default()
        }
    );
}

use compact_str::CompactString;
use rnix::SyntaxKind;
use std::collections::BTreeMap;

mod ast_path;
mod consume_ast;
mod pos_index;

#[derive(Debug, Default, Clone)]
pub struct TancIndex {
    files: BTreeMap<FileKey, FileIndex>,
}
impl TancIndex {
    pub fn insert(&mut self, file_path: impl Into<CompactString>, src: &str) {
        // drop the previous index. Currently no use in persisting on a full new file.
        let _: Option<FileIndex> = self.files.insert(
            FileKey {
                // TODO: Include commit, if any? Need some way to associate any given source with a
                // flakes.lock for automatic association.
                commit: None,
                file_path: file_path.into(),
            },
            FileIndex::new(src),
        );
    }
    pub fn doc(&self, file_path: impl Into<CompactString>, line: usize, char: usize) -> &Doc {
        todo!()
    }
    #[cfg(test)]
    pub fn docs(&self, file_path: impl Into<CompactString>) -> Vec<&Doc> {
        self.files
            .get(&FileKey {
                commit: None,
                file_path: file_path.into(),
            })
            .iter()
            .flat_map(|fi| fi.data.iter())
            .map(|(_, doc)| doc)
            .collect()
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FileKey {
    pub commit: Option<CompactString>,
    pub file_path: CompactString,
}
#[derive(Debug, Default, Clone)]
pub struct FileIndex {
    pos_index: BTreeMap<(usize, usize), String>,
    data: BTreeMap<String, Doc>,
}
impl FileIndex {
    pub fn new(s: &str) -> Self {
        let ast = rnix::Root::parse(s);
        dbg!(&ast.tree());
        let docs = consume_ast::docs_from_ast(ast.syntax().into());
        dbg!(&ast.syntax().kind() == &SyntaxKind::NODE_ROOT);
        todo!();
    }
    pub fn doc(&self, line: usize, char: usize) -> &Doc {
        todo!()
    }
    #[cfg(test)]
    pub fn docs(&self) -> Vec<&Doc> {
        self.data.iter().map(|(_, doc)| doc).collect()
    }
}

#[derive(Debug, Clone, Hash)]
pub struct AstId {
    pub label: CompactString,
    pub id: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Doc {
    pub doc: Option<String>,
}

#[cfg(test)]
pub mod test;

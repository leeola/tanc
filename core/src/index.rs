use compact_str::CompactString;
use rnix::ast::AstToken;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct TancIndex {}

#[derive(Debug, Default, Clone)]
pub struct FileIndex {
    pos_index: BTreeMap<(usize, usize), AstId>,
    data: BTreeMap<AstId, Doc>,
}
impl FileIndex {
    pub fn with_nix(&mut self, s: &str) {
        let ast = rnix::Root::parse(s);
        dbg!(&ast.tree());
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

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Doc {
    pub doc: String,
}

#[cfg(test)]
pub mod test;

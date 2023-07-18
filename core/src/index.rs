use compact_str::CompactString;
use rnix::{
    ast::{AstNode, AstToken, Comment},
    NodeOrToken, SyntaxKind, SyntaxNode, SyntaxToken, TextRange,
};
use std::collections::BTreeMap;
use tracing::{error, warn};

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
    pos_index: BTreeMap<(usize, usize), AstId>,
    data: BTreeMap<AstId, Doc>,
}
impl FileIndex {
    pub fn new(s: &str) -> Self {
        let ast = rnix::Root::parse(s);
        dbg!(&ast.tree());
        let ConsumeAst { docs, .. } = ConsumeAst::new(ast.syntax().into());
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

#[derive(Debug, Default, Clone)]
struct DocWithMeta {
    pub line: usize,
    pub char: usize,
    pub doc: Doc,
}
#[derive(Debug, Default, Clone)]
struct ConsumeAst {
    pub docs: Vec<DocWithMeta>,
    line_cursor: usize,
    line_cursor_char_offset: usize,
    doc_buf: Vec<Comment>,
}
impl ConsumeAst {
    pub fn new(node_or_token: NodeOrToken<SyntaxNode, SyntaxToken>) -> Self {
        let mut self_ = Self::default();
        self_.node_or_token(node_or_token);
        self_
    }
    fn node_or_token(&mut self, node_or_token: NodeOrToken<SyntaxNode, SyntaxToken>) {
        match node_or_token {
            NodeOrToken::Node(node) => self.node(node),
            NodeOrToken::Token(token) => self.token(token),
        }
    }
    fn node(&mut self, node: SyntaxNode) {
        dbg!(node.kind());
        match node.kind() {
            SyntaxKind::NODE_ROOT => {
                for child in node.children_with_tokens() {
                    self.node_or_token(child);
                }
            },
            _ => todo!(),
        }
    }
    fn token(&mut self, token: SyntaxToken) {
        dbg!(token.kind());
        match token.kind() {
            SyntaxKind::TOKEN_COMMENT => {
                // NIT: Are there cases where a comment should advance the cursor?
                let Some(comment) = Comment::cast(token) else {
                    error!("TOKEN_COMMENT failed to cast to Comment");
                    return;
                };
                // TODO: Parse out TANC syntax, etc. Not sure if i want to parse it here or in the
                // consumer of the doc.
                self.doc_buf.push(comment);
            },
            SyntaxKind::TOKEN_WHITESPACE => {
                let start_incl: usize = token.text_range().start().into();
                let (count, index) = token
                    .text()
                    .char_indices()
                    // NIT: Support various newline types? I think just \n vs \r\n, but i'm not
                    // positive how best to support multi-os here. It also may
                    // not matter, as if it's always \r\n, then \n is still the
                    // final char and can be counted and indexed in the same
                    // manner.
                    .filter(|&(_, c)| c == '\n')
                    .enumerate()
                    .last()
                    .map(|(line_index, (index, _))| (line_index + 1, index))
                    .unwrap_or_default();
                self.line_cursor += count;
                self.line_cursor_char_offset = start_incl + index;
            },
            _ => todo!(),
        }
    }
    fn doc_pos(&self, range: TextRange) -> (usize, usize) {
        todo!()
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

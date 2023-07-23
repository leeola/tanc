use crate::index::ast_path::AstSeg;

use super::{ast_path::AstPath, Doc};
use compact_str::CompactString;
use rnix::{
    ast::{AstNode, AstToken, Comment, Ident},
    NodeOrToken, SyntaxKind, SyntaxNode, SyntaxToken, TextRange,
};
use std::collections::BTreeMap;
use tracing::error;

pub fn docs_from_ast(
    node_or_token: NodeOrToken<SyntaxNode, SyntaxToken>,
) -> BTreeMap<AstPath, Doc> {
    let mut shared_state = Default::default();
    let mut consume = Root::new(&mut shared_state, node_or_token);
    shared_state.ast_index
}

#[derive(Debug, Default)]
struct SharedState {
    ast_index: BTreeMap<AstPath, Doc>,
    /// A cursor of the current line for indexing.
    ///
    line_cursor: usize,
    line_cursor_char_offset: usize,
}
impl SharedState {
    // pub fn consume_whitespace(&mut self, )
}
#[derive(Debug)]
struct Root<'a> {
    shared: &'a mut SharedState,
    comment_buf: Vec<Comment>,
}
impl<'a> Root<'a> {
    pub fn new(
        shared_state: &'a mut SharedState,
        node_or_token: NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Self {
        let mut self_ = Self {
            shared: shared_state,
            comment_buf: Vec::new(),
        };
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
            SyntaxKind::NODE_ATTR_SET => {
                // // NIT: attrSet is just an experimental name, undecided atm.
                // let path = self.path.new_child(AstSeg::AttrSet);
                // let mut children: Vec<CompactString> = Vec::new();
                // for child in node.children_with_tokens() {
                //     let mut ast = Root::new(self.shared, path.clone());
                //     ast.node_or_token(child);
                // }
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
                self.comment_buf.push(comment);
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
                self.shared.line_cursor += count;
                self.shared.line_cursor_char_offset = start_incl + index;
            },
            SyntaxKind::TOKEN_L_BRACE | SyntaxKind::TOKEN_R_BRACE => {},
            _ => todo!(),
        }
    }
    fn doc_pos(&self, range: TextRange) -> (usize, usize) {
        todo!()
    }
}
#[derive(Debug)]
struct AttrSet<'a> {
    shared: &'a mut SharedState,
    path: AstPath,
    comment_buf: Vec<Comment>,
    children: Vec<CompactString>,
}
impl AttrSet<'_> {
    pub fn new(
        shared: &mut SharedState,
        path: AstPath,
        comment_buf: Vec<Comment>,
        node: SyntaxNode,
    ) -> AttrSet<'_> {
        todo!()
    }
}
#[derive(Debug)]
struct AttrPathValue<'a> {
    shared: &'a mut SharedState,
    comment_buf: Vec<Comment>,
}
impl AttrPathValue<'_> {
    pub fn new(
        shared: &'_ mut SharedState,
        comment_buf: Vec<Comment>,
        node: SyntaxNode,
    ) -> AttrPathValue<'_> {
        let mut self_ = AttrPathValue {
            shared,
            comment_buf,
        };
        self_.node(node);
        self_
    }
    fn node(&mut self, node: SyntaxNode) {
        dbg!(node.kind());
        match node.kind() {
            SyntaxKind::NODE_ATTRPATH_VALUE => {
                // for child in node.children_with_tokens() {
                //     self.node_or_token(child);
                // }
                todo!();
            },
            SyntaxKind::NODE_ATTRPATH => {
                eprintln!("{}", node.text());
                // for child in node.children() {
                //     self.node(child);
                // }
            },
            SyntaxKind::NODE_IDENT => {
                // for child in node.children_with_tokens() {
                //     self.node_or_token(child);
                // }
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
                self.comment_buf.push(comment);
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
                self.shared.line_cursor += count;
                self.shared.line_cursor_char_offset = start_incl + index;
            },
            SyntaxKind::TOKEN_L_BRACE | SyntaxKind::TOKEN_R_BRACE => {},
            _ => todo!(),
        }
    }
}

enum TokenOrNode {
    /// An unused or unrecognized node or token.
    Skip,
    Comment {
        comment: Comment,
    },
    Whitespace {
        /// The total count of newlines in this whitespace.
        ///
        /// See also: [`SharedState::line_cursor`]
        newline_count: usize,
        /// The char offset of the final newline in this whitespace. Offset is relative to the
        /// origin source, and is necessary to understand syntax positioning relative to
        /// lines.
        ///
        /// See also: [`SharedState::line_cursor_char_offset`]
        last_newline_char_offset: usize,
    },
}
impl From<SyntaxToken> for TokenOrNode {
    fn from(token: SyntaxToken) -> Self {
        match token.kind() {
            SyntaxKind::TOKEN_COMMENT => {
                // NIT: Are there cases where a comment should advance the cursor? May need to
                // include positional information.
                Comment::cast(token).unwrap_or_else(|| {
                    error!("TOKEN_COMMENT failed to cast to Comment");
                    Self::Skip
                })
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
                Self::Whitespace {
                    newline_count: count,
                    last_newline_char_offset: start_incl + index,
                }
            },
            SyntaxKind::TOKEN_L_BRACE | SyntaxKind::TOKEN_R_BRACE => {},
            _ => todo!(),
        }
    }
}

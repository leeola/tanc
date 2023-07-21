#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub line: usize,
    /// The char offset as LSPs seem to expect them, from the start of the associated
    /// [`Self::line`].
    pub char: usize,
}
pub struct AstIndex {}
impl AstIndex {
    pub fn build() -> AstIndexBuilder {
        AstIndexBuilder::default()
    }
}
#[derive(Debug, Default, Clone)]
pub struct AstIndexBuilder {
    pos_index: BTreeMap<Pos, DocBuilder>,
    line_cursor: usize,
    line_cursor_char_offset: usize,
}

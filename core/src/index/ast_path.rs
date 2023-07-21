use compact_str::CompactString;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AstPath(Vec<AstSeg>);
impl AstPath {
    pub fn new_child(&self, seg: impl Into<AstSeg>) -> Self {
        let mut child = self.clone();
        child.0.push(seg.into());
        child
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AstSeg {
    AttrSet,
    Ident(CompactString),
}
impl From<CompactString> for AstSeg {
    fn from(value: CompactString) -> Self {
        Self::Ident(value)
    }
}

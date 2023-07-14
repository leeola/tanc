use compact_str::CompactString;
use std::collections::HashMap;

#[derive(Clone)]
pub struct _SrcBuf {
    _files: HashMap<CompactString, String>,
}

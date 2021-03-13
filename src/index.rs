/// An index entry
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexEntry<'a> {
    /// Pointer (file-relative) to the left subtree of the BST
    pub left: u64,
    /// Pointer (file-relative) to the right subtree of the BST
    pub right: u64,
    /// Row number in source file
    pub row: u32,
    /// Column number in source file
    pub col: u32,
    /// Pointer to the command statement that defines this item
    pub proof: u64,
    /// Index to the item in the relevant table
    pub idx: u32,
    /// To identify the item type
    pub kind: u8,
    /// A null-terminated character string with the name
    pub name: &'a [u8],
}

pub trait Visitor {
    fn visit<'a>(&mut self, ptr: u64, entry: IndexEntry);
}

/// Index
/// The index contains information about items in the proof file that are useful
/// for printing and debugging purposes.
#[derive(Debug)]
pub struct Index<'a> {
    /// The root pointer of the index BST (binary search tree)
    pub(crate) root_bst_ptr: u64,
    /// A pointer to the proof file, used to resolve file relative pointers
    pub(crate) file: &'a [u8],
    pub(crate) sorts: &'a [u8],
    pub(crate) terms: &'a [u8],
    pub(crate) theorems: &'a [u8],
}

use crate::{error, parser};

impl<'a> Index<'a> {
    /// Return a pointer to the root of the index binary search tree (BST)
    pub fn root_bst_ptr(&self) -> u64 {
        self.root_bst_ptr
    }

    /// Return the slice of the data containing the pointers to the sort index
    /// entries
    pub fn sorts(&self) -> &[u8] {
        self.sorts
    }

    /// Return the pointer to the sort index entry by index number
    pub fn sort_pointer(&self, idx: usize) -> u64 {
        let (_, ptr) = parser::parse_index_pointer(self.sorts, idx).unwrap();
        ptr
    }

    /// Return the slice of the data containing the pointers to the term index
    /// entries
    pub fn terms(&self) -> &[u8] {
        self.terms
    }

    /// Return the pointer to the sort index entry by index number
    pub fn term_pointer(&self, idx: usize) -> u64 {
        let (_, ptr) = parser::parse_index_pointer(self.terms, idx).unwrap();
        ptr
    }

    /// Return the slice of the data containing the pointers to the theorem index
    /// entries
    pub fn theorems(&self) -> &[u8] {
        self.theorems
    }

    /// Return the pointer to the theorem index entry by index number
    pub fn theorem_pointer(&self, idx: usize) -> u64 {
        let (_, ptr) = parser::parse_index_pointer(self.theorems, idx).unwrap();
        ptr
    }

    /// Visit all sorts in the index using the supplied `visitor`
    pub fn visit_sorts<V: Visitor>(&self, visitor: &mut V) {
        let num_sorts = self.sorts.len() / 8;
        parser::parse_index(self.file, self.sorts, num_sorts, visitor).unwrap();
    }

    /// Visit all terms in the index using the supplied `visitor`
    pub fn visit_terms<V: Visitor>(&self, visitor: &mut V) {
        let num_terms = self.terms.len() / 8;
        parser::parse_index(self.file, self.terms, num_terms, visitor).unwrap();
    }

    /// Visit all theorems in the index using the supplied `visitor`
    pub fn visit_theorems<V: Visitor>(&self, visitor: &mut V) {
        let num_theorems = self.theorems.len() / 8;
        parser::parse_index(self.file, self.theorems, num_theorems, visitor).unwrap();
    }

    /// Get the pointer to a sort index entry, and the entry data itself
    pub fn sort(&self, idx: usize) -> Result<IndexEntry, nom::Err<error::ParseError>> {
        let (_, entry) = parser::parse_index_by_idx(self.file, self.sorts, idx)?;

        Ok(entry)
    }

    /// Get the pointer to a term index entry, and the entry data itself
    pub fn term(&self, idx: usize) -> Result<IndexEntry, nom::Err<error::ParseError>> {
        let (_, entry) = parser::parse_index_by_idx(self.file, self.terms, idx)?;

        Ok(entry)
    }

    /// Get the pointer to a theorem index entry, and the entry data itself
    pub fn theorem(&self, idx: usize) -> Result<IndexEntry, nom::Err<error::ParseError>> {
        let (_, entry) = parser::parse_index_by_idx(self.file, self.theorems, idx)?;

        Ok(entry)
    }
}

/// An index entry
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexEntry<'a> {
    /// Pointer to this entry in the index
    pub ptr: u64,
    /// Pointer (file-relative) to the left subtree of this item in the BST
    pub left: u64,
    /// Pointer (file-relative) to the right subtree of this item in the BST
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
    fn visit<'a>(&mut self, entry: IndexEntry);
}

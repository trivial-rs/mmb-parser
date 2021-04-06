/// Index
/// The index contains information about items in the proof file that are useful
/// for printing and debugging purposes.
///
/// The index is a collection of tables that in turn contain domain specific
/// data. Because the index is designed to be extensible, each table entry is
/// identified by an id that determines how the data should be interpreted.
#[derive(Debug)]
pub struct Index<'a> {
    pub(crate) file: &'a [u8],

    pub(crate) num_sorts: u8,
    pub(crate) num_terms: u32,
    pub(crate) num_theorems: u32,

    /// The number of table entries in the index
    pub(crate) num_entries: u64,

    /// The slice containing the table entries.
    pub(crate) entries: &'a [u8],
}

use crate::parser;

pub use self::name_table::NameTable;

pub mod name_table;

impl<'a> Index<'a> {
    /// Returns an iterator over all table entries in the index.
    pub fn iter(&self) -> EntryIterator<'a> {
        EntryIterator {
            entries: self.entries,
        }
    }
}

impl<'a> IntoIterator for Index<'a> {
    type Item = Entry;
    type IntoIter = EntryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &Index<'a> {
    type Item = Entry;
    type IntoIter = EntryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over table entries in the index.
pub struct EntryIterator<'a> {
    entries: &'a [u8],
}

/// A table entry in the index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entry {
    pub(crate) id: u32,
    pub(crate) ptr: u64,
}

const NAME_TABLE_ID: u32 = 0x656d614e;

impl Entry {
    /// If this table entry is a name table, return a `NameTable` object to the
    /// name table, or `None` otherwise.
    ///
    /// Because the name table contains file relative pointers, we need the
    /// original index as a parameter to resolve these.
    pub fn as_name_table<'a>(&self, index: &Index<'a>) -> Option<NameTable<'a>> {
        if self.id != NAME_TABLE_ID {
            return None;
        }

        let num = index.num_sorts as u64 + index.num_terms as u64 + index.num_theorems as u64;
        let entries = parser::parse_name_entries(index.file, num, self.ptr)
            .ok()?
            .1;

        Some(NameTable::new(
            index.num_sorts,
            index.num_terms,
            index.num_theorems,
            index.file,
            entries,
        ))
    }
}

impl<'a> Iterator for EntryIterator<'a> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let (left, entry) = parser::parse_index_entry(self.entries).ok()?;

        self.entries = left;

        Some(entry)
    }
}

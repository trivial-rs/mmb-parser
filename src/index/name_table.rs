/// The name table of the index defines the names and pointers to the
/// declarations of sorts, terms and theorems in the proof file.
pub struct NameTable<'a> {
    num_sorts: u8,
    num_terms: u32,
    num_theorems: u32,

    file: &'a [u8],
    entries: &'a [u8],
}

/// A subsection of the name table containing only a single kind of entry.
pub struct NameTableSection<'a> {
    file: &'a [u8],
    entries: &'a [u8],
}

use crate::parser;

impl<'a> NameTableSection<'a> {
    /// Returns an entry of the name table by index, or `None` if the index is
    /// out of range.
    pub fn get(&self, idx: u64) -> Option<Name<'_>> {
        let name = parser::seek_name_entry(self.file, self.entries, idx).ok()?;

        Some(name.1)
    }

    /// Returns an iterator over all entries in this subsection of the name table.
    pub fn iter(&self) -> NameIterator<'a> {
        NameIterator {
            file: self.file,
            entries: self.entries,
        }
    }
}

impl<'a> NameTable<'a> {
    pub(crate) fn new(
        num_sorts: u8,
        num_terms: u32,
        num_theorems: u32,
        file: &'a [u8],
        entries: &'a [u8],
    ) -> NameTable<'a> {
        NameTable {
            num_sorts,
            num_terms,
            num_theorems,
            file,
            entries,
        }
    }

    /// Returns the subsection of the name table containing the sorts.
    pub fn sorts(&self) -> NameTableSection<'_> {
        let from = 0;
        let len = self.num_sorts as u64;

        self.kind(from, len)
    }

    /// Returns the subsection of the name table containing the terms.
    pub fn terms(&self) -> NameTableSection<'_> {
        let from = self.num_sorts as u64;
        let len = self.num_terms as u64;

        self.kind(from, len)
    }

    /// Returns the subsection of the name table containing the theorems.
    pub fn theorems(&self) -> NameTableSection<'_> {
        let from = self.num_sorts as u64 + self.num_terms as u64;
        let len = self.num_theorems as u64;

        self.kind(from, len)
    }

    /// Returns an iterator over all entries in the entire name table.
    pub fn iter(&self) -> NameIterator<'a> {
        NameIterator {
            file: self.file,
            entries: self.entries,
        }
    }

    fn kind(&self, from: u64, len: u64) -> NameTableSection<'_> {
        let entries = parser::subslice_name_table(self.entries, from, len)
            .ok()
            .unwrap()
            .1;

        NameTableSection {
            file: self.file,
            entries,
        }
    }
}

impl<'a> IntoIterator for NameTable<'a> {
    type Item = Name<'a>;
    type IntoIter = NameIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &NameTable<'a> {
    type Item = Name<'a>;
    type IntoIter = NameIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for NameTableSection<'a> {
    type Item = Name<'a>;
    type IntoIter = NameIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &NameTableSection<'a> {
    type Item = Name<'a>;
    type IntoIter = NameIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over entries in the name table.
pub struct NameIterator<'a> {
    file: &'a [u8],
    entries: &'a [u8],
}

/// An entry in the name table.
///
/// Entries in the name table contain a pointer to the declaration of the item
/// in the proof stream and a nul-terminated string of the name of the item.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name<'a> {
    pub ptr: u64,
    pub name: &'a [u8],
}

impl<'a> Iterator for NameIterator<'a> {
    type Item = Name<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (left, name) = parser::parse_name_entry(self.file, self.entries).ok()?;

        self.entries = left;

        Some(name)
    }
}

impl<'a> Name<'a> {
    pub fn to_str(&self) -> Result<&str, &[u8]> {
        if self.name.len() == 0 {
            Err(&[])
        } else {
            std::str::from_utf8(self.name).map_err(|_| self.name)
        }
    }
}

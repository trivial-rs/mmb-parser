pub use mmb_types::opcode;
mod error;
pub mod index;
mod parser;
pub mod visitor;

pub use visitor::{ProofStream, UnifyStream, Visitor};

#[derive(Debug)]
pub struct Mmb<'a> {
    file: &'a [u8],
    version: u8,
    num_sorts: u8,
    num_terms: u32,
    num_theorems: u32,
    sorts: &'a [u8],
    terms: &'a [u8],
    theorems: &'a [u8],
    proofs: &'a [u8],
    index: Option<index::Index<'a>>,
}

impl<'a> Mmb<'a> {
    /// Build a `Mmb` struct by parsing the file header
    pub fn from(file: &'a [u8]) -> Option<Mmb<'a>> {
        parser::parse(file).map(|x| x.1).ok()
    }

    /// Return the slice containing the entire file
    pub fn file(&self) -> &[u8] {
        self.file
    }

    /// Return the version of the proof file format
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Return the number of sorts in the sort table
    pub fn num_sorts(&self) -> u8 {
        self.num_sorts
    }

    /// Return the number of terms in the term table
    pub fn num_terms(&self) -> u32 {
        self.num_terms
    }

    /// Return the number of theorems in the theorem table
    pub fn num_theorems(&self) -> u32 {
        self.num_theorems
    }

    /// Return the slice containing the sort table
    pub fn sorts(&self) -> &[u8] {
        self.sorts
    }

    /// Return the slice containing the term table
    pub fn terms(&self) -> &[u8] {
        self.terms
    }

    /// Return the slice containing the theorem table
    pub fn theorems(&self) -> &[u8] {
        self.theorems
    }

    /// Return the slice containing the proof section
    pub fn proofs(&self) -> &[u8] {
        self.proofs
    }

    /// Return a reference to the optional index section
    pub fn index(&self) -> Option<&index::Index> {
        self.index.as_ref()
    }

    pub fn visit<V: Visitor<'a>>(
        &self,
        visitor: &mut V,
    ) -> Result<(), nom::Err<error::ParseError>> {
        let (_, _) = parser::parse_sorts(self.sorts, self.num_sorts, visitor)?;
        let (_, _) = parser::scan_statement_stream(self.proofs, visitor)?;
        let (_, _) = parser::parse_terms(self.file, self.terms, self.num_terms as usize, visitor)?;
        let (_, _) = parser::parse_theorems(
            self.file,
            self.theorems,
            self.num_theorems as usize,
            visitor,
        )?;

        Ok(())
    }
}

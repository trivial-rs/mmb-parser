use mmb_types::opcode;
mod error;
pub mod index;
mod parser;
pub mod visitor;

pub use visitor::{ProofStream, UnifyStream, Visitor};

#[derive(Debug)]
pub struct Mmb<'a> {
    pub file: &'a [u8],
    pub version: u8,
    pub num_sorts: u8,
    pub num_terms: u32,
    pub num_theorems: u32,
    pub sorts: &'a [u8],
    pub terms: &'a [u8],
    pub theorems: &'a [u8],
    pub proofs: &'a [u8],
    pub sort_index: &'a [u8],
    pub term_index: &'a [u8],
    pub theorem_index: &'a [u8],
}

impl<'a> Mmb<'a> {
    pub fn from(file: &'a [u8]) -> Option<Mmb<'a>> {
        parser::parse(file).map(|x| x.1).ok()
    }

    pub fn visit_sort_index<V: index::Visitor>(&self, visitor: &mut V) {
        parser::parse_index(self.file, self.sort_index, self.num_sorts as usize, visitor).unwrap();
    }

    pub fn visit_term_index<V: index::Visitor>(&self, visitor: &mut V) {
        parser::parse_index(self.file, self.term_index, self.num_terms as usize, visitor).unwrap();
    }

    pub fn visit_theorem_index<V: index::Visitor>(&self, visitor: &mut V) {
        parser::parse_index(
            self.file,
            self.theorem_index,
            self.num_theorems as usize,
            visitor,
        )
        .unwrap();
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

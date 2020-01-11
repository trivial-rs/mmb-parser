use crate::opcode::{Command, Proof, Statement, Unify};

pub trait UnifyStream {
    fn push(&mut self, command: Command<Unify>);

    fn done(&self) -> (usize, usize);
}

pub trait ProofStream {
    fn push(&mut self, command: Command<Proof>);

    fn done(&self) -> (usize, usize);
}

pub trait Visitor<'a> {
    type Binder: From<u64>;
    type Sort: From<u8>;
    type Statement: From<Statement>;
    type Unify: UnifyStream;
    type Proof: ProofStream;

    fn parse_sort(&mut self, sort: Self::Sort);

    fn parse_statement(
        &mut self,
        statement: Self::Statement,
        offset: usize,
        slice: &'a [u8],
        proof: Option<(usize, usize)>,
    );

    fn try_reserve_binder_slice(&mut self, nr: usize) -> Option<(&mut [Self::Binder], usize)>;

    fn start_unify_stream(&mut self) -> &mut Self::Unify;

    fn start_proof_stream(&mut self) -> &mut Self::Proof;

    fn parse_term(
        &mut self,
        sort_idx: u8,
        binders: (usize, usize),
        ret_ty: Self::Binder,
        unify: &'a [u8],
        unify_indices: (usize, usize),
    );

    fn parse_theorem(
        &mut self,
        binders: (usize, usize),
        unify: &'a [u8],
        unify_indices: (usize, usize),
    );
}

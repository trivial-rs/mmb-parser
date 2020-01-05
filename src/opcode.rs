const OPCODE_END: u8 = 0x00;
const OPCODE_STMT_AXIOM: u8 = 0x02;
const OPCODE_STMT_SORT: u8 = 0x04;
const OPCODE_STMT_TERM_DEF: u8 = 0x05;
const OPCODE_STMT_THM: u8 = 0x06;
const OPCODE_STMT_LOCAL_DEF: u8 = 0x0D;
const OPCODE_STMT_LOCAL_TERM: u8 = 0x0E;
const OPCODE_PROOF_TERM: u8 = 0x10;
const OPCODE_PROOF_TERM_SAVE: u8 = 0x11;
const OPCODE_PROOF_REF: u8 = 0x12;
const OPCODE_PROOF_DUMMY: u8 = 0x13;
const OPCODE_PROOF_THM: u8 = 0x14;
const OPCODE_PROOF_THM_SAVE: u8 = 0x15;
const OPCODE_PROOF_HYP: u8 = 0x16;
const OPCODE_PROOF_CONV: u8 = 0x17;
const OPCODE_PROOF_REFL: u8 = 0x18;
const OPCODE_PROOF_SYMM: u8 = 0x19;
const OPCODE_PROOF_CONG: u8 = 0x1A;
const OPCODE_PROOF_UNFOLD: u8 = 0x1B;
const OPCODE_PROOF_CONV_CUT: u8 = 0x1C;
const OPCODE_PROOF_CONV_REF: u8 = 0x1D;
const OPCODE_PROOF_CONV_SAVE: u8 = 0x1E;
const OPCODE_UNIFY_TERM: u8 = 0x30;
const OPCODE_UNIFY_TERM_SAVE: u8 = 0x31;
const OPCODE_UNIFY_REF: u8 = 0x32;
const OPCODE_UNIFY_DUMMY: u8 = 0x33;
const OPCODE_UNIFY_HYP: u8 = 0x36;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    End = OPCODE_END,
    StAxiom = OPCODE_STMT_AXIOM,
    StSort = OPCODE_STMT_SORT,
    StTermDef = OPCODE_STMT_TERM_DEF,
    StThm = OPCODE_STMT_THM,
    StLocalDef = OPCODE_STMT_LOCAL_DEF,
    StLocalTerm = OPCODE_STMT_LOCAL_TERM,
    PrTerm = OPCODE_PROOF_TERM,
    PrTermSave = OPCODE_PROOF_TERM_SAVE,
    PrRef = OPCODE_PROOF_REF,
    PrDummy = OPCODE_PROOF_DUMMY,
    PrThm = OPCODE_PROOF_THM,
    PrThmSave = OPCODE_PROOF_THM_SAVE,
    PrHyp = OPCODE_PROOF_HYP,
    PrConv = OPCODE_PROOF_CONV,
    PrRefl = OPCODE_PROOF_REFL,
    PrSymm = OPCODE_PROOF_SYMM,
    PrCong = OPCODE_PROOF_CONG,
    PrUnfold = OPCODE_PROOF_UNFOLD,
    PrConvCut = OPCODE_PROOF_CONV_CUT,
    PrConvRef = OPCODE_PROOF_CONV_REF,
    PrConvSave = OPCODE_PROOF_CONV_SAVE,
    UnTerm = OPCODE_UNIFY_TERM,
    UnTermSave = OPCODE_UNIFY_TERM_SAVE,
    UnRef = OPCODE_UNIFY_REF,
    UnDummy = OPCODE_UNIFY_DUMMY,
    UnHyp = OPCODE_UNIFY_HYP,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Command<T> {
    pub opcode: T,
    pub operand: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Statement {
    End = OPCODE_END,
    Axiom = OPCODE_STMT_AXIOM,
    Sort = OPCODE_STMT_SORT,
    TermDef = OPCODE_STMT_TERM_DEF,
    Thm = OPCODE_STMT_THM,
    LocalDef = OPCODE_STMT_LOCAL_DEF,
    LocalTerm = OPCODE_STMT_LOCAL_TERM,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Unify {
    End = OPCODE_END,
    Term = OPCODE_UNIFY_TERM,
    TermSave = OPCODE_UNIFY_TERM_SAVE,
    Ref = OPCODE_UNIFY_REF,
    Dummy = OPCODE_UNIFY_DUMMY,
    Hyp = OPCODE_UNIFY_HYP,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Proof {
    End = OPCODE_END,
    Term = OPCODE_PROOF_TERM,
    TermSave = OPCODE_PROOF_TERM_SAVE,
    Ref = OPCODE_PROOF_REF,
    Dummy = OPCODE_PROOF_DUMMY,
    Thm = OPCODE_PROOF_THM,
    ThmSave = OPCODE_PROOF_THM_SAVE,
    Hyp = OPCODE_PROOF_HYP,
    Conv = OPCODE_PROOF_CONV,
    Refl = OPCODE_PROOF_REFL,
    Symm = OPCODE_PROOF_SYMM,
    Cong = OPCODE_PROOF_CONG,
    Unfold = OPCODE_PROOF_UNFOLD,
    ConvCut = OPCODE_PROOF_CONV_CUT,
    ConvRef = OPCODE_PROOF_CONV_REF,
    ConvSave = OPCODE_PROOF_CONV_SAVE,
}

use core::convert::TryFrom;

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Opcode, ()> {
        use Opcode::*;
        match value {
            OPCODE_END => Ok(End),
            OPCODE_STMT_AXIOM => Ok(StAxiom),
            OPCODE_STMT_SORT => Ok(StSort),
            OPCODE_STMT_TERM_DEF => Ok(StTermDef),
            OPCODE_STMT_THM => Ok(StThm),
            OPCODE_STMT_LOCAL_DEF => Ok(StLocalDef),
            OPCODE_STMT_LOCAL_TERM => Ok(StLocalTerm),
            OPCODE_PROOF_TERM => Ok(PrTerm),
            OPCODE_PROOF_TERM_SAVE => Ok(PrTermSave),
            OPCODE_PROOF_REF => Ok(PrRef),
            OPCODE_PROOF_DUMMY => Ok(PrDummy),
            OPCODE_PROOF_THM => Ok(PrThm),
            OPCODE_PROOF_THM_SAVE => Ok(PrThmSave),
            OPCODE_PROOF_HYP => Ok(PrHyp),
            OPCODE_PROOF_CONV => Ok(PrConv),
            OPCODE_PROOF_REFL => Ok(PrRefl),
            OPCODE_PROOF_SYMM => Ok(PrSymm),
            OPCODE_PROOF_CONG => Ok(PrCong),
            OPCODE_PROOF_UNFOLD => Ok(PrUnfold),
            OPCODE_PROOF_CONV_CUT => Ok(PrConvCut),
            OPCODE_PROOF_CONV_REF => Ok(PrConvRef),
            OPCODE_PROOF_CONV_SAVE => Ok(PrConvSave),
            OPCODE_UNIFY_TERM => Ok(UnTerm),
            OPCODE_UNIFY_TERM_SAVE => Ok(UnTermSave),
            OPCODE_UNIFY_REF => Ok(UnRef),
            OPCODE_UNIFY_DUMMY => Ok(UnDummy),
            OPCODE_UNIFY_HYP => Ok(UnHyp),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Statement {
    type Error = ();

    fn try_from(value: u8) -> Result<Statement, ()> {
        use Statement::*;
        match value {
            OPCODE_END => Ok(End),
            OPCODE_STMT_AXIOM => Ok(Axiom),
            OPCODE_STMT_SORT => Ok(Sort),
            OPCODE_STMT_TERM_DEF => Ok(TermDef),
            OPCODE_STMT_THM => Ok(Thm),
            OPCODE_STMT_LOCAL_DEF => Ok(LocalDef),
            OPCODE_STMT_LOCAL_TERM => Ok(LocalTerm),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Unify {
    type Error = ();

    fn try_from(value: u8) -> Result<Unify, ()> {
        use Unify::*;
        match value {
            OPCODE_END => Ok(End),
            OPCODE_UNIFY_TERM => Ok(Term),
            OPCODE_UNIFY_TERM_SAVE => Ok(TermSave),
            OPCODE_UNIFY_REF => Ok(Ref),
            OPCODE_UNIFY_DUMMY => Ok(Dummy),
            OPCODE_UNIFY_HYP => Ok(Hyp),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Proof {
    type Error = ();

    fn try_from(value: u8) -> Result<Proof, ()> {
        use Proof::*;
        match value {
            OPCODE_END => Ok(End),
            OPCODE_PROOF_TERM => Ok(Term),
            OPCODE_PROOF_TERM_SAVE => Ok(TermSave),
            OPCODE_PROOF_REF => Ok(Ref),
            OPCODE_PROOF_DUMMY => Ok(Dummy),
            OPCODE_PROOF_THM => Ok(Thm),
            OPCODE_PROOF_THM_SAVE => Ok(ThmSave),
            OPCODE_PROOF_HYP => Ok(Hyp),
            OPCODE_PROOF_CONV => Ok(Conv),
            OPCODE_PROOF_REFL => Ok(Refl),
            OPCODE_PROOF_SYMM => Ok(Symm),
            OPCODE_PROOF_CONG => Ok(Cong),
            OPCODE_PROOF_UNFOLD => Ok(Unfold),
            OPCODE_PROOF_CONV_CUT => Ok(ConvCut),
            OPCODE_PROOF_CONV_REF => Ok(ConvRef),
            OPCODE_PROOF_CONV_SAVE => Ok(ConvSave),
            _ => Err(()),
        }
    }
}

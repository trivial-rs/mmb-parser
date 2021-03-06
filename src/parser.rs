use index::Entry;
use nom::bytes::complete;
use nom::number;
use nom::Err;

use crate::index;
use crate::opcode::{Command, Proof, Unify};
use crate::visitor::{ProofStream, UnifyStream, Visitor};
use crate::Mmb;
use crate::{error::*, index::name_table::Name};

const TABLE_ENTRY_SIZE: u64 = 8 * 2;

pub fn parse(input: &[u8]) -> IResult<Mmb> {
    let (i, _) = take_magic(input)?;

    let (i, version) = number::complete::le_u8(i)?;
    let (i, num_sorts) = number::complete::le_u8(i)?;
    let (i, _padding) = complete::take(2u8)(i)?;
    let (i, num_terms) = number::complete::le_u32(i)?;
    let (i, num_theorems) = number::complete::le_u32(i)?;
    let (i, terms_ptr) = number::complete::le_u32(i)?;
    let (i, theorems_ptr) = number::complete::le_u32(i)?;
    let (i, proofs_ptr) = number::complete::le_u32(i)?;

    let (i, _padding) = complete::take(4u8)(i)?;

    let (i, index_ptr) = number::complete::le_u64(i)?;
    let (i, sorts) = complete::take(num_sorts)(i)?;

    let index = if index_ptr != 0 {
        let (index, _) = complete::take(index_ptr as usize)(input)?;

        let (j, num) = number::complete::le_u64(index)?;
        let (_, entries) = complete::take(num * TABLE_ENTRY_SIZE)(j)?;

        let index = index::Index {
            file: input,
            num_sorts,
            num_terms,
            num_theorems,
            num_entries: num,
            entries,
        };
        /*
        for _ in 0..num {
            let (k, id) = number::complete::le_u32(j)?;
            let (k, _padding) = number::complete::le_u32(k)?;
            let (k, ptr) = number::complete::le_u64(k)?;

            /*
            if id == 0x656d614e {
                let (kk, ptr) = number::complete::le_u64(ptr)?;
                //
            }
            */

            println!("{:x}, {}, {}", id, _padding, ptr);
            j = k;
        }


        let (j, sorts) = complete::take(num_sorts * 8)(j)?;
        let (j, terms) = complete::take(num_terms * 8)(j)?;
        let (_, theorems) = complete::take(num_theorems * 8)(j)?;

        let index = index::Index {
            root_bst_ptr,
            file: input,
            sorts,
            terms,
            theorems,
        };
        Some(index)
        */

        Some(index)
    } else {
        None
    };

    let (proofs, _) = complete::take(proofs_ptr as usize)(input)?;

    let (terms, _) = complete::take(terms_ptr as usize)(input)?;
    let (_, terms) = complete::take(num_terms as usize * 8)(terms)?;

    let (theorems, _) = complete::take(theorems_ptr as usize)(input)?;
    let (_, theorems) = complete::take(num_theorems as usize * 8)(theorems)?;

    Ok((
        i,
        Mmb {
            file: input,
            version,
            num_sorts,
            num_terms,
            num_theorems,
            sorts,
            terms,
            theorems,
            proofs,
            index,
        },
    ))
}

pub fn parse_index_entry<'a>(entries: &'a [u8]) -> IResult<'a, Entry> {
    let (left, id) = number::complete::le_u32(entries)?;
    // TODO: check if padding is zero?
    let (left, _padding) = number::complete::le_u32(left)?;
    let (left, ptr) = number::complete::le_u64(left)?;

    let entry = Entry { id, ptr };

    Ok((left, entry))
}

const NAME_ENTRY_SIZE: u64 = 8 * 2;

pub fn parse_name_entries<'a>(file: &'a [u8], num: u64, ptr: u64) -> IResult<'a, &'a [u8]> {
    let num = num * NAME_ENTRY_SIZE;
    let (entries, _) = complete::take(ptr as usize)(file)?;
    let (left, entries) = complete::take(num)(entries)?;

    Ok((left, entries))
}

pub fn seek_name_entry<'a>(file: &'a [u8], entries: &'a [u8], idx: u64) -> IResult<'a, Name<'a>> {
    let (entry, _) = complete::take(idx * NAME_ENTRY_SIZE)(entries)?;
    let (left, name) = parse_name_entry(file, entry)?;

    Ok((left, name))
}

pub fn parse_name_entry<'a>(file: &'a [u8], entry: &'a [u8]) -> IResult<'a, Name<'a>> {
    let (left, ptr) = number::complete::le_u64(entry)?;
    let (left, name_ptr) = number::complete::le_u64(left)?;

    let (name, _) = complete::take(name_ptr as usize)(file)?;
    let (_, name) = parse_nul_terminated_slice(name)?;

    let name = Name { ptr, name };

    Ok((left, name))
}

pub fn subslice_name_table<'a>(entries: &'a [u8], from: u64, len: u64) -> IResult<'a, &'a [u8]> {
    let from = from * NAME_ENTRY_SIZE;
    let len = len * NAME_ENTRY_SIZE;

    let (left, _) = complete::take(from)(entries)?;
    let (left, subslice) = complete::take(len)(left)?;

    Ok((left, subslice))
}

fn parse_binders<'a, T: From<u64>>(input: &'a [u8], slice: &mut [T]) -> IResult<'a, ()> {
    let mut left = input;

    for e in slice {
        let (i, n) = number::complete::le_u64(left)?;
        left = i;
        *e = From::from(n);
    }

    Ok((left, ()))
}

fn parse_nul_terminated_slice(i: &[u8]) -> IResult<'_, &[u8]> {
    let (_, len) = complete::take_till(|c| c == 0)(i)?;
    let len = len.len();

    complete::take(len)(i)
}

fn parse_term<'a, V: Visitor<'a>>(
    file: &'a [u8],
    input: &'a [u8],
    visitor: &mut V,
) -> IResult<'a, ()> {
    let (i, num_args) = number::complete::le_u16(input)?;
    let (i, sort) = number::complete::le_u8(i)?;
    let (i, _padding) = complete::take(1usize)(i)?;
    let (i, ptr_binders) = number::complete::le_u32(i)?;

    let (binders, _) = complete::take(ptr_binders as usize)(file)?;
    let (ret_ty, binders) = complete::take(num_args as usize * 8)(binders)?;
    let (opt_unify, ret_ty) = complete::take(8usize)(ret_ty)?;

    let (offset, ret_ty) = {
        let (slice, offset) = visitor
            .try_reserve_binder_slice(num_args as usize)
            .ok_or(Err::Error(ParseError(input, ErrorType::Memory)))?;

        let (_, _) = parse_binders(binders, slice)?;

        let (_, ret_ty) = number::complete::le_u64(ret_ty)?;

        (offset, ret_ty)
    };

    let stream = visitor.start_unify_stream();

    let unify = if (sort & 0x80) == 0x80 {
        // is definition
        let (_, unify) = take_unify_until_end(opt_unify, stream)?;
        unify
    } else {
        (Default::default(), 0usize)
    };

    let unify_indices = stream.done();

    visitor.parse_term(
        sort,
        (offset, offset + num_args as usize),
        From::from(ret_ty),
        unify.0,
        unify_indices,
    );

    Ok((i, ()))
}

pub fn parse_terms<'a, V: Visitor<'a>>(
    file: &'a [u8],
    input: &'a [u8],
    num_terms: usize,
    visitor: &mut V,
) -> IResult<'a, ()> {
    let mut parse_term = |i| parse_term(file, i, visitor);

    let mut left = input;

    for _ in 0..num_terms {
        let (i, _) = parse_term(left)?;

        if left.is_empty() {
            break;
        }

        left = i;
    }

    Ok((left, ()))
}

fn parse_theorem<'a, V: Visitor<'a>>(
    file: &'a [u8],
    input: &'a [u8],
    visitor: &mut V,
) -> IResult<'a, ()> {
    let (i, num_args) = number::complete::le_u16(input)?;
    let (i, _padding) = complete::take(2usize)(i)?;
    let (i, ptr_binders) = number::complete::le_u32(i)?;

    let (binders, _) = complete::take(ptr_binders as usize)(file)?;
    let (unify, binders) = complete::take(num_args as usize * 8)(binders)?;

    let offset = {
        let (slice, offset) = visitor
            .try_reserve_binder_slice(num_args as usize)
            .ok_or(Err::Error(ParseError(input, ErrorType::Memory)))?;

        let (_, _) = parse_binders(binders, slice)?;

        offset
    };

    let stream = visitor.start_unify_stream();

    let (_, unify) = take_unify_until_end(unify, stream)?;

    let unify_indices = stream.done();

    visitor.parse_theorem((offset, offset + num_args as usize), unify.0, unify_indices);

    Ok((i, ()))
}

pub fn parse_sorts<'a, V: Visitor<'a>>(
    input: &'a [u8],
    num_sorts: u8,
    visitor: &mut V,
) -> IResult<'a, ()> {
    let mut left = input;

    for _ in 0..num_sorts {
        let (i, n) = number::complete::le_u8(left)?;

        visitor.parse_sort(From::from(n));

        if left.is_empty() {
            break;
        }

        left = i;
    }

    Ok((left, ()))
}

pub fn parse_theorems<'a, V: Visitor<'a>>(
    file: &'a [u8],
    input: &'a [u8],
    num_theorems: usize,
    visitor: &mut V,
) -> IResult<'a, ()> {
    let mut parse_theorem = |i| parse_theorem(file, i, visitor);

    let mut left = input;

    for _ in 0..num_theorems {
        let (i, _) = parse_theorem(left)?;

        if left.is_empty() {
            break;
        }

        left = i;
    }

    Ok((left, ()))
}

fn parse_skip(input: &[u8]) -> IResult<u32> {
    let (ii, opcode) = number::complete::le_u8(input)?;

    if opcode & 0x3F == 0x00 {
        return Err(Err::Error(ParseError(input, ErrorType::InvalidCommand)));
    }

    match opcode & 0xC0 {
        0x00 => Ok((input, 0)),
        0x40 => {
            let (_, operand) = number::complete::le_u8(ii)?;
            Ok((input, operand as u32))
        }
        0x80 => {
            let (_, operand) = number::complete::le_u16(ii)?;
            Ok((input, operand as u32))
        }
        0xC0 => {
            let (_, operand) = number::complete::le_u32(ii)?;
            Ok((input, operand))
        }
        _ => unreachable!("impossible"),
    }
}

use core::convert::TryFrom;

pub fn parse_opcode<T: TryFrom<u8>>(input: &[u8]) -> IResult<Command<T>> {
    let (i, opcode) = number::complete::le_u8(input)?;
    let (i, (operand, _size)) = parse_operand(i, opcode)?;

    let opcode: T = (opcode & 0x3F)
        .try_into()
        .map_err(|_| Err::Error(ParseError(input, ErrorType::InvalidCommand)))?;

    let c = Command { opcode, operand };

    Ok((i, c))
}

pub fn scan_statement_stream<'a, V: Visitor<'a>>(
    input: &'a [u8],
    visitor: &mut V,
) -> IResult<'a, &'a [u8]> {
    let x = nom::multi::length_data(parse_skip);
    let mut left = input;
    let mut len = 0;

    loop {
        match x(left) {
            Ok((i, o)) => {
                let (opt_proof, command) = parse_opcode(o)?;

                let indices = if opt_proof.is_empty() {
                    None
                } else {
                    let stream = visitor.start_proof_stream();
                    take_proof_until_end(opt_proof, stream)?;
                    Some(stream.done())
                };

                visitor.parse_statement(From::from(command.opcode), len, o, indices);

                len += o.len();
                left = i;
            }
            Err(Err::Error(_)) => {
                let (_, proofs) = complete::take(len)(input)?;

                return Ok((left, proofs));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn take_proof_until_end<'a, S: ProofStream>(input: &'a [u8], stream: &mut S) -> IResult<'a, ()> {
    let mut i = input;

    loop {
        let (left, command) = parse_opcode(i)?;
        i = left;

        stream.push(command);

        if let Proof::End = command.opcode {
            break;
        }
    }

    Ok((i, ()))
}

fn take_magic(input: &[u8]) -> IResult<()> {
    let (rem, _) = complete::tag([0x4d, 0x4d, 0x30, 0x42])(input)?;

    Ok((rem, ()))
}

use core::convert::TryInto;

/*
#[derive(Debug, Copy, Clone)]
pub struct Command {
    opcode: Opcode,
    size: u8,
    operand: u32,
}
*/

pub fn parse_unify_opcode(input: &[u8]) -> IResult<Command<Unify>> {
    let (i, opcode) = number::complete::le_u8(input)?;
    let (i, (operand, _size)) = parse_operand(i, opcode)?;

    let opcode: Unify = (opcode & 0x3F)
        .try_into()
        .map_err(|_| Err::Error(ParseError(input, ErrorType::InvalidCommand)))?;

    let c = Command { opcode, operand };

    Ok((i, c))
}

fn take_unify_until_end<'a, S: UnifyStream>(
    input: &'a [u8],
    stream: &mut S,
) -> IResult<'a, (&'a [u8], usize)> {
    let mut i = input;
    let mut counter = 0;

    loop {
        let (left, command) = parse_unify_opcode(i)?;
        i = left;
        counter += 1;

        stream.push(From::from(command));

        if let Unify::End = command.opcode {
            break;
        }
    }

    let (x, code) = complete::take(input.len() - i.len())(input)?;

    Ok((x, (code, counter)))
}

fn parse_operand<'a, E: nom::error::ParseError<&'a [u8]>>(
    input: &'a [u8],
    opcode: u8,
) -> nom::IResult<&'a [u8], (u32, u8), E> {
    match opcode & 0xC0 {
        0x00 => Ok((input, (0, 0))),
        0x40 => {
            let (i, operand) = number::complete::le_u8(input)?;
            Ok((i, (operand as u32, 1)))
        }
        0x80 => {
            let (i, operand) = number::complete::le_u16(input)?;
            Ok((i, (operand as u32, 2)))
        }
        0xC0 => {
            let (i, operand) = number::complete::le_u32(input)?;
            Ok((i, (operand, 4)))
        }
        _ => unreachable!("impossible"),
    }
}

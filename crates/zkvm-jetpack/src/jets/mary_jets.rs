use nockvm::interpreter::Context;
use nockvm::jets::util::{bite_to_word, bits_to_word, chop, slot};
use nockvm::jets::JetErr;
use nockvm::noun::{Atom, IndirectAtom, Noun, D, NO, T, YES};
use nockvm::jets::list::util::{lent, reap};
use nockvm::jets::bits::util::{lsh, met};
use nockvm::jets::math::util::add;
use nockvm::mem::NockStack;
use tracing::{debug,error};
use nockvm::jets::bits::jet_xeb;
use crate::form::Belt;
use crate::form::mary::*;
use crate::form::math::mary::*;
use crate::form::tip5::DIGEST_LENGTH;
use crate::hand::handle::{finalize_mary, finalize_poly, new_handle_mut_mary, new_handle_mut_slice};
use crate::hand::structs::HoonList;
use crate::jets::base_jets::{levy_based, rip_correct};
use crate::jets::bp_jets::init_bpoly;
use crate::jets::shape_jets::leaf_sequence_jet;
use crate::jets::tip5_jets::digest_to_noundigest;
use crate::jets::utils::jet_err;
use crate::noun::noun_ext::AtomExt;
use crate::utils::u128_as_noun;

pub fn mary_swag_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let door = slot(subject, 7)?;
    let ma = slot(door, 6)?;
    let sam = slot(subject, 6)?;
    let i = sam.as_cell()?.head().as_direct()?.data() as usize;
    let j = sam.as_cell()?.tail().as_direct()?.data() as usize;

    let Ok(mary) = MarySlice::try_from(ma) else {
        debug!("cannot convert mary arg to mary");
        return jet_err();
    };

    let (res, res_poly): (IndirectAtom, MarySliceMut) =
        new_handle_mut_mary(&mut context.stack, mary.step as usize, j);
    let step = mary.step as usize;

    res_poly
        .dat
        .copy_from_slice(&mary.dat[(i * step)..(i + j) * step]);

    let res_cell = finalize_mary(&mut context.stack, step, j, res);
    Ok(res_cell)
}

pub fn mary_weld_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let door = slot(subject, 7)?;
    let ma = slot(door, 6)?;
    let ma2 = slot(subject, 6)?;

    let step = ma.as_cell()?.head().as_direct()?.data() as u32;
    let step2 = ma2.as_cell()?.head().as_direct()?.data() as u32;
    if step != step2 {
        debug!("can only weld marys of same step");
        return jet_err();
    }

    let (Ok(mary1), Ok(mary2)) = (MarySlice::try_from(ma), MarySlice::try_from(ma2)) else {
        debug!("mary1 or mary2 is not an fpoly");
        return jet_err();
    };
    let res_len = mary1.len + mary2.len;
    let (res, res_poly): (IndirectAtom, MarySliceMut) =
        new_handle_mut_mary(&mut context.stack, step as usize, res_len as usize);

    mary_weld(mary1, mary2, res_poly);
    let res_cell = finalize_mary(&mut context.stack, step as usize, res_len as usize, res);
    Ok(res_cell)
}

pub fn mary_transpose_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let door = slot(subject, 7)?;
    let ma = slot(door, 6)?;
    let offset = slot(subject, 6)?;

    let (Ok(mary), Ok(offset)) = (MarySlice::try_from(ma), offset.as_atom()?.as_u64()) else {
        debug!("fp is not an fpoly or n is not an atom");
        return jet_err();
    };

    let offset = offset as usize;

    let (res, mut res_poly): (IndirectAtom, MarySliceMut) = new_handle_mut_mary(
        &mut context.stack,
        mary.len as usize * offset,
        mary.step as usize / offset,
    );

    mary_transpose(mary, offset, &mut res_poly);

    let res_cell = finalize_mary(
        &mut context.stack, res_poly.step as usize, res_poly.len as usize, res,
    );

    Ok(res_cell)
}

pub fn lift_elt_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let stack = &mut context.stack;
    let door = slot(subject, 7)?;
    let step = slot(door, 6)?.as_atom()?.as_u64()?;
    let a = slot(subject, 6)?;

    if step == 1u64 {
        Ok(a)
    } else {
        let reap_res = reap(stack, step-1, D(0))?;
        let init_bpoly_arg = T(stack, &[a, reap_res]);
        let init_bpoly_arg_list = HoonList::try_from(init_bpoly_arg)?;

        let count = init_bpoly_arg_list.count();
        let (res, res_poly): (IndirectAtom, &mut [Belt]) = new_handle_mut_slice(stack, Some(count));
        init_bpoly(init_bpoly_arg_list, res_poly);

        let res_cell = finalize_poly(stack, Some(res_poly.len()), res);
        Ok(res_cell.as_cell()?.tail())
    }
}

pub fn fet_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let stack = &mut context.stack;
    let door = slot(subject, 7)?;
    let step = slot(door, 6)?.as_atom()?.as_u64()?;
    let a = slot(subject, 6)?.as_atom()?;

    let v = rip_correct(stack, 6, 1, a)?;

    let lent_v = lent(v)? as u64;

    if ((lent_v==1) && (step == 1)) || (lent_v==(step+1)) && levy_based(v) {
        Ok(YES)
    } else {
        Ok(NO)
    }
}

pub fn transpose_bpolys_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let sam = slot(subject, 6)?;
    let bpolys = MarySlice::try_from(sam).expect("cannot convert bpolys arg");
    transpose_bpolys(context, bpolys)
}

fn transpose_bpolys(context: &mut Context, bpolys: MarySlice) -> Result<Noun, JetErr> {
    let offset = 1;

    let (res, mut res_poly): (IndirectAtom, MarySliceMut) = new_handle_mut_mary(
        &mut context.stack,
        bpolys.len as usize * offset,
        bpolys.step as usize / offset,
    );

    mary_transpose(bpolys, offset, &mut res_poly);

    let res_cell = finalize_mary(
        &mut context.stack,
        res_poly.step as usize,
        res_poly.len as usize,
        res,
    );

    Ok(res_cell)
}

pub fn snag_one_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let stack = &mut context.stack;
    let door = slot(subject, 7)?;
    let mary_noun = slot(door, 6)?;
    let i = slot(subject, 6)?.as_direct()?.data() as usize;

    snag_one(stack, mary_noun, i)
}


pub fn snag_one(stack: &mut NockStack, mary_noun: Noun, i: usize) -> Result<Noun, JetErr> {
    let mary_cell = mary_noun.as_cell()?;
    let ma_step = mary_cell.head().as_atom()?.as_u32()?;
    let ma_len = mary_cell.tail().as_cell()?.head().as_atom()?.as_u32()?;
    let ma_dat: Atom = mary_cell.tail().as_cell()?.tail().as_atom()?;

    assert!(i < ma_len as usize);

    let res = cut(stack, 6, i * ma_step as usize, ma_step as usize, ma_dat)?;
    if ma_step == 1 { return Ok(res); }
    let high_bit = lsh(stack, 0, bex(6) * ma_step as usize, D(1).as_atom()?)?;

    Ok(add(stack, high_bit.as_atom()?, res.as_atom()?).as_noun())
}

// cut from hoon-138
fn cut(stack: &mut NockStack, bloq: usize, start: usize, run: usize, atom: Atom) -> Result<Noun,JetErr> {
    if run == 0 {
        return Ok(D(0));
    }

    let new_indirect = unsafe {
        let (mut new_indirect, new_slice) =
            IndirectAtom::new_raw_mut_bitslice(stack, bite_to_word(bloq, run)?);
        chop(bloq, start, run, 0, new_slice, atom.as_bitslice())?;
        new_indirect.normalize_as_atom()
    };
    Ok(new_indirect.as_noun())
}
fn bex(arg: usize) -> usize {
    if arg >= 63 {
        error!("simple bex implementation only valid for arg <63 !!");
    }
    1 << arg
}

pub fn snag_as_bpoly_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let stack = &mut context.stack;
    let door = slot(subject, 7)?;
    let mary_noun = slot(door, 6)?;
    let i = slot(subject, 6)?.as_direct()?.data() as usize;

    snag_as_bpoly(stack, mary_noun, i)
}

fn snag_as_bpoly(stack: &mut NockStack, mary_noun: Noun, i: usize) -> Result<Noun, JetErr> {
    let mary_cell = mary_noun.as_cell()?;
    let ma_step = mary_cell.head().as_atom()?.as_u32()?;

    let dat = snag_one(stack, mary_noun, i)?;

    if ma_step == 1 {
        let step = bex(6) * ma_step as usize;
        let high_bit = lsh(stack, 0, step, D(1).as_atom()?)?;
        let res_add = add(stack, high_bit.as_atom()?, dat.as_atom()?).as_noun();
        return Ok(T(stack, &[D(ma_step as u64), res_add]));
    }

    Ok(T(stack, &[D(ma_step as u64), dat]))
}




pub fn do_bp_build_merk_heap_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let stack = &mut context.stack;
    let mary_noun = slot(subject, 6)?;

    let m = MarySlice::try_from(mary_noun).expect("cannot convert m arg");
    let heap_mary = heapify_mary(stack, m, mary_noun)?;
    let xeb_m = simple_xeb(m.len as usize);

    let snag_digest = snag_as_digest( stack, heap_mary, 0)?;

    let res1 = T(stack, &[ snag_digest, heap_mary ]);
    let res = T(stack, &[ D(xeb_m as u64), res1]);
    Ok(res)
}

fn simple_xeb(n : usize) -> usize {
    if n == 0 {
        0
    } else {
        (64 - n.leading_zeros()) as usize
    }
}

fn heapify_mary(stack: &mut NockStack, m:MarySlice, m_noun: Noun ) -> Result<Noun, JetErr> {
    // |=  m=mary
    // ^-  mary

    // =/  size  (dec (bex (xeb len.array.m)))
    let size = simple_xeb(m.len as usize);

    // calc high-bit
    // =/  high-bit  (lsh [6 (mul size 5)] 1)
    let high_bit = lsh(stack, 6, size*5, D(1).as_atom()?)?.as_atom()?;
    // let step = size * 5;
    // let a = D(1).as_atom().unwrap();
    // let len = 1;
    // let au64 = 1u64;
    // let abitsize = (64 - au64.leading_zeros()) as usize;
    // let new_size = bits_to_word(abitsize + (6<<step))?;
    // let (mut atom, dest) = IndirectAtom::new_raw_mut_bitslice(stack, new_size);
    // unsafe { chop(6, 0, len, step, dest, a.as_bitslice())?; }
    // let high_bit = atom.as_u64()?;

    // make leaves

    // =/  res=(list (list @))
    //   %+  turn
    //     (range len.array.m)
    //   |=  i=@
    //   =/  t  (~(snag-as-bpoly ave m) i)
    //   (leaf-sequence:shape (hash-hashable:tip5 (hashable-bpoly:tip5 t)))

    let mut res:Vec<Noun> = Vec::new();
    for i in 0 .. m.len+1 {
        let t=snag_as_bpoly(stack, m_noun, i as usize)?;



        res.push(t);
    }



    // :+  5
    //   size
    // %+  add
    //   high-bit
    // %+  rep  6
    // %-  zing
    // ^-  (list (list @))
    // =/  curr  res
    //
    //
    // |-
    // ?:  =((lent curr) 1)
    //   res
    // ~&  %do-bp-build-merk-heap-heapify-mary-pairs
    // =/  pairs  (hash-pairs:tip5 curr)
    // %=  $
    //   res      (weld pairs res)
    //   curr     pairs
    // ==



    jet_err()
}


pub fn snag_as_digest_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let stack = &mut context.stack;
    let sam = slot(subject, 6)?;
    let m_noun = slot(sam, 2)?;
    let i_noun = slot(sam, 3)?;

    let i = i_noun.as_atom()?.as_u64()? as usize;
    snag_as_digest( stack, m_noun, i)
}

fn snag_as_digest(stack: &mut NockStack, m_noun: Noun, i: usize) -> Result<Noun, JetErr>{
    let buf = snag_one(stack, m_noun, i)?.as_atom()?;

    let mut digest = [0u64; DIGEST_LENGTH];
    digest[0] = cut(stack, 6, 0, 1, buf)?.as_atom()?.as_u64()?;
    digest[1] = cut(stack, 6, 1, 1, buf)?.as_atom()?.as_u64()?;
    digest[2] = cut(stack, 6, 2, 1, buf)?.as_atom()?.as_u64()?;
    digest[3] = cut(stack, 6, 3, 1, buf)?.as_atom()?.as_u64()?;
    digest[4] = cut(stack, 6, 4, 1, buf)?.as_atom()?.as_u64()?;

    Ok(digest_to_noundigest(stack, digest))
}




use crate::form::{Belt, Felt};
use crate::jets::tip5_jets::assert_all_based;
use crate::utils::hoon_list_to_vecbelt;
use ibig::UBig;
use nockvm::interpreter::Context;
use nockvm::jets::util::slot;
use nockvm::jets::JetErr;
use nockvm::noun::{Atom, Noun};
use std::ops::{BitOr, Shl};

const DEG:u64 = 3; // field extension degree


// frep: inverse of frip; list of belts are rep'd to a felt
fn frep(x: Vec<Belt>) -> Result<Felt, JetErr> {
    assert_eq!(x.len() as u64, DEG);
    assert_all_based(&x);
    Ok(felt_from_u64s(x[0].0,x[1].0,x[2].0))
}

// build felt from 3 given u64s
fn felt_from_u64s( x0:u64, x1:u64, x2:u64) -> Felt {
    let data : [u64; 3] = [x0,x1,x2];
    Felt::from(data)
}

// create a noun of a felt
fn felt_as_noun(context: &mut Context, felt:Felt) -> Result<Noun, JetErr> {
    let res_big =
        UBig::from(felt[0].0).shl(0)
            .bitor(UBig::from(felt[1].0).shl(64))
            .bitor(UBig::from(felt[2].0).shl(128))
            .bitor(UBig::from(1u64).shl(192));
    Ok(Atom::from_ubig(&mut context.stack, &res_big).as_noun())
}

// frep_jet
pub fn frep_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let sample = slot(subject, 6)?;
    let x = hoon_list_to_vecbelt(sample)?;
    let felt = frep(x)?;
    felt_as_noun(context, felt)
}


#[cfg(test)]
mod tests {
    use super::*;
    use nockvm::jets::util::test::*;
    use nockvm::noun::{D, T};

    #[test]
    fn test_frep_jet() {
        let c = &mut init_context();

        // > (frep.two ~[1 2 3])
        // 0x1.0000.0000.0000.0003.0000.0000.0000.0002.0000.0000.0000.0001
        let sam = T( &mut c.stack, &[ D(1), D(2), D(3), D(0) ]);
        let res = felt_as_noun(c, felt_from_u64s(1,2,3)).expect("felt_as_noun");
        assert_jet(c, frep_jet, sam, res);

        // > (frep.two ~[154.432.865.123.134.542 252.542.541.761.653.234 354.345.546.134.763.356])
        // 0x1.04ea.e365.951a.b75c.0381.361a.8c60.a9f2.0224.a7df.634f.6c4e
        let sam = T( &mut c.stack, &[ D(154432865123134542), D(252542541761653234), D(354345546134763356), D(0) ]);
        let res = felt_as_noun(c, felt_from_u64s(0x0224a7df634f6c4e, 0x0381361a8c60a9f2, 0x04eae365951ab75c)).expect("felt_as_noun");
        assert_jet(c, frep_jet, sam, res);
    }
}
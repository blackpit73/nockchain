use nockvm::interpreter::Context;
use nockvm::jets::JetErr;
use nockvm::jets::util::slot;
use nockvm::noun::Noun;
use crate::utils::{vec_to_hoon_list};

pub fn leaf_sequence_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let t = slot(subject, 6)?;

    let mut leaf: Vec<u64> = Vec::<u64>::new();
    do_leaf_sequence(t, &mut leaf)?;
    Ok(vec_to_hoon_list(&mut context.stack, &leaf))
}

pub fn do_leaf_sequence(noun: Noun, vec: &mut Vec<u64>) -> Result<(), JetErr> {
    if noun.is_atom() {
        vec.push(noun.as_atom()?.as_u64()?);
        Ok(())
    } else {
        let cell = noun.as_cell()?;
        do_leaf_sequence(cell.head(), vec)?;
        do_leaf_sequence(cell.tail(), vec)?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use nockvm::jets::util::test::*;
    use nockvm::noun::{D, T};

    #[test]
    fn test_mont_reduction_jet() {
        let c = &mut init_context();

            // > (leaf-sequence:shape.zeke 1)
            // ~[1]
            let sam = D(1);
            let res = T( &mut c.stack,&[ D(1), D(0)]);
            assert_jet(c, leaf_sequence_jet, sam, res);

            // > (leaf-sequence:shape.zeke ~)
            // ~[0]
            let sam = D(0);
            let res = T( &mut c.stack,&[ D(0), D(0)]);
            assert_jet(c, leaf_sequence_jet, sam, res);

            // > (leaf-sequence:shape.zeke ~[1 2 3])
            // ~[1 2 3 0]
            let sam = T( &mut c.stack,&[ D(1), D(2), D(3), D(0)]);
            let res = T( &mut c.stack,&[ D(1), D(2), D(3), D(0), D(0)]);
            assert_jet(c, leaf_sequence_jet, sam, res);

            // > (leaf-sequence:shape.zeke [[1 2] 3])
            // ~[1 2 3]
            let t12 = T( &mut c.stack,&[ D(1), D(2)]);
            let sam = T( &mut c.stack,&[ t12, D(3), D(0)]);
            let res = T( &mut c.stack,&[ D(1), D(2), D(3), D(0), D(0)]);
            assert_jet(c, leaf_sequence_jet, sam, res);

            // > (leaf-sequence:shape.zeke [[1 2] 3 [4 5] 6])
            // ~[1 2 3 4 5 6]
            let t12 = T( &mut c.stack,&[ D(1), D(2)]);
            let t45 = T( &mut c.stack,&[ D(4), D(5)]);
            let sam = T( &mut c.stack,&[ t12, D(3), t45, D(6)]);
            let res = T( &mut c.stack,&[ D(1), D(2), D(3), D(4), D(5), D(6), D(0)]);
            assert_jet(c, leaf_sequence_jet, sam, res);
    }
}

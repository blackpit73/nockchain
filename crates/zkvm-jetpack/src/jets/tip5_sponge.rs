// ++  sponge
//   ~%  %sponge  +>  ~
//   |_  sponge=tip5-state

use bitvec::prelude::{BitSlice, Lsb0};
use crate::form::tip5::{permute, STATE_SIZE};
use nockvm::interpreter::Context;
use nockvm::jets::util::slot;
use nockvm::jets::JetErr;
use nockvm::mem::NockStack;
use nockvm::noun::{Atom, Cell, Noun, Slots, D, NONE};
use crate::jets::tip5_jets::{hoon_list_to_sponge, tip5_absorb_input, tip5_assert_all_based, tip5_calc_digest, tip5_calc_q_r, tip5_montify_vecbelt, tip5_pad_vecbelt};
use crate::jets::utils::jet_err;
use crate::utils::{hoon_list_to_vecbelt, vec_to_hoon_list};

// copied from interpreter.rs (fn edit)
fn door_edit(
    stack: &mut NockStack,
    edit_axis_path: u64,
    patch: Noun,
    mut tree: Noun,
) -> Noun {
    let edit_axis = BitSlice::<u64, Lsb0>::from_element(&edit_axis_path);
    
    let mut res = patch;
    let mut dest: *mut Noun = &mut res;
    let mut cursor = edit_axis
        .last_one()
        .expect("0 is not allowed as an edit axis");
    loop {
        if cursor == 0 {
            unsafe {
                *dest = patch;
            }
            break;
        };
        if let Ok(tree_cell) = tree.as_cell() {
            cursor -= 1;
            if edit_axis[cursor] {
                unsafe {
                    let (cell, cellmem) = Cell::new_raw_mut(stack);
                    *dest = cell.as_noun();
                    (*cellmem).head = tree_cell.head();
                    dest = &mut ((*cellmem).tail);
                }
                tree = tree_cell.tail();
            } else {
                unsafe {
                    let (cell, cellmem) = Cell::new_raw_mut(stack);
                    *dest = cell.as_noun();
                    (*cellmem).tail = tree_cell.tail();
                    dest = &mut ((*cellmem).head);
                }
                tree = tree_cell.head();
            }
        } else {
            panic!("Invalid axis for edit");
        };
    }
    res
}



//   ++  absorb
//     ~/  %absorb
//     |=  input=(list belt)
//     ^+  +>.$
//     =*  rng  +>.$
//     |^
//     ::  assert that input is made of base field elements
//     ?>  (levy input based)
//     =/  [q=@ r=@]  (dvr (lent input) rate)
//     ::  pad input with ~[1 0 ... 0] to be a multiple of rate
//     =.  input  (weld input [1 (reap (dec (sub rate r)) 0)])
//     ::  bring input into montgomery space
//     =.  input  (turn input montify)
//     |-
//     =.  sponge  (absorb-rate (scag rate input))
//     ?:  =(q 0)
//       rng
//     $(q (dec q), input (slag rate input))
//     ::
//     ++  absorb-rate
//       |=  input=(list belt)
//       ^+  sponge
//       ?>  =((lent input) rate)
//       =.  sponge  (weld input (slag rate sponge))
//       $:permute
//     --
//   ::
pub fn sponge_absorb_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let input_noun = slot(subject, 6)?;
    let door = slot(subject, 7)?;
    let sponge_noun = slot(door, 6)?;

    let mut input_vec = hoon_list_to_vecbelt(input_noun)?;
    let mut sponge = hoon_list_to_sponge(sponge_noun)?;

    // assert that input is made of base field elements
    tip5_assert_all_based(&input_vec);

    // pad input with ~[1 0 ... 0] to be a multiple of rate
    let (q, r) = tip5_calc_q_r(&input_vec);
    tip5_pad_vecbelt(&mut input_vec, r);

    // bring input into montgomery space
    tip5_montify_vecbelt(&mut input_vec);

    // process input in batches of size RATE
    tip5_absorb_input(&mut input_vec, &mut sponge, q);

    // update sponge in door
    let new_sponge = vec_to_hoon_list(context, &sponge);
    let edit = door_edit(&mut context.stack, 6, new_sponge, door);
    
    Ok(edit)
}

//   ++  permute
//     ~%  %permute  +  ~
//     |.  ^+  sponge
//     (permutation sponge)
//   ::
// pub fn sponge_permute_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
//     let door = slot(subject, 7)?;
//     let sponge_noun = slot(door, 6)?;
//     let mut sponge = hoon_list_to_sponge(sponge_noun)?;
// 
//     permute(&mut sponge);
// 
//     // update sponge in door
//     let new_sponge = vec_to_hoon_list(context, &sponge);
//     let edit = door_edit(&mut context.stack, 6, new_sponge, door);
//     
//     Ok(edit)
// }

//   ++  squeeze
//     ~%  %squeeze  +  ~
//     |.  ^+  [*(list belt) +.$]
//     =*  rng  +.$
//     ::  squeeze out the full rate and bring out of montgomery space
//     =/  output  (turn (scag rate sponge) mont-reduction)
//     =.  sponge  $:permute
//     [output rng]
//   --
// pub fn sponge_squeeze_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
//     let door = slot(subject, 7)?;
//     let sponge_noun = slot(door, 6)?;
//     let sponge = hoon_list_to_sponge(sponge_noun)?;
// 
//     // calc digest
//     //let digest = tip5_calc_digest(&sponge);
//     
//     let mut digest = [0u64; DIGEST_LENGTH];
//     for i in 0..DIGEST_LENGTH {
//         digest[i] = mont_reduction(sponge[i] as u128).0;
//     }
//     digest
//     
// 
//     Ok(vec_to_hoon_list(context, &digest))
// }

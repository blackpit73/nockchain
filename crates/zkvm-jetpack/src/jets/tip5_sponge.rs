// ++  sponge
//   ~%  %sponge  +>  ~
//   |_  sponge=tip5-state

use bitvec::prelude::{BitSlice, Lsb0};
use crate::form::tip5::STATE_SIZE;
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
    edit_axis: &BitSlice<u64, Lsb0>,
    patch: Noun,
    mut tree: Noun,
) -> Noun {
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
    let mut sponge_noun = slot(door, 6)?;

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

    // update sponge in door TODO
    // see interpreter.rs, fn edit

    // let mut i=0;
    // let mut current = sponge_noun;
    // while current.is_cell() {
    //     let cell = current.as_cell()?;
    //     let n = Atom::new(&mut context.stack, sponge[i]).as_noun();
    //     let mut dest: *mut Noun = &mut cell.head();
    //     unsafe { *dest = n; };
    //     current = cell.tail();
    //     i = i + 1;
    // }
    // assert_eq!(i,STATE_SIZE);
    //Ok(NONE)

    let new_sponge = vec_to_hoon_list(context, &sponge);
    // let xdoor = slot(subject, 7)?;
    // let mut sponge_noun = &mut slot(door, 6)?;
    // let x = &door.slot(6)?;
    // //let mut dest: &Noun = x;
    // unsafe { *sponge_noun = new_sponge; };
    //
    // let door = slot(subject, 7)?;
    // let mut sponge_noun = slot(door, 6)?;
    let edit_axis_path:u64 = 6;
    let edit_axis = BitSlice::from_element(&edit_axis_path);
    let edit = door_edit(&mut context.stack, edit_axis, new_sponge, door);

    //Ok(new_sponge)
    //Ok(NONE)
    Ok(edit)
}

//   ++  permute
//     ~%  %permute  +  ~
//     |.  ^+  sponge
//     (permutation sponge)
//   ::
pub fn sponge_permute_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let door = slot(subject, 7)?;
    let sponge = slot(door, 6)?;

    let sponge = [0u64; STATE_SIZE];
    Ok(vec_to_hoon_list(context, &sponge))
}

//   ++  squeeze
//     ~%  %squeeze  +  ~
//     |.  ^+  [*(list belt) +.$]
//     =*  rng  +.$
//     ::  squeeze out the full rate and bring out of montgomery space
//     =/  output  (turn (scag rate sponge) mont-reduction)
//     =.  sponge  $:permute
//     [output rng]
//   --
pub fn sponge_squeeze_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let door = slot(subject, 7)?;
    let sponge_noun = slot(door, 6)?;
    let sponge = hoon_list_to_sponge(sponge_noun)?;

    // calc digest
    let digest = tip5_calc_digest(&sponge);

    Ok(vec_to_hoon_list(context, &digest))
}

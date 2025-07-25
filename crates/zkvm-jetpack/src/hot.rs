use either::Either::*;
use nockvm::jets::hot::{HotEntry, K_138};

use crate::jets::base_jets::*;
use crate::jets::bp_jets::*;
use crate::jets::cheetah_jets::*;
use crate::jets::compute_table_jets_v2::*;
use crate::jets::crypto_jets::*;
use crate::jets::fext_jets::*;
use crate::jets::fp_jets::*;
use crate::jets::fpntt_jets::*;
use crate::jets::mary_jets::*;
use crate::jets::mega_jets::*;
use crate::jets::memory_table_jets_v2::*;
use crate::jets::proof_gen_jets::*;
use crate::jets::shape_jets::*;
use crate::jets::tip5_jets::*;
use crate::jets::tip5_sponge::*;
use crate::jets::trace_gen_jets::*;
use crate::jets::verifier_jets::*;

pub fn produce_prover_hot_state() -> Vec<HotEntry> {
    let mut jets: Vec<HotEntry> = Vec::new();
    jets.extend(BASE_FIELD_JETS);
    jets.extend(BASE_POLY_JETS);
    jets.extend(CURVE_JETS);
    jets.extend(ZTD_JETS);
    jets.extend(KEYGEN_JETS);
    jets.extend(XTRA_JETS);
    jets.extend(EXTENSION_FIELD_JETS);
    jets.extend(ZKVM_TABLE_JETS_V2);
    jets.extend(CUSTOM_LIST_JETS);

    jets
}

pub const ZKVM_TABLE_JETS_V2: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"memory-table-v2"),
            Left(b"funcs"),
            Left(b"extend"),
        ],
        1,
        memory_v2_extend_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"memory-table-v2"),
            Left(b"funcs"),
            Left(b"mega-extend"),
        ],
        1,
        memory_v2_mega_extend_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"compute-table-v2"),
            Left(b"funcs"),
            Left(b"extend"),
        ],
        1,
        compute_v2_extend_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"compute-table-v2"),
            Left(b"funcs"),
            Left(b"mega-extend"),
        ],
        1,
        compute_v2_mega_extend_jet,
    ),
];

pub const XTRA_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"weld"),
        ],
        1,
        mary_weld_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"swag"),
        ],
        1,
        mary_swag_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"proof-lib"),
            Left(b"utils"),
            Left(b"fri"),
            Left(b"table-lib"),
            Left(b"stark-core"),
            Left(b"fock-core"),
            Left(b"pow"),
            Left(b"stark-engine"),
            Left(b"stark-verifier"),
            Left(b"verify-door"),
            Left(b"evaluate-deep"),
        ],
        1,
        evaluate_deep_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"transpose"),
        ],
        1,
        mary_transpose_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"mp-to-mega"),
            Left(b"mpeval"),
        ],
        1,
        mpeval_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"mp-substitute-mega"),
        ],
        1,
        mp_substitute_mega_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"transpose-bpolys"),
        ],
        1,
        transpose_bpolys_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"snag"),
        ],
        1,
        snag_one_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"snag-as-bpoly"),
        ],
        1,
        snag_as_bpoly_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"rip-correct"),
        ],
        1,
        rip_correct_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"based"),
        ],
        1,
        based_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"mary-utils"),
            Left(b"fet"),
        ],
        1,
        fet_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"mary-utils"),
            Left(b"lift-elt"),
        ],
        1,
        lift_elt_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"change-step"),
        ],
        1,
        change_step_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"merkle"),
            Left(b"bp-build-merk-heap"),
        ],
        1,
        bp_build_merk_heap_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"proof-lib"),
            Left(b"utils"),
            Left(b"constraint-util"),
            Left(b"build-tree-data"),
        ],
        1,
        build_tree_data_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"proof-lib"),
            Left(b"utils"),
            Left(b"fri"),
            Left(b"table-lib"),
            Left(b"stark-core"),
            Left(b"precompute-ntts"),
        ],
        1,
        precompute_ntts_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"proof-lib"),
            Left(b"utils"),
            Left(b"fri"),
            Left(b"table-lib"),
            Left(b"stark-core"),
            Left(b"compute-deep"),
        ],
        1,
        compute_deep_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bpeval-lift"),
        ],
        1,
        bpeval_lift_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"based-noun"),
        ],
        1,
        based_noun_jet,
    ),
];

pub const EXTENSION_FIELD_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-shift"),
        ],
        1,
        bp_shift_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-coseword"),
        ],
        1,
        bp_coseword_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"coseword"),
        ],
        1,
        fp_coseword_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fadd"),
        ],
        1,
        fadd_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fsub"),
        ],
        1,
        fsub_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fneg"),
        ],
        1,
        fneg_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fmul"),
        ],
        1,
        fmul_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"finv"),
        ],
        1,
        finv_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fdiv"),
        ],
        1,
        fdiv_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fpow"),
        ],
        1,
        fpow_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"frep"),
        ],
        1,
        frep_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fp-ntt"),
        ],
        1,
        fp_ntt_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"init-fpoly"),
        ],
        1,
        init_fpoly_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fpeval"),
        ],
        1,
        fpeval_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"lift-to-fpoly"),
        ],
        1,
        lift_to_fpoly_jet,
    ),
];

pub const BASE_FIELD_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"badd"),
        ],
        1,
        badd_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bsub"),
        ],
        1,
        bsub_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bneg"),
        ],
        1,
        bneg_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bmul"),
        ],
        1,
        bmul_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ordered-root"),
        ],
        1,
        ordered_root_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpow"),
        ],
        1,
        bpow_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-ntt"),
        ],
        1,
        bp_ntt_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-fft"),
        ],
        1,
        bp_fft_jet,
    ),
];

pub const BASE_POLY_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpoly-to-list"),
        ],
        1,
        bpoly_to_list_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpadd"),
        ],
        1,
        bpadd_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpneg"),
        ],
        1,
        bpneg_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpsub"),
        ],
        1,
        bpsub_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpscal"),
        ],
        1,
        bpscal_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpmul"),
        ],
        1,
        bpmul_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bp-hadamard"),
        ],
        1,
        bp_hadamard_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpdvr"),
        ],
        1,
        bpdvr_jet,
    ),
];

pub const ZTD_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"permutation"),
        ],
        1,
        permutation_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"montify"),
        ],
        1,
        montify_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"montiply"),
        ],
        1,
        montiply_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"mont-reduction"),
        ],
        1,
        mont_reduction_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-varlen"),
        ],
        1,
        hash_varlen_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"shape"),
            Left(b"leaf-sequence"),
        ],
        1,
        leaf_sequence_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"shape"),
            Left(b"dyck"),
        ],
        1,
        dyck_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"snag-as-digest"),
        ],
        1,
        snag_as_digest_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"sponge"),
            Left(b"absorb"),
        ],
        1,
        sponge_absorb_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-belts-list"),
        ],
        1,
        hash_belts_list_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-10"),
        ],
        1,
        hash_10_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"sponge"),
            Left(b"squeeze"),
        ],
        1,
        sponge_squeeze_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-pairs"),
        ],
        1,
        hash_pairs_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-ten-cell"),
        ],
        1,
        hash_ten_cell_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-pairs"),
        ],
        1,
        hash_pairs_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-ten-cell"),
        ],
        1,
        hash_ten_cell_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-noun-varlen"),
        ],
        1,
        hash_noun_varlen_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"hash-hashable"),
        ],
        1,
        hash_hashable_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"mary-to-list"),
        ],
        1,
        mary_to_list_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bp-is-zero"),
        ],
        1,
        bp_is_zero_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"tip5-lib"),
            Left(b"digest-to-atom"),
        ],
        1,
        digest_to_atom_jet,
    ),
];

pub const KEYGEN_JETS: &[HotEntry] = &[(
    &[
        K_138,
        Left(b"one"),
        Left(b"two"),
        Left(b"tri"),
        Left(b"qua"),
        Left(b"pen"),
        Left(b"zeke"),
        Left(b"ext-field"),
        Left(b"misc-lib"),
        Left(b"proof-lib"),
        Left(b"utils"),
        Left(b"fri"),
        Left(b"table-lib"),
        Left(b"stark-core"),
        Left(b"fock-core"),
        Left(b"pow"),
        Left(b"stark-engine"),
        Left(b"zose"),
        Left(b"argon"),
        Left(b"argon2"),
    ],
    1,
    argon2_jet,
)];

pub const CURVE_JETS: &[HotEntry] = &[(
    &[
        K_138,
        Left(b"one"),
        Left(b"two"),
        Left(b"tri"),
        Left(b"qua"),
        Left(b"pen"),
        Left(b"zeke"),
        Left(b"ext-field"),
        Left(b"misc-lib"),
        Left(b"cheetah"),
        Left(b"curve"),
        Left(b"affine"),
        Left(b"ch-scal"),
    ],
    1,
    ch_scal_jet,
)];

pub const CUSTOM_LIST_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"range"),
        ],
        1,
        range_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"zip-roll"),
        ],
        1,
        zip_roll_jet,
    ),
];

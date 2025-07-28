use ark_bn254::Fr;
use ark_ff::{ AdditiveGroup, BigInt };
// use program_sdk::{
//     error::ProgramSdkResult,
//     inputs::ProofInputsFr,
//     prover::{ circuit::get_circuit, generate_proof::generate_proof, setup },
// };

// #[test]
// pub fn test_generate_proof() -> ProgramSdkResult<()> {
//     let ext_amount = Fr::new(BigInt::<4>::from(1_000_000_000u64));
//     let fee = Fr::ZERO;
//     let public_amount = ext_amount - fee;

//     let inputs = ProofInputsFr {
//         root: light_protocol_program::utils::config::,
//         public_amount,
//         tx_integrity_hash: todo!(),
//         nullifier0: todo!(),
//         nullifier1: todo!(),
//         leaf_right: todo!(),
//         leaf_left: todo!(),
//         amount0: todo!(),
//         amount1: todo!(),
//         secret_key0: todo!(),
//         secret_key1: todo!(),
//         randomness0: todo!(),
//         randomness1: todo!(),
//         out_amount0: todo!(),
//         out_amount1: todo!(),
//         out_secret_key0: todo!(),
//         out_secret_key1: todo!(),
//         out_randomness0: todo!(),
//         out_randomness1: todo!(),
//         recipient: todo!(),
//         ext_amount,
//         relayer: todo!(),
//         fee,
//         merkle_tree_pda_pubkey: todo!(),
//         merkle_tree_index: todo!(),
//         encrypted_utxos: todo!(),
//         merkle_path0: todo!(),
//         path_indices0: todo!(),
//         merkle_path1: todo!(),
//         path_indices1: todo!(),
//     };
//     let wtns_path = "";
//     let r1cs_path = "";
//     let pk_path = "";
//     let vk_path = "";
//     let circuit = get_circuit(&inputs, wtns_path, r1cs_path)?;
//     let setup = setup::SetupParams::init_or_load(circuit, pk_path, vk_path)?;
//     //move circuit
//     let (proof, public_inputs) = generate_proof(setup, circuit)?;
//     todo!()
// }

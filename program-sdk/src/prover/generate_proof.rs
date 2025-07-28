use std::sync::Arc;

use crate::{ error::{ ProgramSdkError, ProgramSdkResult }, prover::setup::SetupParams };
use anyhow;
use ark_bn254::{ Bn254, Fr };
use ark_circom::CircomCircuit;
use ark_groth16::{ Groth16, Proof };
use ark_std::rand::thread_rng;

pub fn generate_proof(
    setup: Arc<SetupParams>,
    circuit: CircomCircuit<Fr>
) -> ProgramSdkResult<(Proof<Bn254>, Vec<Fr>)> {
    let (pk, _vk) = setup.get();

    let public_inputs = circuit
        .get_public_inputs()
        .ok_or_else(|| ProgramSdkError::Other(anyhow::anyhow!("Failed to get public inputs")))?;

    // proof generation
    let mut rng = thread_rng();
    let proof: Proof<Bn254> = Groth16::<Bn254>::create_random_proof_with_reduction(
        circuit,
        &pk,
        &mut rng
    )?;

    Ok((proof, public_inputs))
}

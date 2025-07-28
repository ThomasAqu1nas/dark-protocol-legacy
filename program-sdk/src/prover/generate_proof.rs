use std::sync::Arc;

use crate::{
    error::{ProgramSdkError, ProgramSdkResult},
    inputs::{ProofInputsBigInterger256, ProofInputsFr},
    prover::setup::SetupParams,
};
use anyhow;
use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomBuilder, CircomConfig};
use ark_groth16::{Groth16, Proof};
use ark_std::rand::thread_rng;

pub fn generate_proof(
    setup: Arc<SetupParams>,
    inputs: &ProofInputsFr,
    wtns_path: &str,
    r1cs_path: &str,
) -> ProgramSdkResult<(Proof<Bn254>, Vec<Fr>)> {
    let config = CircomConfig::<Fr>::new(wtns_path, r1cs_path)?;

    let mut builder = CircomBuilder::new(config);

    let bigint_inputs: ProofInputsBigInterger256 = inputs.into();
    bigint_inputs.push_inputs(&mut builder);
    let circuit = builder.build()?;
    let (pk, _vk) = setup.get();

    let public_inputs = circuit
        .get_public_inputs()
        .ok_or_else(|| ProgramSdkError::Other(anyhow::anyhow!("Failed to get public inputs")))?;

    // proof generation
    let mut rng = thread_rng();
    let proof: Proof<Bn254> =
        Groth16::<Bn254>::create_random_proof_with_reduction(circuit, &pk, &mut rng)?;

    Ok((proof, public_inputs))
}

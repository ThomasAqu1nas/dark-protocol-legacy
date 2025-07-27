use std::sync::Arc;

use ark_bn254::{ Bn254, Fr };
use ark_circom::{ CircomBuilder, CircomConfig };
use anyhow;
use ark_groth16::{ Groth16, Proof };
use ark_std::rand::thread_rng;
use crate::{ inputs::{ ProofInputsBigInterger256, ProofInputsFr }, prover::setup::SetupParams };

pub fn generate_proof(
    setup: Arc<SetupParams>,
    inputs: &ProofInputsFr,
    wtns_path: &str,
    r1cs_path: &str
) -> anyhow::Result<()> {
    let config = CircomConfig::<Fr>
        ::new(wtns_path, r1cs_path)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut builder = CircomBuilder::new(config);

    let bigint_inputs: ProofInputsBigInterger256 = inputs.into();
    bigint_inputs.push_inputs(&mut builder);
    let circuit = builder.build().map_err(|e| anyhow::anyhow!(e))?;
    let (pk, _vk) = setup.get();

    let public_inputs = circuit.get_public_inputs();

    // proof generation
    let mut rng = thread_rng();
    let proof: Proof<Bn254> = Groth16::<Bn254>
        ::create_random_proof_with_reduction(circuit, &pk, &mut rng)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(())
}

use ark_bn254::Fr;
use ark_circom::{ CircomBuilder, CircomCircuit, CircomConfig };

use crate::{ error::ProgramSdkResult, inputs::{ ProofInputsBigInterger256, ProofInputsFr } };

pub fn get_circuit(
    inputs: &ProofInputsFr,
    wtns_path: &str,
    r1cs_path: &str
) -> ProgramSdkResult<CircomCircuit<Fr>> {
    let config = CircomConfig::<Fr>::new(wtns_path, r1cs_path)?;

    let mut builder = CircomBuilder::new(config);

    let bigint_inputs: ProofInputsBigInterger256 = inputs.into();
    bigint_inputs.push_inputs(&mut builder);
    let circuit = builder.build()?;
    Ok(circuit)
}

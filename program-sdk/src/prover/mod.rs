use ark_bn254::{ Bn254, Fr };
use ark_circom::{ CircomBuilder, CircomConfig };


use anyhow;

use crate::inputs::ProofInputs;

pub fn generate_proof(
    inputs: &ProofInputs<Bn254>,
    wtns_path: &str,
    r1cs_path: &str
) -> anyhow::Result<()> {

    let config = CircomConfig::<Bn254>
        ::new("/circuits/target/witness.wtns", "/circuits/target/universal.r1cs")
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut builder = CircomBuilder::new(config);

    builder.push_input("root", inputs.root);
    Ok(())
}

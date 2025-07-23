
use ark_bn254::Fr;
use ark_circom::{CircomBuilder, CircomConfig};
use anyhow;
pub fn generate_proof() -> anyhow::Result<()> {
    let config = CircomConfig::<Fr>::new(
        "/circuits/target/witness.wtns", 
        "/circuits/target/universal.r1cs"
    ).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let builder = CircomBuilder::new(
        config
    );
    Ok(())
}
use solana_poseidon::PoseidonHash;

pub fn u64x4_to_u8x32(input: [u64; 4]) -> [u8; 32] {
    let mut output = [0u8; 32];
    for (i, val) in input.iter().enumerate() {
        output[i * 8..(i + 1) * 8].copy_from_slice(&val.to_le_bytes());
    }
    output
}

pub fn calculate_tx_integrity_hash() -> PoseidonHash {
    todo!()
}
use ark_bn254::Fr;
use num_bigint::BigInt;

#[derive(Debug)]
pub struct ProofInputs<T: Into<BigInt>> {
    // Публичные сигналы
    pub root: T,
    pub public_amount: T,
    pub tx_integrity_hash: T,
    pub nullifier0: T,
    pub nullifier1: T,
    pub leaf_right: T,
    pub leaf_left: T,
    // Приватные сигналы
    pub amount0: T,
    pub amount1: T,
    pub secret_key0: T,
    pub secret_key1: T,
    pub randomness0: T,
    pub randomness1: T,
    pub out_amount0: T,
    pub out_amount1: T,
    pub out_secret_key0: T,
    pub out_secret_key1: T,
    pub out_randomness0: T,
    pub out_randomness1: T,
    pub recipient: T,
    pub ext_amount: T,
    pub relayer: T,
    pub fee: T,
    pub merkle_tree_pda_pubkey: T,
    pub merkle_tree_index: T,
    pub encrypted_utxos: Vec<T>,
    pub merkle_path0: Vec<T>,
    pub path_indices0: Vec<T>,
    pub merkle_path1: Vec<T>,
    pub path_indices1: Vec<T>,
}

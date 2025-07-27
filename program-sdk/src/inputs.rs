use ark_bn254::{ Fq, Fr };
use ark_circom::CircomBuilder;
use ark_crypto_primitives::sponge::Absorb;
use ark_ff::{ BigInteger256, Field };

#[derive(Debug, Clone)]
pub struct ProofInputsFr {
    // ✅ Публичные сигналы (попадают в publicInputs + on-chain)
    pub root: Fr, // [in] — начальный параметр
    pub public_amount: Fr, // [in] — вычисляется из ext_amount, fee
    pub tx_integrity_hash: Fr, // [computed] ← Poseidon(recipient, ..., encrypted_utxos)
    pub nullifier0: Fr, // [computed] ← Poseidon(secret_key0, 0)
    pub nullifier1: Fr, // [computed] ← Poseidon(secret_key1, 1)
    pub leaf_right: Fr, // [computed] ← Poseidon(out_amount0, out_secret_key0, out_randomness0)
    pub leaf_left: Fr, // [computed] ← Poseidon(out_amount1, out_secret_key1, out_randomness1)

    // 🔒 Приватные сигналы (inputs в circuit, не попадают в пруф)
    pub amount0: Fr, // [in] — начальный параметр
    pub amount1: Fr, // [in] — начальный параметр
    pub secret_key0: Fr, // [in] — начальный параметр
    pub secret_key1: Fr, // [in] — начальный параметр
    pub randomness0: Fr, // [in] — начальный параметр
    pub randomness1: Fr, // [in] — начальный параметр
    pub out_amount0: Fr, // [in] — начальный параметр
    pub out_amount1: Fr, // [in] — начальный параметр
    pub out_secret_key0: Fr, // [in] — начальный параметр
    pub out_secret_key1: Fr, // [in] — начальный параметр
    pub out_randomness0: Fr, // [in] — начальный параметр
    pub out_randomness1: Fr, // [in] — начальный параметр
    pub recipient: Fr, // [in] — начальный параметр
    pub ext_amount: Fr, // [in] — начальный параметр (может быть < 0)
    pub relayer: Fr, // [in] — начальный параметр
    pub fee: Fr, // [in] — начальный параметр
    pub merkle_tree_pda_pubkey: Fr, // [in] — начальный параметр
    pub merkle_tree_index: Fr, // [in] — начальный параметр

    pub encrypted_utxos: Vec<Fr>, // [in] — начальный параметр
    pub merkle_path0: Vec<Fr>, // [in] — Merkle proof for UTXO0
    pub path_indices0: Vec<Fr>, // [in] — Merkle path bits for UTXO0
    pub merkle_path1: Vec<Fr>, // [in] — Merkle proof for UTXO1
    pub path_indices1: Vec<Fr>, // [in] — Merkle path bits for UTXO1
}

#[derive(Debug, Clone)]
pub struct ProofInputsBigInterger256 {
    pub root: BigInteger256,
    pub public_amount: BigInteger256,
    pub tx_integrity_hash: BigInteger256,
    pub nullifier0: BigInteger256,
    pub nullifier1: BigInteger256,
    pub leaf_right: BigInteger256,
    pub leaf_left: BigInteger256,

    pub amount0: BigInteger256,
    pub amount1: BigInteger256,
    pub secret_key0: BigInteger256,
    pub secret_key1: BigInteger256,
    pub randomness0: BigInteger256,
    pub randomness1: BigInteger256,
    pub out_amount0: BigInteger256,
    pub out_amount1: BigInteger256,
    pub out_secret_key0: BigInteger256,
    pub out_secret_key1: BigInteger256,
    pub out_randomness0: BigInteger256,
    pub out_randomness1: BigInteger256,
    pub recipient: BigInteger256,
    pub ext_amount: BigInteger256,
    pub relayer: BigInteger256,
    pub fee: BigInteger256,
    pub merkle_tree_pda_pubkey: BigInteger256,
    pub merkle_tree_index: BigInteger256,

    pub encrypted_utxos: Vec<BigInteger256>,
    pub merkle_path0: Vec<BigInteger256>,
    pub path_indices0: Vec<BigInteger256>,
    pub merkle_path1: Vec<BigInteger256>,
    pub path_indices1: Vec<BigInteger256>,
}

impl ProofInputsBigInterger256 {
    pub fn push_inputs(
        &self,
        circom_builder: &mut CircomBuilder<
            ark_ff::Fp<ark_ff::MontBackend<ark_bn254::FrConfig, 4>, 4>
        >
    ) {
        circom_builder.push_input("root", self.root);
        circom_builder.push_input("public_amount", self.public_amount);
        circom_builder.push_input("tx_integrity_hash", self.tx_integrity_hash);
        circom_builder.push_input("nullifier0", self.nullifier0);
        circom_builder.push_input("nullifier1", self.nullifier1);
        circom_builder.push_input("leaf_right", self.leaf_right);
        circom_builder.push_input("leaf_left", self.leaf_left);

        circom_builder.push_input("amount0", self.amount0);
        circom_builder.push_input("amount1", self.amount1);
        circom_builder.push_input("secret_key0", self.secret_key0);
        circom_builder.push_input("secret_key1", self.secret_key1);
        circom_builder.push_input("randomness0", self.randomness0);
        circom_builder.push_input("randomness1", self.randomness1);
        circom_builder.push_input("out_amount0", self.out_amount0);
        circom_builder.push_input("out_amount1", self.out_amount1);
        circom_builder.push_input("out_secret_key0", self.out_secret_key0);
        circom_builder.push_input("out_secret_key1", self.out_secret_key1);
        circom_builder.push_input("out_randomness0", self.out_randomness0);
        circom_builder.push_input("out_randomness1", self.out_randomness1);
        circom_builder.push_input("recipient", self.recipient);
        circom_builder.push_input("ext_amount", self.ext_amount);
        circom_builder.push_input("relayer", self.relayer);
        circom_builder.push_input("fee", self.fee);
        circom_builder.push_input("merkle_tree_pda_pubkey", self.merkle_tree_pda_pubkey);
        circom_builder.push_input("merkle_tree_index", self.merkle_tree_index);

        for &elt in &self.encrypted_utxos {
            circom_builder.push_input("encrypted_utxos", elt);
        }

        for &elt in &self.merkle_path0 {
            circom_builder.push_input("merkle_path0", elt);
        }

        for &elt in &self.path_indices0 {
            circom_builder.push_input("path_indices0", elt);
        }

        for &elt in &self.merkle_path1 {
            circom_builder.push_input("merkle_path1", elt);
        }

        for &elt in &self.path_indices1 {
            circom_builder.push_input("path_indices1", elt);
        }
    }
}

impl Into<ProofInputsBigInterger256> for &ProofInputsFr {
    fn into(self) -> ProofInputsBigInterger256 {
        ProofInputsBigInterger256 {
            root: self.root.into_int256(),
            public_amount: self.public_amount.into_int256(),
            tx_integrity_hash: self.tx_integrity_hash.into_int256(),
            nullifier0: self.nullifier0.into_int256(),
            nullifier1: self.nullifier1.into_int256(),
            leaf_right: self.leaf_right.into_int256(),
            leaf_left: self.leaf_left.into_int256(),
            amount0: self.amount0.into_int256(),
            amount1: self.amount1.into_int256(),
            secret_key0: self.secret_key0.into_int256(),
            secret_key1: self.secret_key1.into_int256(),
            randomness0: self.randomness0.into_int256(),
            randomness1: self.randomness1.into_int256(),
            out_amount0: self.out_amount0.into_int256(),
            out_amount1: self.out_amount1.into_int256(),
            out_secret_key0: self.out_secret_key0.into_int256(),
            out_secret_key1: self.out_secret_key1.into_int256(),
            out_randomness0: self.out_randomness0.into_int256(),
            out_randomness1: self.out_randomness1.into_int256(),
            recipient: self.recipient.into_int256(),
            ext_amount: self.ext_amount.into_int256(),
            relayer: self.relayer.into_int256(),
            fee: self.fee.into_int256(),
            merkle_tree_pda_pubkey: self.merkle_tree_pda_pubkey.into_int256(),
            merkle_tree_index: self.merkle_tree_index.into_int256(),
            encrypted_utxos: self.encrypted_utxos
                .iter()
                .map(|fq| fq.into_int256())
                .collect(),
            merkle_path0: self.merkle_path0
                .iter()
                .map(|fq| fq.into_int256())
                .collect(),
            path_indices0: self.path_indices0
                .iter()
                .map(|fq| fq.into_int256())
                .collect(),
            merkle_path1: self.merkle_path1
                .iter()
                .map(|fq| fq.into_int256())
                .collect(),
            path_indices1: self.path_indices1
                .iter()
                .map(|fq| fq.into_int256())
                .collect(),
        }
    }
}

pub trait FqToBigInteger256 {
    fn into_int256(self) -> BigInteger256;
}

impl FqToBigInteger256 for Fr {
    fn into_int256(self) -> BigInteger256 {
        BigInteger256::from(self)
    }
}

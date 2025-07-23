use ark_ff::BigInteger256;
use solana_poseidon::PoseidonHash;
use solana_sdk::pubkey::Pubkey;

use crate::{ instruction::{ instruction_indexes, TEMPLATE_DISC }, utils::u64x4_to_u8x32 };
pub struct InitializeNewMerkleTreeDataLayout;
pub struct InitializeUserAccountDataLayout;

#[derive(Debug, Clone, Copy)]
pub struct TransactInstructionDataLayout {
    prefix: [u8; 8],
    instruction_id: u8,
    root: [u8; 32],
    public_amount: [u8; 32], //u64 as BigInteger256 (32 bytes)
    tx_integrity_hash: [u8; 32], //poseidon
    nullifier0: [u8; 32],
    nullifier1: [u8; 32],
    leaf_right: [u8; 32],
    leaf_left: [u8; 32],
    proof: [u8; 256],
    recipient: [u8; 32],
    ext_amount: [u8; 8], //i64
    relayer: [u8; 32],
    fee: [u8; 8], //u64
    merkle_tree_pda_pubkey: [u8; 32],
    merkle_tree_index: u8,
    encrypted_utxos: [u8; 222], // ENCRYPTED_UTXOS_LENGTH = 222
}

pub trait InstructionDataLayout {}

impl InstructionDataLayout for InitializeNewMerkleTreeDataLayout {}
impl InstructionDataLayout for InitializeUserAccountDataLayout {}
impl InstructionDataLayout for TransactInstructionDataLayout {}

impl TransactInstructionDataLayout {
    pub fn new(
        prefix: u64,
        instruction_id: u8,
        root: PoseidonHash,
        public_amount: BigInteger256,
        tx_integrity_hash: PoseidonHash,
        nullifier0: PoseidonHash,
        nullifier1: PoseidonHash,
        leaf_right: PoseidonHash,
        leaf_left: PoseidonHash,
        proof: [u8; 256],
        recipient: Pubkey,
        ext_amount: i64,
        relayer: Pubkey,
        fee: u64,
        merkle_tree_pda_pubkey: Pubkey,
        merkle_tree_index: u8,
        encrypted_utxos: [u8; 222]
    ) -> Self {
        Self {
            prefix: prefix.to_be_bytes(),
            instruction_id,
            root: root.0,
            public_amount: u64x4_to_u8x32(public_amount.0),
            tx_integrity_hash: tx_integrity_hash.0,
            nullifier0: nullifier0.0,
            nullifier1: nullifier1.0,
            leaf_right: leaf_right.0,
            leaf_left: leaf_left.0,
            proof,
            recipient: recipient.to_bytes(),
            ext_amount: ext_amount.to_le_bytes(),
            relayer: relayer.to_bytes(),
            fee: fee.to_le_bytes(),
            merkle_tree_pda_pubkey: merkle_tree_pda_pubkey.to_bytes(),
            merkle_tree_index,
            encrypted_utxos,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = Vec::<u8>::with_capacity(8 * 3 + 1 * 2 + 32 * 10 + 222 + 256);
        data.extend_from_slice(&self.prefix);
        data.push(self.instruction_id);
        data.extend_from_slice(&self.root);
        data.extend_from_slice(&self.public_amount);
        data.extend_from_slice(&self.tx_integrity_hash);
        data.extend_from_slice(&self.nullifier0);
        data.extend_from_slice(&self.nullifier1);
        data.extend_from_slice(&self.leaf_right);
        data.extend_from_slice(&self.leaf_left);
        data.extend_from_slice(&self.proof);
        data.extend_from_slice(&self.recipient);
        data.extend_from_slice(&self.ext_amount);
        data.extend_from_slice(&self.relayer);
        data.extend_from_slice(&self.fee);
        data.extend_from_slice(&self.merkle_tree_pda_pubkey);
        data.push(self.merkle_tree_index);
        data.extend_from_slice(&self.encrypted_utxos);
        data
    }
}

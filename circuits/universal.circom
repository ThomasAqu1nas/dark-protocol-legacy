
pragma circom 2.0.0;

include "circomlib/circuits/poseidon.circom";
include "circomlib/circuits/comparators.circom";
include "./merkleTree.circom";

// Universal circuit for deposit and withdrawal in Dark Protocol
template Transaction(levels, ENC_LEN) {
    // Public signals
    signal input root; // Merkle tree root
    signal input publicAmount; // -7 SOL (deposit) or 6 SOL (withdrawal)
    signal input txIntegrityHash; // Keccak256(recipient, ext_amount, ...)
    signal input nullifier0; // Nullifier for UTXO0
    signal input nullifier1; // Nullifier for UTXO1
    signal input leafRight; // New UTXO (e.g., 4 SOL)
    signal input leafLeft; // New UTXO (e.g., 3 SOL or 0 SOL)

    // Private signals
    signal private input amount0; // Input amount (0 for deposit, 10 SOL for withdrawal)
    signal private input amount1; // 0
    signal private input secretKey0; // Key for UTXO0
    signal private input secretKey1; // Key for UTXO1
    signal private input randomness0; // Randomness for UTXO0
    signal private input randomness1; // Randomness for UTXO1
    signal private input outAmount0; // Output amount (4 SOL)
    signal private input outAmount1; // Output amount (3 SOL or 0 SOL)
    signal private input outSecretKey0; // Key for new UTXO
    signal private input outSecretKey1; // Key for new UTXO
    signal private input outRandomness0; // Randomness for new UTXO
    signal private input outRandomness1; // Randomness for new UTXO
    signal private input recipient; // Recipient pubkey
    signal private input extAmount; // 7 SOL (deposit) or -6 SOL (withdrawal)
    signal private input relayer; // Relayer pubkey
    signal private input fee; // Fee (e.g., 0)
    signal private input merkleTreePdaPubkey; // Merkle tree pubkey
    signal private input merkleTreeIndex; // Merkle tree index
    signal private input encryptedUtxos[ENC_LEN]; // Encrypted UTXOs
    signal private input merklePath0[levels]; // Merkle path for UTXO0
    signal private input pathIndices0[levels]; // Path bits for UTXO0
    signal private input merklePath1[levels]; // Merkle path for UTXO1
    signal private input pathIndices1[levels]; // Path bits for UTXO1
    signal private input isDeposit; // 1 for deposit, 0 for withdrawal

    // 1. Conditional logic for deposit
    component isDepositCheck = IsZero();
    isDepositCheck.in <== isDeposit;
    (amount0 * (1 - isDepositCheck.out)) === 0; // amount0 == 0 for deposit
    (amount1 * (1 - isDepositCheck.out)) === 0; // amount1 == 0 for deposit

    // 2. Input UTXOs
    // UTXO0: Poseidon(amount0, secretKey0, randomness0)
    component hasher0 = Poseidon(3);
    hasher0.inputs[0] <== amount0;
    hasher0.inputs[1] <== secretKey0;
    hasher0.inputs[2] <== randomness0;

    // Nullifier0: Poseidon(secretKey0, 0)
    component nullifier0Hasher = Poseidon(2);
    nullifier0Hasher.inputs[0] <== secretKey0;
    nullifier0Hasher.inputs[1] <== 0;
    nullifier0Hasher.out === nullifier0;

    // Merkle proof for UTXO0 (only for withdrawal)
    component tree0 = MerkleTreeChecker(levels);
    tree0.leaf <== hasher0.out;
    tree0.root <== root;
    for (var i = 0; i < levels; i++) {
        tree0.pathElements[i] <== merklePath0[i] * (1 - isDepositCheck.out); // 0 for deposit
        tree0.pathIndices[i] <== pathIndices0[i] * (1 - isDepositCheck.out);
    }

    // UTXO1: Poseidon(amount1, secretKey1, randomness1)
    component hasher1 = Poseidon(3);
    hasher1.inputs[0] <== amount1;
    hasher1.inputs[1] <== secretKey1;
    hasher1.inputs[2] <== randomness1;

    // Nullifier1: Poseidon(secretKey1, 1)
    component nullifier1Hasher = Poseidon(2);
    nullifier1Hasher.inputs[0] <== secretKey1;
    nullifier1Hasher.inputs[1] <== 1;
    nullifier1Hasher.out === nullifier1;

    // Merkle proof for UTXO1 (only for withdrawal)
    component tree1 = MerkleTreeChecker(levels);
    tree1.leaf <== hasher1.out;
    tree1.root <== root;
    for (var i = 0; i < levels; i++) {
        tree1.pathElements[i] <== merklePath1[i] * (1 - isDepositCheck.out); // 0 for deposit
        tree1.pathIndices[i] <== pathIndices1[i] * (1 - isDepositCheck.out);
    }

    // 3. Output UTXOs
    // leafRight: Poseidon(outAmount0, outSecretKey0, outRandomness0)
    component outHasher0 = Poseidon(3);
    outHasher0.inputs[0] <== outAmount0;
    outHasher0.inputs[1] <== outSecretKey0;
    outHasher0.inputs[2] <== outRandomness0;
    outHasher0.out === leafRight;

    // leafLeft: Poseidon(outAmount1, outSecretKey1, outRandomness1)
    component outHasher1 = Poseidon(3);
    outHasher1.inputs[0] <== outAmount1;
    outHasher1.inputs[1] <== outSecretKey1;
    outHasher1.inputs[2] <== outRandomness1;
    outHasher1.out === leafLeft;

    // 4. Balance check
    amount0 + amount1 === outAmount0 + outAmount1 + publicAmount;

    // 5. txIntegrityHash check (placeholder, should be Keccak256)
    component integrityHasher = Poseidon(7 + ENC_LEN); // Placeholder
    integrityHasher.inputs[0] <== recipient;
    integrityHasher.inputs[1] <== extAmount;
    integrityHasher.inputs[2] <== relayer;
    integrityHasher.inputs[3] <== fee;
    integrityHasher.inputs[4] <== merkleTreePdaPubkey;
    integrityHasher.inputs[5] <== merkleTreeIndex;
    for (var i = 0; i < ENC_LEN; i++) {
        integrityHasher.inputs[6 + i] <== encryptedUtxos[i];
    }
    integrityHasher.out === txIntegrityHash;
}

component main {public [root, publicAmount, txIntegrityHash, nullifier0, nullifier1, leafRight, leafLeft]} = Transaction(18, 8);

pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/poseidon.circom";
include "../node_modules/circomlib/circuits/comparators.circom";
include "./merkleTree.circom";

// Universal circuit for deposit and withdrawal in Dark Protocol
template Transaction(levels, ENC_LEN) {
    // Public signals
    signal input root; // Merkle tree root
    signal input publicAmount; // u64, absolute amount (e.g., 6000000000 lamports)
    signal input txIntegrityHash; // Poseidon hash of transaction inputs
    signal input nullifier0; // Nullifier for UTXO0
    signal input nullifier1; // Nullifier for UTXO1
    signal input leafRight; // New UTXO (right)
    signal input leafLeft; // New UTXO (left)

    // Private signals
    signal private input amount0; // Input amount (0 for deposit, e.g., 6000000000 for withdrawal)
    signal private input amount1; // Input amount (usually 0)
    signal private input secretKey0; // Secret key for UTXO0
    signal private input secretKey1; // Secret key for UTXO1
    signal private input randomness0; // Randomness for UTXO0
    signal private input randomness1; // Randomness for UTXO1
    signal private input outAmount0; // Output amount (e.g., 0 for withdrawal)
    signal private input outAmount1; // Output amount (e.g., 0)
    signal private input outSecretKey0; // Secret key for new UTXO (right)
    signal private input outSecretKey1; // Secret key for new UTXO (left)
    signal private input outRandomness0; // Randomness for new UTXO (right)
    signal private input outRandomness1; // Randomness for new UTXO (left)
    signal private input recipient; // Recipient pubkey (32 bytes)
    signal private input extAmount; // i64, positive for deposit (e.g., 6000000000), negative for withdrawal (e.g., -6000000000)
    signal private input relayer; // Relayer pubkey (32 bytes)
    signal private input fee; // u64, relayer fee (e.g., 100000000)
    signal private input merkleTreePdaPubkey; // Merkle tree pubkey (32 bytes)
    signal private input merkleTreeIndex; // Merkle tree index (u8)
    signal private input encryptedUtxos[ENC_LEN]; // Encrypted UTXOs (256 bytes, ENC_LEN = 8)
    signal private input merklePath0[levels]; // Merkle path for UTXO0
    signal private input pathIndices0[levels]; // Path bits for UTXO0
    signal private input merklePath1[levels]; // Merkle path for UTXO1
    signal private input pathIndices1[levels]; // Path bits for UTXO1

    // 1. Determine deposit or withdrawal
    component isExtAmountPositive = GreaterThan(252);
    isExtAmountPositive.in[0] <== extAmount;
    isExtAmountPositive.in[1] <== 0;
    var isDeposit = isExtAmountPositive.out;

    // For deposit: amount0 = amount1 = 0
    (amount0 * isExtAmountPositive.out) === 0;
    (amount1 * isExtAmountPositive.out) === 0;

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
        tree0.pathElements[i] <== merklePath0[i] * (1 - isExtAmountPositive.out);
        tree0.pathIndices[i] <== pathIndices0[i] * (1 - isExtAmountPositive.out);
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
        tree1.pathElements[i] <== merklePath1[i] * (1 - isExtAmountPositive.out);
        tree1.pathIndices[i] <== pathIndices1[i] * (1 - isExtAmountPositive.out);
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
    // Adjust extAmount for relayer fee in deposits
    signal extAmountAdjusted;
    extAmountAdjusted <== extAmount - fee * isExtAmountPositive.out;
    // publicAmount = |extAmountAdjusted|
    publicAmount === extAmountAdjusted * (2 * isDeposit - 1);
    // Balance: amount0 + amount1 = outAmount0 + outAmount1 + extAmountAdjusted * (1 - 2 * isDeposit)
    amount0 + amount1 === outAmount0 + outAmount1 + (extAmountAdjusted * (1 - 2 * isDeposit));

    // 5. txIntegrityHash check
    component integrityHasher = Poseidon(7 + ENC_LEN);
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
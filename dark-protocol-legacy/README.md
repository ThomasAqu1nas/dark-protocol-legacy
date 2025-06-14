# Dark Protocol V1

This is the Dark Protocol V1 (Legacy) Repository which represents a Fork of the Light Protocol V1 on-chain program for Shielded Pools - We have introduced a few non audit breaking changes that optimise transaction submission aswel as allow for the deposit of multiple SPL tokens. 

For the deployed code see the deploy branch which features the right config file and a non-rentexempt temporary storage account to save costs.

## Running Tests
- cargo test-bpf deposit_should_succeed
- cargo test-bpf withdrawal_should_succeed

Important: Run tests selectively test-bpf crashes sometimes if tests run in parallel.

## General Description of Dark-V1 

The Light Protocol forked program verifies zkSNARK proofs to enable anonymous transactions on Solana.

An updated Dark-Protocol focused SDK will follow soon. Developers will be able to build solana-based applications on top of private transactions.
If you're a developer interested in using or integrating with the program, reach out to us: [Discord community](https://discord.gg/WDAAaX6je2)  /  [Twitter](https://twitter.com/privateLP)

Zero-knowledge proofs verify that the owner of recipient address B has deposited tokens to a shielded pool (similar to Zcash) from another address A before.
The zero-knowledge proof includes meta data such as the recipient address. In case this data is tampered with the zero-knowledge proof becomes invalid and the withdrawal fails. Therefore, Dark Protocol is completely trustless.

### Documentation

For additional documentation refer to DOCUMENTATION.md.

### State

We keep state in four types of accounts, a large persistent account for a sparse merkle tree, many small accounts storing two leaves each, many small accounts storing one nullifier each, plus temporary accounts to store intermediate results of computation for SNARK verification and updating the Merkle tree root. For every interaction with the shielded pool a new temporary account has to be created. Since, computation is conducted over 1502 instructions temporary accounts keep an index of the current computational step.

### Fork Security Report:

Light Protocol has been audited at commit 5a79cdff5e9ea4d621b5d50be16d938124b24723.
Read the [audit report here](https://github.com/Lightprotocol/light-protocol-program/blob/main/Audit/Light%20Protocol%20Audit%20Report.pdf).

### Notes:
- The implementation of the groth16_verifier is based on the arkworks libraries, mainly [ark_bn254](https://docs.rs/ark-bn254/0.3.0/ark_bn254/), [ark_ec](https://docs.rs/ark-ec/0.3.0/ark_ec/) and [ark_ff](https://docs.rs/ark-ff/0.3.0/ark_ff/).
- The implementation of the poseidon hash is based on [arkworks_gadgets](https://docs.rs/arkworks-gadgets/0.3.14/arkworks_gadgets/poseidon/circom/index.html)
- Light uses a circuit based on [tornado_nova](https://github.com/tornadocash/tornado-nova/tree/master/circuits).

Also note that we're using a fork of arkwork's ark-ec: https://github.com/Lightprotocol/algebra where we've made certain functions/structs public:

...bls12/g2.rs
- G2HomProjective
- mul_by_char
- doubling_step
- addition_step

...models/bn/mod.rs
- ell

so we can reuse them mostly in tests.

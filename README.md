# sui-zk-airdrop

This is a demonstration of using a Merkle Tree to contain a list of Sui wallet addresses. These wallets can verify their inclusion in this list onchain, by submitting offchain proofs.

Onchain verification is managed by [Groth16](https://docs.sui.io/guides/developer/cryptography/groth16).

This infrastructure could be useful for airdrops where the number of wallets makes the gas costs of wallet-to-wallet transfers unfeasible, or if it it preferred that claimants interact in order to participate.

The wallet lists and Merkle roots can be stored on [Walrus](https://www.walrus.xyz/), to enable a fully decentralized user journey.

---

- [Sui contract](./sources/bagdrop.move)
- [Circom circuit](./src/leanimt.circom)

---

### Build Instructions

#### Compile the Circom circuit
> Requires installing [Circom](https://docs.circom.io/getting-started/installation/)

`circom src/leanimt.circom -l ./path/to/circomlib/circuits --r1cs --wasm -o ./dist`

#### Display the Merkle root
> This root will be saved onchain (and can also be stored alongside the leaves on Walrus)

`cargo run --bin create_root`

#### Generate claim values
> Any wallet in `./wallets.json` can be used

`WALLET=0x5354085bc8a8d3f96383483a9ba42410476af916d42ff5dd4f05bad55608f2ce cargo run --bin generate_claim_proof`

These output values should be passed to the Sui contract and the claim can be validated.

### Further resources:
- [Lean Incremental Merkle Tree in Circom](https://github.com/privacy-scaling-explorations/zk-kit.circom/issues/17)
- [fastcrypto-zkp](https://docs.rs/fastcrypto-zkp/latest/fastcrypto_zkp/)
- [Sui Groth16](https://docs.sui.io/guides/developer/cryptography/groth16)

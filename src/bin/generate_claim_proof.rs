use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomBuilder, CircomConfig};
use ark_crypto_primitives::snark::SNARK;
use ark_groth16::{prepare_verifying_key, Groth16};
use ark_serialize::CanonicalSerialize;
use bagdrop::poseidon;
use std::str::FromStr;
use sui_sdk_types::Address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet = std::env::var("WALLET")?;

    let mut rng = rand::rngs::OsRng;
    let tree = bagdrop::build_tree()?;

    let cfg = CircomConfig::<Fr>::new("./dist/leanimt_js/leanimt.wasm", "./dist/leanimt.r1cs")?;

    let circuit = {
        let mut builder = CircomBuilder::new(cfg);

        let addr = Address::from_str(&wallet)?;
        println!("Wallet: {}", addr.to_string());

        let leaf = poseidon::hash_address(&addr)?;
        builder.push_input("leaf", leaf.clone());

        let leaf_index = bagdrop::get_leaf_index(&wallet)?;
        println!("Leaf index: {}", leaf_index);
        builder.push_input("leafIndex", leaf_index);

        let siblings = tree.generate_proof(leaf_index)?;
        for (_i, sibling) in siblings.into_iter().enumerate() {
            builder.push_input("siblings", sibling);
        }

        builder.build()?
    };

    let pk =
        Groth16::<Bn254>::generate_random_parameters_with_reduction(circuit.clone(), &mut rng)?;

    let public_inputs = circuit.get_public_inputs().ok_or("Missing public inputs")?;

    // Create proof
    let proof = Groth16::<Bn254>::prove(&pk, circuit, &mut rng)?;

    // Verify proof
    let pvk = prepare_verifying_key(&pk.vk);
    let verified = Groth16::<Bn254>::verify_with_processed_vk(&pvk, &public_inputs, &proof)?;
    assert!(verified);

    // Print verifying key
    let mut pk_bytes = Vec::new();
    pk.vk.serialize_compressed(&mut pk_bytes)?;
    println!("Verifying key: {}", hex::encode(pk_bytes));

    // Print proof
    let mut proof_serialized = Vec::new();
    proof.serialize_compressed(&mut proof_serialized)?;
    println!("Proof: {}", hex::encode(proof_serialized));

    // Print root
    let mut root_serialized = Vec::new();
    public_inputs[0].serialize_compressed(&mut root_serialized)?;
    println!("Root: {}", hex::encode(root_serialized));

    Ok(())
}

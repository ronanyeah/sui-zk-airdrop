use ark_ff::{BigInteger, PrimeField};
use fastcrypto_zkp::bn254::poseidon::poseidon;
use num_bigint::BigInt;

pub fn hash_address(wallet: &sui_sdk_types::Address) -> anyhow::Result<BigInt> {
    // Split the 32 bytes into two 16-byte chunks and hash them
    let wallet_bytes: Vec<_> = wallet
        //
        .as_bytes()
        .into_iter()
        .map(|x| (*x).into())
        .collect();
    let (first_half, second_half) = wallet_bytes.split_at(16);

    // Hash each 16-byte chunk separately
    let hash1 = poseidon(first_half.to_vec())?;
    let hash2 = poseidon(second_half.to_vec())?;

    let final_hash = poseidon(vec![hash1, hash2])?;

    Ok(BigInt::from_bytes_le(
        num_bigint::Sign::Plus,
        &final_hash.into_bigint().to_bytes_le(),
    ))
}

pub fn hash_ints(xs: Vec<BigInt>) -> anyhow::Result<BigInt> {
    let hash = poseidon(
        xs.into_iter()
            .map(|n| {
                let bts = bigint_to_bits(&n);
                let ark_bigint = ark_ff::BigInt::<4>::from_bits_le(&bts);
                ark_bigint.into()
            })
            .collect(),
    )?;
    Ok(BigInt::from_bytes_le(
        num_bigint::Sign::Plus,
        &hash.into_bigint().to_bytes_le(),
    ))
}

fn bigint_to_bits(value: &BigInt) -> Vec<bool> {
    value
        .to_bytes_le()
        .1 // Discard the sign byte
        .iter()
        .flat_map(|byte| (0..8).map(move |i| ((byte >> i) & 1) == 1))
        .collect()
}

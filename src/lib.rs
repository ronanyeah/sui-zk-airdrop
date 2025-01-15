pub mod merkle;
pub mod poseidon;

use std::str::FromStr;

pub fn build_tree() -> anyhow::Result<merkle::LeanIMT> {
    let wallets = read_wallets()?;

    let mut tree = merkle::LeanIMT::new(14)?;

    for n in &wallets {
        let addr = sui_sdk_types::Address::from_str(n)?;
        tree.insert(poseidon::hash_address(&addr)?)?;
    }

    Ok(tree)
}

pub fn get_leaf_index(wallet: &str) -> anyhow::Result<usize> {
    let wallets = read_wallets()?;
    let leaf_index = wallets
        .iter()
        .position(|x| *x == wallet)
        .ok_or(anyhow::anyhow!("Leaf index not found"))?;
    Ok(leaf_index)
}

pub fn read_wallets() -> anyhow::Result<Vec<String>> {
    // todo: store on walrus
    Ok(serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open("./wallets.json")?,
    ))?)
}

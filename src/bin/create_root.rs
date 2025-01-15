fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tree = bagdrop::build_tree()?;

    let root = tree.get_root()?;

    println!("Root: {}", hex::encode(root.to_bytes_le().1));

    Ok(())
}

use crate::poseidon;
use anyhow::{anyhow, Result};
use num_bigint::BigInt;
use std::collections::HashMap;

pub struct LeanIMT {
    pub max_depth: usize,
    pub num_leaves: usize,
    pub max_leaves: usize,
    nodes: HashMap<(usize, usize), BigInt>, // (level, index) -> hash
}

impl LeanIMT {
    pub fn new(max_depth: usize) -> Result<Self> {
        if max_depth == 0 {
            Err(anyhow!("Tree depth must be greater than 0"))?;
        }
        Ok(LeanIMT {
            max_depth,
            nodes: HashMap::new(),
            num_leaves: 0,
            max_leaves: 1 << max_depth,
        })
    }

    pub fn get_root(&self) -> Result<BigInt> {
        self.nodes
            .get(&(self.max_depth, 0))
            .cloned()
            .ok_or(anyhow!("Tree is empty"))
    }

    pub fn insert(&mut self, leaf: BigInt) -> Result<usize> {
        if self.num_leaves >= self.max_leaves {
            Err(anyhow!("Tree is full - cannot insert more leaves"))?;
        }
        let leaf_index = self.num_leaves;
        self.update_tree(0, leaf_index, leaf)?;
        self.num_leaves += 1;
        Ok(leaf_index)
    }

    pub fn generate_proof(&self, leaf_index: usize) -> Result<Vec<BigInt>> {
        if leaf_index >= self.num_leaves {
            Err(anyhow!("Index out of bounds"))?;
        }

        let mut proof = Vec::with_capacity(self.max_depth);
        let mut current_index = leaf_index;

        for level in 0..self.max_depth {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if let Some(sibling_value) = self.nodes.get(&(level, sibling_index)) {
                proof.push(sibling_value.clone());
            } else {
                proof.push(BigInt::from(0));
            }

            current_index /= 2;
        }

        Ok(proof)
    }

    pub fn verify_proof(&self, leaf: BigInt, leaf_index: usize, proof: &[BigInt]) -> Result<bool> {
        if proof.len() != self.max_depth {
            Err(anyhow!("Invalid proof"))?;
        }

        let mut current_hash = leaf;
        let mut current_index = leaf_index;

        for (_level, sibling) in proof.iter().enumerate() {
            if sibling == &BigInt::from(0) {
                // If sibling is empty, propagate current hash up
                current_index /= 2;
                continue;
            }

            let (left, right) = if current_index % 2 == 0 {
                (current_hash, sibling.clone())
            } else {
                (sibling.clone(), current_hash)
            };

            current_hash = poseidon::hash_ints(vec![left, right])?;
            current_index /= 2;
        }

        Ok(current_hash == self.get_root()?)
    }

    // Internal

    fn update_tree(&mut self, level: usize, index: usize, value: BigInt) -> Result<()> {
        self.nodes.insert((level, index), value.clone());

        if level >= self.max_depth {
            return Ok(());
        }

        // Check if this node has a sibling
        let parent_index = index / 2;
        let is_left_child = index % 2 == 0;
        let sibling_index = if is_left_child { index + 1 } else { index - 1 };

        // If we have both children, compute parent
        if let Some(sibling_value) = self.nodes.get(&(level, sibling_index)) {
            let left_value = if is_left_child {
                value.clone()
            } else {
                sibling_value.clone()
            };
            let right_value = if is_left_child {
                sibling_value.clone()
            } else {
                value
            };

            let parent_value = poseidon::hash_ints(vec![left_value, right_value])?;
            self.update_tree(level + 1, parent_index, parent_value)?;
        }
        // If no sibling exists, propagate current value up
        else {
            self.update_tree(level + 1, parent_index, value)?;
        }

        Ok(())
    }
}

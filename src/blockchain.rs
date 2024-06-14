use serde::{Serialize, Deserialize};
use ring::digest::{Context, SHA256};
use std::time::{SystemTime, UNIX_EPOCH};
use hex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Self::calculate_hash(index, timestamp, &previous_hash, &data);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u128, previous_hash: &str, data: &str) -> String {
        let mut context = Context::new(&SHA256);
        context.update(&index.to_be_bytes());
        context.update(&timestamp.to_be_bytes());
        context.update(previous_hash.as_bytes());
        context.update(data.as_bytes());
        let digest = context.finish();
        hex::encode(digest.as_ref())
    }
}

#[derive(Serialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.add_genesis_block();
        blockchain
    }

    fn add_genesis_block(&mut self) {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            let recalculated_hash = Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.previous_hash,
                &current_block.data,
            );

            if current_block.hash != recalculated_hash {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        assert_eq!(block.index, 0);
        assert_eq!(block.previous_hash, "0");
        assert_eq!(block.data, "Genesis Block");
    }

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1); // Genesis block should be present
    }

    #[test]
    fn test_blockchain_validity() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("First block".to_string());
        blockchain.add_block("Second block".to_string());
        assert!(blockchain.is_valid());
    }
}


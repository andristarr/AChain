use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::proof_of_work::ProofOfWork;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
    pub timestamp: u128,
    pub nonce: usize,
}

impl Block {
    pub fn new(data: String, prev_block_hash: String) -> Block {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            data,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
        };

        let pow = ProofOfWork::new();

        let (nonce, hash) = pow.run(&block);

        block.hash = hash.clone();
        block.nonce = nonce;

        block
    }
}

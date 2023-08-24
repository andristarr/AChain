use crate::block::Block;
use sha256::digest;

use num::{bigint::BigUint as u256, Num};

use std::ops::ShlAssign;

pub struct ProofOfWork {
    pub target: u256,

    max_nonce: usize,
}

impl ProofOfWork {
    pub fn new() -> ProofOfWork {
        let mut target: u256 = u256::new(vec![1]);

        target.shl_assign(256 - 12);

        ProofOfWork {
            target,
            max_nonce: usize::MAX,
        }
    }

    fn prepare_data(&self, nonce: usize, block: &Block) -> String {
        let mut data = String::new();

        data.push_str(&block.prev_block_hash);
        data.push_str(&block.data);
        data.push_str(&block.timestamp.to_string());
        data.push_str(&self.target.to_string());
        data.push_str(&nonce.to_string());

        data
    }

    pub fn run(&self, block: &Block) -> (usize, String) {
        let mut hash = String::from("");

        let mut nonce = 0;

        println!("Mining the block containing {:?}", block.data);

        while nonce < self.max_nonce {
            let data = self.prepare_data(nonce, block);
            hash = digest(data);

            let hash_int = u256::from_str_radix(&hash, 16).unwrap();

            if hash_int.lt(&self.target) {
                break;
            } else {
                nonce += 1;
            }
        }

        (nonce, hash)
    }

    pub fn validate(&self, block: &Block) -> bool {
        let data = self.prepare_data(block.nonce, block);

        let hash = digest(data);

        let hash_int = u256::from_str_radix(&hash, 16).unwrap();

        hash_int.lt(&self.target)
    }
}

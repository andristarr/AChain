use serde::{Deserialize, Serialize};
use sled::Db;

use crate::block::Block;

#[derive(Serialize, Deserialize, Debug)]
struct DbBlock {
    hash: String,
    block: Block,
}

pub struct Blockchain {
    pub tip: String,
    db: Db,
}

impl Blockchain {
    pub fn add(&mut self, data: String) {
        let prev_block_hash = self.tip.clone();
        let new_block = Block::new(data, prev_block_hash);

        let mut blocks = self.get_blocks_from_db();
        self.tip = new_block.hash.clone();
        blocks.push(DbBlock {
            hash: new_block.hash.clone(),
            block: new_block,
        });

        self.update_blocks_in_db(&blocks);
    }

    pub fn new() -> Blockchain {
        let db = sled::open("achain_alpha").unwrap();
        let tip = match db.get(b"blocks") {
            Ok(Some(db_blocks)) => {
                let blocks = String::from_utf8(db_blocks.to_vec());
                match blocks {
                    Ok(blocks) => {
                        let blocks: Vec<DbBlock> = serde_json::from_str(&blocks).unwrap();

                        if blocks.len() == 0 {
                            Blockchain::generate_genesis(&db)
                        } else {
                            blocks.last().unwrap().hash.clone()
                        }
                    }
                    Err(_) => String::new(),
                }
            }
            _ => Blockchain::generate_genesis(&db),
        };

        Blockchain { tip, db }
    }

    pub fn clear(&self) {
        self.update_blocks_in_db(&vec![]);
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.get_blocks_from_db()
            .into_iter()
            .map(|b| b.block)
            .collect()
    }

    fn get_blocks_from_db(&self) -> Vec<DbBlock> {
        let blocks = self.db.get(b"blocks").unwrap_or_default();
        String::from_utf8(blocks.unwrap().to_vec())
            .ok()
            .and_then(|blocks| serde_json::from_str(&blocks).ok())
            .unwrap_or_default()
    }

    fn update_blocks_in_db(&self, blocks: &[DbBlock]) {
        let blocks_str = serde_json::to_string(blocks).unwrap();
        self.db.insert(b"blocks", blocks_str.as_bytes()).unwrap();
    }

    fn generate_genesis(db: &Db) -> String {
        let genesis = Block::new(String::from("genesis"), String::new());
        let tip = genesis.hash.clone();
        let db_block = DbBlock {
            hash: tip.clone(),
            block: genesis,
        };
        let inserted = vec![db_block];

        db.insert(
            b"blocks",
            serde_json::to_string(&inserted).unwrap().as_bytes(),
        )
        .unwrap();
        tip
    }
}

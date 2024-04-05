use sha2::{Digest, Sha256};
use anyhow::Result;
use log::info;
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};


struct Block {
    header: BlockHeader,
    body: BlockBody
}

struct BlockHeader {
    hash: String,
    height: u64,
    prev_hash: String,
    timestamp: u64
}

type BlockBody = Vec<String>;

impl Block {
    fn new(height: u64, prev_hash: String, body: BlockBody) -> Self {
        let timestamp = time::OffsetDateTime::now_utc().unix_timestamp() as u64;
        let mut bh = BlockHeader {
            hash: String::new(),
            height,
            prev_hash,
            timestamp
        };

        bh.hash = Self::calc_block_hash(height, &bh.prev_hash, timestamp, &body);
        Block { header:bh, body }
    }


//计算当前区块的hash值
fn calc_block_hash(height: u64, prev_hash: &str, timestamp: u64, body: &BlockBody) -> String {
    let concated_str = vec![
        height.to_string(),
        prev_hash.to_string(),
        timestamp.to_string(),
        body.concat(),
    ].concat();

    let mut hasher = Sha256::new();
    hasher.update(concated_str.as_bytes());
    hex::encode(hasher.finalize().as_slice())
}
}



#[test]
fn test_block_hash() {
    let block1 = Block::new(10, "aaabbbcccdddeeefff".to_string(), vec![]);
    let block2 = Block::new(10, "aaabbbcccdddeeefff".to_string(), vec![]);
    assert_eq!(block1.header.height, block2.header.height);
    assert_eq!(block1.header.prev_hash, block2.header.prev_hash);
    // XXX: have little probability to fail
    assert_eq!(block1.header.timestamp, block2.header.timestamp);
    // XXX: have little probability to fail
    assert_eq!(block1.header.hash, block2.header.hash);

    assert_eq!(block1.body, block2.body);
}
fn main() {
    println!("Hello, world!");
}



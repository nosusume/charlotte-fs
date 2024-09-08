use bytes::Bytes;
use crate::engine::interface::BlockEngine;
use anyhow::Result;
use crate::utils::get_sha256_by_vec;

pub struct CharlotteFile<T: BlockEngine> {
    pub path: String,
    pub size: u64,
    pub hash: String,
    pub block_size: u64,
    engine: T,
    pub blocks: Vec<String>,
}

impl <T: BlockEngine> CharlotteFile<T> {
    pub fn new(path: String, block_size: u64, data: Bytes, e: &mut T) -> Result<Self> {
        let size = data.len() as u64;
        let mut blocks = Vec::new();
        for (_, chunk) in data.chunks(block_size as usize).enumerate() {
            let block = e.put_block(chunk)?;
            blocks.push(block);
        }
        let hash = get_sha256_by_vec(&blocks);
        Ok(Self{
            path,
            size,
            hash,
            block_size,
            engine: e.clone(),
            blocks
        })
    }
}
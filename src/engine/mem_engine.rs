use anyhow::Result;
use bytes::Bytes;
use std::collections::HashMap;
use anyhow::anyhow;
use crate::engine::interface::BlockEngine;
use crate::utils::get_sha256;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct MemEngine {
    blocks: Arc<RwLock<HashMap<String, Bytes>>>,
}

impl MemEngine{
    pub fn new() -> Self {
        Self { blocks: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn get_block(&self, hash: &str) -> Result<Bytes> {
        let blocks = self.blocks.read().map_err(|e| anyhow!("Failed to read blocks: {:?}", e))?;
        let block = blocks.get(hash).ok_or(anyhow!("Block not found"))?;
        Ok(block.clone())
    }

    pub fn put_block(&mut self, block: &[u8]) -> Result<String> {
        let hash = get_sha256(&block);
        let mut blocks = self.blocks.write().map_err(|e| anyhow!("Failed to write blocks: {:?}", e))?;
        blocks.insert(hash.clone(), Bytes::from(block.to_vec()));
        Ok(hash)
    }

    pub fn delete_block(&mut self, hash: &str) -> Result<()> {
        let mut blocks = self.blocks.write().map_err(|e| anyhow!("Failed to write blocks: {:?}", e))?;
        blocks.remove(hash);
        Ok(())
    }

    pub fn check_file_exists(&self, hash: &str) -> Result<bool> {
        let blocks = self.blocks.read().map_err(|e| anyhow!("Failed to read blocks: {:?}", e))?;
        Ok(blocks.contains_key(hash))
    }
}


impl BlockEngine for MemEngine {
    fn get_block(&self, hash: &str) -> Result<Bytes> {
        self.get_block(hash)
    }

    fn put_block(&mut self, block: &[u8]) -> Result<String> {
        self.put_block(block)
    }

    fn delete_block(&mut self, hash: &str) -> Result<()> {
        self.delete_block(hash)
    }

    fn check_file_exists(&self, hash: &str) -> Result<bool> {
        self.check_file_exists(hash)
    }
}
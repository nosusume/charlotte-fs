use anyhow::Result;
use bytes::Bytes;
use crate::engine::interface::BlockEngine;
use crate::utils::{get_sha256, check_file_exists};

#[derive(Debug, Clone)]
pub struct OsFsEngine {
    pub base_path: String,
}

impl OsFsEngine {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    fn get_dir(&self, hash: &str) -> String {
        format!("{}/{}", self.base_path, &hash[0..2])
    }

    fn get_file_path(&self, hash: &str) -> String {
        format!("{}/{}", self.get_dir(hash), hash)
    }

    fn save_file(&self, hash: &str, data: &[u8]) -> Result<()> {
         let block_path = self.get_file_path(hash);
        std::fs::write(block_path, data)?;
        Ok(())
    }

    fn read_file(&self, hash: &str) -> Result<Bytes> {
        let block_path = self.get_file_path(hash);
        let data = std::fs::read(block_path)?;
        Ok(Bytes::from(data))
    }
}

impl BlockEngine for OsFsEngine {
    fn delete_block(&mut self, hash: &str) -> Result<()> {
        let block_path = self.get_dir(hash);
        std::fs::remove_file(block_path)?;
        Ok(())
    }

    fn put_block(&mut self, block: &[u8]) -> Result<String> {
        let hash = get_sha256(block);
        self.save_file(&hash, block)?;
        Ok(hash)
    }

    fn get_block(&self, hash: &str) -> Result<Bytes> {
        self.read_file(hash)
    }

    fn check_file_exists(&self, hash: &str) -> Result<bool> {
        let file_path = self.get_file_path(hash);
        Ok(check_file_exists(&file_path))
    }
}
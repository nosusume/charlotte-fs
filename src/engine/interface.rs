use anyhow::Result;
use bytes::Bytes;
use crate::engine::mem_engine::MemEngine;
use crate::engine::os_fs_engine::OsFsEngine;

pub trait BlockEngine: Clone {
    fn get_block(&self, hash: &str) -> Result<Bytes>;
    fn put_block(&mut self, block: &[u8]) -> Result<String>;
    fn delete_block(&mut self, hash: &str) -> Result<()>;
    fn check_file_exists(&self, hash: &str) -> Result<bool>;
}
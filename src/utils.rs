use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fs;
use std::path::Path;

pub fn get_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data);
    hasher.result_str()
}

pub fn get_sha256_by_vec(data: &Vec<String>) -> String {
    let mut hasher = Sha256::new();
    for block in data {
        hasher.input(block.as_bytes());
    }
    hasher.result_str()
}

pub fn create_dir_recursively(path: &str) -> std::io::Result<()> {
    if Path::new(path).exists() {
        return Ok(());
    }
    fs::create_dir_all(path)
}

pub fn check_file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

//! Shared integrity checks for vendored meta-schemas.

use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
struct LockFile {
    asset: Vec<LockAsset>,
}

#[derive(Debug, serde::Deserialize)]
struct LockAsset {
    path: String,
    sha256: String,
}

pub fn assert_lock_integrity(crate_root: &Path, all_len: usize) {
    let lock_path = crate_root.join("meta-schemas.lock.toml");
    let text = fs::read_to_string(&lock_path).expect("read lock");
    let lock: LockFile = toml::from_str(&text).expect("parse lock");
    assert_eq!(lock.asset.len(), all_len, "lock entry count vs ALL");
    for asset in &lock.asset {
        let path = crate_root.join("meta-schemas").join(&asset.path);
        let bytes = fs::read(&path).expect("read asset");
        let actual = hex_sha256(&bytes);
        assert_eq!(actual, asset.sha256, "sha256 mismatch for {}", asset.path);
    }
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

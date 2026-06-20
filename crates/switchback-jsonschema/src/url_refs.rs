//! HTTPS `$ref` fetch helpers (feature-gated; off by default).

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

static CACHE: OnceLock<Mutex<BTreeMap<String, Vec<u8>>>> = OnceLock::new();

fn cache() -> &'static Mutex<BTreeMap<String, Vec<u8>>> {
    CACHE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

/// Fetch a URL `$ref` target with an in-memory cache.
pub async fn fetch_url(url: &str) -> switchback_traits::Result<Vec<u8>> {
    if let Ok(guard) = cache().lock() {
        if let Some(bytes) = guard.get(url) {
            return Ok(bytes.clone());
        }
    }

    let response = reqwest::get(url)
        .await
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|e| switchback_traits::SwitchbackError::load(e.to_string()))?
        .to_vec();

    if let Ok(mut guard) = cache().lock() {
        guard.insert(url.to_string(), bytes.clone());
    }
    Ok(bytes)
}

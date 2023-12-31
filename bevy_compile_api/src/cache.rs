use log::error;
use std::fs;

use crate::CACHE_FOLDER_PATH;

pub fn get(hash: u128) -> Option<CacheEntry> {
    let dir = match fs::read_dir(CACHE_FOLDER_PATH) {
        Ok(dir) => dir,
        Err(err) => {
            error!("Failed to read cache dir: {err:?}");
            return None;
        }
    };
    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                error!("Failed to get cache dir entry: {err:?}");
                continue;
            }
        };
        if entry.file_name().eq_ignore_ascii_case(&hash.to_string()) {
            let mut bytes = match fs::read(entry.path()) {
                Ok(bytes) => bytes,
                Err(err) => {
                    error!("Failed to open cache entry: {err:?}");
                    return None;
                }
            };
            let len = bytes.len();
            let wasm_len = usize::from_be_bytes(bytes[len - 16..len - 8].try_into().unwrap());
            let js_len = usize::from_be_bytes(bytes[len - 8..len].try_into().unwrap());
            bytes.resize(len - 16, 0);
            return Some(CacheEntry {
                wasm_len,
                js_len,
                body: bytes,
            });
        }
    }
    None
}

pub fn insert(hash: u128, entry: CacheEntry) {
    let mut data = entry.body;
    data.append(&mut entry.wasm_len.to_be_bytes().to_vec());
    data.append(&mut entry.js_len.to_be_bytes().to_vec());
    if let Err(err) = fs::write(format!("{CACHE_FOLDER_PATH}/{hash}"), data) {
        error!("Failed to write cache entry to disk: {err:?}");
    }
}

pub struct CacheEntry {
    pub wasm_len: usize,
    pub js_len: usize,
    pub body: Vec<u8>,
}

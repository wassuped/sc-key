use super::utils;
use std::fs::File;
use std::io::Read;

const PATTERN: &[u8] = &[0x1A, 0xD5, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const KEY_SIZE: usize = 128;

#[derive(Debug)]
struct KeyInfo {
    pub obfuscated: Vec<u8>,
    pub entropy: f64,
}

pub fn extract_key_from_library(file_path: &str) -> Result<Option<Vec<u8>>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let potential_keys = find_potential_keys(&data);

    if let Some(best_key) = potential_keys
        .into_iter()
        .max_by(|a, b| a.entropy.partial_cmp(&b.entropy).unwrap())
    {
        Ok(Some(best_key.obfuscated))
    } else {
        Ok(None)
    }
}

fn find_potential_keys(data: &[u8]) -> Vec<KeyInfo> {
    let mut potential_keys = Vec::new();
    let mut offset = 0;

    while let Some(index) = utils::find_pattern(data, PATTERN, offset) {
        if let Some(key_start) = index.checked_sub(KEY_SIZE) {
            if key_start < data.len() {
                let obfuscated_key = &data[key_start..index];
                let entropy = utils::entropy(obfuscated_key);

                potential_keys.push(KeyInfo {
                    obfuscated: obfuscated_key.to_vec(),
                    entropy,
                });
            }
        }

        offset = index + PATTERN.len();
        if offset >= data.len() {
            break;
        }
    }

    potential_keys
}

mod core;

use core::{deobfuscator, key_extractor};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <libg.so|obfuscated_key>", args[0]);
        return;
    }

    let input = &args[1];

    if let Ok(server_public_key_obf) = deobfuscator::hex_string_to_byte_array(input) {
        if server_public_key_obf.len() == 128 {
            let server_public_key = deobfuscator::load_server_public_key(&server_public_key_obf);
            println!("{}", deobfuscator::byte_array_to_hex(&server_public_key));
        } else {
            eprintln!(
                "Key must be 128 bytes, got {}",
                server_public_key_obf.len()
            );
        }
    } else {
        match key_extractor::extract_key_from_library(input) {
            Ok(Some(server_public_key_obf)) => {
                let server_public_key =
                    deobfuscator::load_server_public_key(&server_public_key_obf);
                println!("{}", deobfuscator::byte_array_to_hex(&server_public_key));
            }
            Ok(None) => {
                eprintln!("No keys in {}", input);
            }
            Err(e) => {
                eprintln!("Cannot read {}: {}", input, e);
            }
        }
    }
}

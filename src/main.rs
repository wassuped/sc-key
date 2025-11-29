use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <obfuscated_key>", args[0]);
        return;
    }
        
    let server_public_key_str = &args[1];
    
    let server_public_key_obf = hex_string_to_byte_array(server_public_key_str)
        .expect("Invalid hex string");
    
    let server_public_key = load_server_public_key(&server_public_key_obf);
    println!("{}", byte_array_to_hex(&server_public_key));
}

fn byte_array_to_hex(byte_array: &[u8]) -> String {
    byte_array
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn hex_string_to_byte_array(hex_string: &str) -> Result<Vec<u8>, &'static str> {
    if hex_string.len() % 2 != 0 {
        return Err("String must have even length");
    }
    
    (0..hex_string.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_string[i..i + 2], 16)
                .map_err(|_| "Invalid hex character")
        })
        .collect()
}

fn load_server_public_key(server_public_key_obf: &[u8]) -> Vec<u8> {
    let mut server_public_key = vec![0u8; 32];
    
    for i in 0..16 {
        let v16 = read_unsigned_short_le(server_public_key_obf, 31 - 2 * i + 32);
        let v17_part1 = read_unsigned_short_le(server_public_key_obf, 2 * i + 1);
        let v17_part2 = read_unsigned_short_le(server_public_key_obf, 2 * i);
        
        let v17 = (v17_part1 ^ v16) | (v16 ^ v17_part2);
        
        let shift = (i & 7) as u32;
        let rotated = v17.rotate_left(11 - shift);
        let final_value = rotated ^ read_unsigned_short_le(server_public_key_obf, 31 - i + 32);
        
        server_public_key[2 * i] = (final_value & 0xFF) as u8;
        server_public_key[2 * i + 1] = ((final_value >> 8) & 0xFF) as u8;
    }
    
    server_public_key
}

fn read_unsigned_short_le(bytes: &[u8], index: usize) -> u16 {
    let pos = 2 * index;
    u16::from_le_bytes([bytes[pos], bytes[pos + 1]])
}


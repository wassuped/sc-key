use super::utils;

pub fn byte_array_to_hex(byte_array: &[u8]) -> String {
    utils::byte_array_to_hex(byte_array)
}

pub fn hex_string_to_byte_array(hex_string: &str) -> Result<Vec<u8>, &'static str> {
    utils::hex_string_to_byte_array(hex_string)
}

pub fn load_server_public_key(server_public_key_obf: &[u8]) -> Vec<u8> {
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
    utils::read_unsigned_short_le(bytes, index)
}

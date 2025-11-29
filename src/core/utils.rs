pub fn byte_array_to_hex(byte_array: &[u8]) -> String {
    byte_array.iter().map(|b| format!("{:02X}", b)).collect()
}

pub fn hex_string_to_byte_array(hex_string: &str) -> Result<Vec<u8>, &'static str> {
    (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16).map_err(|_| "Invalid hex char"))
        .collect()
}

pub fn read_unsigned_short_le(bytes: &[u8], index: usize) -> u16 {
    let pos = 2 * index;
    u16::from_le_bytes([bytes[pos], bytes[pos + 1]])
}

pub fn find_pattern(data: &[u8], pattern: &[u8], start_offset: usize) -> Option<usize> {
    if start_offset >= data.len() {
        return None;
    }

    data[start_offset..]
        .windows(pattern.len())
        .position(|window| window == pattern)
        .map(|pos| pos + start_offset)
}

pub fn entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut freq = [0u32; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }

    let mut entropy = 0.0;
    let data_len = data.len() as f64;

    for &count in freq.iter().filter(|&&c| c > 0) {
        let p = count as f64 / data_len;
        entropy -= p * p.log2();
    }

    entropy
}

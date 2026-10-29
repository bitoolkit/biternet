use base64::alphabet::Alphabet;
use base64::engine::{Engine, GeneralPurpose, GeneralPurposeConfig};

const BASE64_ALPHABET: &str = "LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA";

pub fn xencode(message: &str, key: &str) -> Vec<u8> {
    if message.is_empty() {
        return Vec::new();
    }

    let mut message = mix(message.as_bytes(), true);
    let key = mix(key.as_bytes(), false);
    let len = message.len();
    let last = len - 1;
    let mut right = message[last];
    let salt = 0x9e3779b9_u32;
    let mut delta = 0_u32;

    for _ in 0..6 + 52 / len {
        delta = delta.wrapping_add(salt);
        let offset = delta >> 2 & 3;
        for position in 0..=last {
            let left = message[(position + 1) % len];
            right = ((right >> 5) ^ (left << 2))
                .wrapping_add((left >> 3 ^ right << 4) ^ (delta ^ left))
                .wrapping_add(key[(position & 3) ^ offset as usize] ^ right)
                .wrapping_add(message[position]);
            message[position] = right;
        }
    }

    split(&message, false)
}

pub fn fkbase64(payload: Vec<u8>) -> String {
    let alphabet = Alphabet::new(BASE64_ALPHABET).unwrap();
    let engine = GeneralPurpose::new(&alphabet, GeneralPurposeConfig::new());

    engine.encode(payload)
}

fn mix(buffer: &[u8], append_size: bool) -> Vec<u32> {
    let mut mixed: Vec<u32> = buffer
        .chunks(4)
        .map(|chunk| {
            u32::from_le_bytes(chunk.try_into().unwrap_or_else(|_| {
                let mut last_chunk = [0_u8; 4];
                last_chunk[..chunk.len()].clone_from_slice(chunk);
                last_chunk
            }))
        })
        .collect();

    if append_size {
        mixed.push(buffer.len() as u32);
    }

    mixed
}

fn split(buffer: &[u32], include_size: bool) -> Vec<u8> {
    let len = buffer.len();
    let size_record = buffer[len - 1];
    if include_size {
        let size = ((len - 1) * 4) as u32;
        if size_record < size - 3 || size_record > size {
            return Vec::new();
        }
    }

    let mut split: Vec<u8> = buffer
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    if include_size {
        split.truncate(size_record as usize);
    }

    split
}

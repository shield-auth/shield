use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngCore;

pub enum Length {
    // U32,
    U64,
}

pub fn generate_random_string(length: Length) -> String {
    let length = match length {
        // Length::U32 => 32,
        Length::U64 => 64,
    };

    let mut bytes = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut bytes);
    STANDARD.encode(&bytes)
}

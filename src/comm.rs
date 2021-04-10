use rand::Rng;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;

use crate::cli::FixedSizeByteSequenceParameter;
use crate::crypto::{
    decrypt_iv, decrypt_message, derive_aes_key_from_keycode, encrypt_iv, encrypt_message,
};
use crate::network;

#[derive(Debug)]
pub enum CommunicationError {
    InvalidCommand(String),
}

impl fmt::Display for CommunicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommunicationError::InvalidCommand(command) => write!(f, "Invalid command {}", command),
        }
    }
}
impl Error for CommunicationError {}

pub fn send_wol_packet(target_ip: &str, target_mac_address: &str) -> Result<usize, Box<dyn Error>> {
    let mac = FixedSizeByteSequenceParameter::from_string(target_mac_address.to_string(), ':', 6)?;
    const MAGIC_PACKET_HEADER: [u8; 6] = [0xFF; 6];

    let mut magic_packet = [0u8; 102];

    MAGIC_PACKET_HEADER
        .iter()
        .chain([mac.bytes].iter().cycle().take(16).flatten())
        .enumerate()
        .for_each(|(i, byte)| magic_packet[i] = *byte);

    network::send_udp_message(target_ip, &magic_packet).map_err(|e| e.into())
}

pub fn send_command(
    host: &str,
    port: u16,
    keycode: &str,
    salt: &[u8; 16],
    mut command: String,
) -> Result<String, Box<dyn Error>> {
    let aes_key = derive_aes_key_from_keycode(keycode, salt);
    let randomly_generated_iv: [u8; 16] = rand::thread_rng().gen();

    command.push('\r');
    let mut encrypted_message = encrypt_iv(&randomly_generated_iv, &aes_key);
    encrypted_message.extend_from_slice(&encrypt_message(
        command.as_str(),
        &randomly_generated_iv,
        &aes_key,
    ));

    let response = match network::send_and_receive_tcp_message((host, port), &encrypted_message) {
        Ok(response_bytes) => response_bytes,
        Err(e) => return Err(Box::new(e)),
    };
    let (encrypted_response_iv, encrypted_response_message) = response.split_at(16);
    let encrypted_response_iv: &[u8; 16] = encrypted_response_iv
        .try_into()
        .expect("Response IV wasn't 16 bytes. This should never happen!");

    let decrypted_response_iv = decrypt_iv(&encrypted_response_iv, &aes_key);
    let decrypted_response_message =
        decrypt_message(encrypted_response_message, &decrypted_response_iv, &aes_key);

    let null_char_position = decrypted_response_message
        .iter()
        .position(|b| *b == 0)
        .unwrap_or(decrypted_response_message.len());
    Ok(String::from_utf8_lossy(&decrypted_response_message[..null_char_position]).into_owned())
}

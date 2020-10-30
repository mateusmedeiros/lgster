use std::convert::TryInto;
use ::crypto::buffer::{BufferResult,ReadBuffer,RefReadBuffer,RefWriteBuffer,WriteBuffer};
use ::crypto::hmac::Hmac;
use ::crypto::pbkdf2::pbkdf2;
use ::crypto::sha2::Sha256;
use ::crypto::aes::{cbc_encryptor,ecb_encryptor,cbc_decryptor,ecb_decryptor,KeySize};
use ::crypto::blockmodes;
use ::crypto::symmetriccipher::{Decryptor,Encryptor,SymmetricCipherError};

trait CryptoOperator {
    fn operate(&mut self, input: &mut RefReadBuffer, output: &mut RefWriteBuffer, eof: bool) -> Result<BufferResult, SymmetricCipherError>;
}

impl CryptoOperator for Box<dyn Encryptor> {
    fn operate(&mut self, input: &mut RefReadBuffer, output: &mut RefWriteBuffer, eof: bool) -> Result<BufferResult, SymmetricCipherError> {
        self.encrypt(input, output, eof)
    }
}

impl CryptoOperator for Box<dyn Decryptor> {
    fn operate(&mut self, input: &mut RefReadBuffer, output: &mut RefWriteBuffer, eof: bool) -> Result<BufferResult, SymmetricCipherError> {
        self.decrypt(input, output, eof)
    }
}

fn operate(mut operator: impl CryptoOperator, content: &[u8]) -> Vec<u8> {
    let mut input_buffer = RefReadBuffer::new(content);
    let mut output = [0u8; 96]; // should be enough for any possible command (even a future one)
    let mut output_buffer = RefWriteBuffer::new(&mut output);

    loop {
        let result = operator.operate(&mut input_buffer, &mut output_buffer, true);
        match result {
            Ok(BufferResult::BufferUnderflow) => break,
            _ => {}
        }
    };

    output_buffer.take_read_buffer().take_remaining().to_vec()
}

fn encrypt(encryptor: impl CryptoOperator, content: &[u8]) -> Vec<u8> {
    operate(encryptor, content)
}

fn decrypt(decryptor: impl CryptoOperator, content: &[u8]) -> Vec<u8> {
    operate(decryptor, content)
}

pub fn derive_aes_key_from_keycode(keycode: &str, salt: &[u8; 16]) -> [u8; 16] {
    let mut hmac = Hmac::new(Sha256::new(), keycode.as_bytes());
    let mut aes_key = [0u8; 16];

    let iterations = 2u32.pow(14);
    pbkdf2(&mut hmac, salt, iterations, &mut aes_key);

    aes_key
}

pub fn encrypt_message(message: &str, iv: &[u8; 16], aes_key: &[u8; 16]) -> Vec<u8> {
    let encryptor = cbc_encryptor(KeySize::KeySize128, aes_key, iv, blockmodes::PkcsPadding);
    encrypt(encryptor, message.as_bytes())
}

pub fn encrypt_iv(iv: &[u8; 16], aes_key: &[u8; 16]) -> Vec<u8> {
    let encryptor = ecb_encryptor(KeySize::KeySize128, aes_key, blockmodes::NoPadding);
    encrypt(encryptor, iv)
}

pub fn decrypt_message(message: &[u8], iv: &[u8; 16], aes_key: &[u8; 16]) -> Vec<u8> {
    let decryptor = cbc_decryptor(KeySize::KeySize128, aes_key, iv, blockmodes::NoPadding);
    decrypt(decryptor, message)
}

pub fn decrypt_iv(encrypted_iv: &[u8; 16], aes_key: &[u8; 16]) -> [u8; 16] {
    let decryptor = ecb_decryptor(KeySize::KeySize128, aes_key, blockmodes::NoPadding);
    let decrypted_response = decrypt(decryptor, encrypted_iv);
    // Here we unwrap directly because something very crazy would have to happen inside the rust_crypto
    // crate for us to send an iv slice of 16 bytes to be decrypted and get back a slice of a different size
    decrypted_response[..].try_into().unwrap()
}

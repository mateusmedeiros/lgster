use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidByteSequenceError {
    invalid_byte: String,
}

impl fmt::Display for InvalidByteSequenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid byte {} (should be a hex number from 00-FF)",
            self.invalid_byte
        )
    }
}

impl Error for InvalidByteSequenceError {}

#[derive(Debug)]
pub struct WrongByteSequenceSize {
    should_be_size: usize,
}

impl fmt::Display for WrongByteSequenceSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Wrong size of byte sequence (should be {}))",
            self.should_be_size
        )
    }
}

impl Error for WrongByteSequenceSize {}

#[derive(Debug)]
pub struct FixedSizeByteSequenceParameter {
    pub sequence_string: String,
    pub delimiter: char,
    pub size: usize,
    pub bytes: Vec<u8>,
}

impl FixedSizeByteSequenceParameter {
    pub fn from_string(
        sequence_string: String,
        delimiter: char,
        size: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let mut collected_bytes = Vec::with_capacity(size);
        for byte_entry in sequence_string
            .split(delimiter)
            .take(size + 1)
            .map(|b| (b, u8::from_str_radix(b, 16)))
            .enumerate()
        {
            match byte_entry {
                (i, _) if i == size => {
                    return Err(Box::new(WrongByteSequenceSize {
                        should_be_size: size,
                    }))
                }
                (_, (_, Ok(byte))) => collected_bytes.push(byte),
                (_, (byte, Err(_))) => {
                    return Err(Box::new(InvalidByteSequenceError {
                        invalid_byte: byte.to_string(),
                    }))
                }
            }
        }

        if collected_bytes.len() < size {
            Err(Box::new(WrongByteSequenceSize {
                should_be_size: size,
            }))
        } else {
            Ok(FixedSizeByteSequenceParameter {
                sequence_string,
                delimiter,
                size,
                bytes: collected_bytes,
            })
        }
    }
}

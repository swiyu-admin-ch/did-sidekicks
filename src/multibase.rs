// SPDX-License-Identifier: MIT

use crate::errors::DidSidekicksError;
use bs58::{decode as base58_decode, encode as base58_encode, Alphabet as Alphabet58};
use std::cmp::PartialEq;

/// See https://www.ietf.org/archive/id/draft-multiformats-multibase-08.html#appendix-D.1
pub const BASE58BTC_MULTIBASE_IDENTIFIER: &str = "z";

/// A helper capable of encoding/decoding data in Multibase format according to
/// See https://www.ietf.org/archive/id/draft-multiformats-multibase-08.html#appendix-D.1
#[derive(PartialEq, Debug)]
pub enum MultibaseAlgorithm {
    /// Base58 bitcoin
    Base58btc,
}

impl MultibaseAlgorithm {
    /// Encode bytes into a new owned string using the alphabet supplied earlier.
    pub fn encode(&self, data: &[u8]) -> String {
        match self {
            MultibaseAlgorithm::Base58btc => {
                let encoded = base58_encode(data)
                    .with_alphabet(Alphabet58::BITCOIN)
                    .into_string();
                // See https://www.ietf.org/archive/id/draft-multiformats-multibase-08.html#name-base-58-bitcoin-encoding
                format!("{BASE58BTC_MULTIBASE_IDENTIFIER}{encoded}")
            }
        }
    }

    /// Decode into the given buffer.
    ///
    /// If the buffer is resizeable it will be extended and the new data will be written to the end
    /// of it.
    ///
    /// If the buffer is not resizeable bytes will be written from the beginning and bytes after
    /// the final encoded byte will not be touched.
    pub fn decode_onto(&self, multibase: &str, result: &mut [u8]) -> Result<(), DidSidekicksError> {
        match self {
            MultibaseAlgorithm::Base58btc => {
                if !multibase.starts_with(BASE58BTC_MULTIBASE_IDENTIFIER) {
                    return Err(DidSidekicksError::DeserializationFailed(format!(
                        "Invalid multibase algorithm identifier '{self:?}'",
                    )));
                }

                let raw = multibase.chars().skip(1).collect::<String>(); // get rid of the multibase identifier

                // decode into the given buffer
                match base58_decode(raw)
                    .with_alphabet(Alphabet58::BITCOIN)
                    .onto(result)
                {
                    Ok(_) => Ok(()),
                    Err(err) => Err(DidSidekicksError::DeserializationFailed(format!("{err}"))),
                }
            }
        }
    }
}

impl Default for MultibaseAlgorithm {
    fn default() -> Self {
        Self::Base58btc
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DidSidekicksErrorKind;
    use rstest::rstest;

    fn get_all_algorithms() -> Vec<MultibaseAlgorithm> {
        vec![MultibaseAlgorithm::Base58btc]
    }

    #[rstest]
    fn test_multibase_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let data = "hello world";
        for algorithm in get_all_algorithms().iter() {
            let encoded = algorithm.encode(data.as_bytes()); // == "z6sBRWyteSSzHrs"

            let mut buff = vec![0; 16];
            algorithm.decode_onto(encoded.as_str(), &mut buff)?;
            let decoded = String::from_utf8_lossy(&buff).to_string();
            assert!(decoded.starts_with(data));
        }
        Ok(())
    }

    #[rstest]
    fn test_multibase_conversion_invalid_multibase() -> Result<(), Box<dyn std::error::Error>> {
        let data = "hello world";
        for algorithm in get_all_algorithms().iter() {
            let encoded = algorithm.encode(data.as_bytes());

            // Now, to induce error, just get rid of the multibase code (prefix char 'z')
            let encoded_without_multibase = encoded.chars().skip(1).collect::<String>();
            let mut buff = vec![0; 16];
            let res = algorithm.decode_onto(encoded_without_multibase.as_str(), &mut buff);
            assert!(res.is_err());
            let err = res.unwrap_err(); // panic-safe unwrap call (see the previous line)
            assert_eq!(err.kind(), DidSidekicksErrorKind::DeserializationFailed);
            assert!(err
                .to_string()
                .contains("Invalid multibase algorithm identifier 'Base58btc'"));
        }
        Ok(())
    }

    #[rstest]
    fn test_multibase_conversion_buffer_too_small() {
        let data = "hello world";
        for algorithm in get_all_algorithms().iter() {
            let encoded = algorithm.encode(data.as_bytes()); // == "z6sBRWyteSSzHrs"

            // all it takes to reproduce the behaviour
            let mut buff = vec![0; 8]; // empirical size for "helloworld" (encoded)

            let res = algorithm.decode_onto(encoded.as_str(), &mut buff);
            assert!(res.is_err());
            let err = res.unwrap_err(); // panic-safe unwrap call (see the previous line)
            assert_eq!(err.kind(), DidSidekicksErrorKind::DeserializationFailed);
            assert!(err
                .to_string()
                .contains("buffer provided to decode base58 encoded string into was too small"));
        }
    }
}

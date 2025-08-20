// SPDX-License-Identifier: MIT

//! This project implements the following things:
//!
//! - General util structs reused by other libraries of swiyu-admin-ch
//!

extern crate core;

pub mod did_doc;
pub mod ed25519;
pub mod errors;
pub mod jcs_sha256_hasher;
pub mod multibase;
pub mod vc_data_integrity;
pub mod custom_jsonschema_keywords;
pub mod did_jsonschema;

// CAUTION All structs required by UniFFI bindings generator (declared in UDL) MUST also be "used" here
use did_doc::*;
//use ed25519::*;
use errors::*;
use did_jsonschema::*;

uniffi::include_scaffolding!("did_sidekicks");

#[cfg(test)]
mod test {
    use super::ed25519::*;
    use super::jcs_sha256_hasher::*;
    use super::multibase::*;
    use crate::errors::*;
    use crate::vc_data_integrity::*;
    use chrono::DateTime;
    use hex::encode as hex_encode;
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use rstest::{fixture, rstest};
    use serde_json::json;
    use std::vec;

    #[fixture]
    fn unique_base_url() -> String {
        let random_thing: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        format!("https://localhost:8000/{random_thing}")
    }

    #[fixture]
    #[once]
    fn ed25519_key_pair() -> Ed25519KeyPair {
        Ed25519KeyPair::generate()
    }

    #[rstest]
    fn test_multibase_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let multibase = MultibaseEncoderDecoder::default();
        let encoded = multibase.encode_base58btc("helloworld".as_bytes()); // == "z6sBRWyteSSzHrs"

        let mut buff = vec![0; 16];
        multibase.decode_base58_onto(encoded.as_str(), &mut buff)?;
        let decoded = String::from_utf8_lossy(&buff).to_string();
        assert!(decoded.starts_with("helloworld"));
        //assert_eq!(decoded, "helloworld");
        Ok(())
    }

    #[rstest]
    fn test_multibase_conversion_invalid_multibase() {
        let multibase = MultibaseEncoderDecoder::default();
        let encoded = multibase.encode_base58btc("helloworld".as_bytes()); // == "z6sBRWyteSSzHrs"

        // Now, to induce error, just get rid of the multibase code (prefix char 'z')
        let encoded_without_multibase = encoded.chars().skip(1).collect::<String>();
        let mut buff = vec![0; 16];
        let res = multibase.decode_base58_onto(encoded_without_multibase.as_str(), &mut buff);
        assert!(res.is_err());
        let err = res.unwrap_err(); // panic-safe unwrap call (see the previous line)
        assert_eq!(err.kind(), DidSidekicksErrorKind::DeserializationFailed);
        assert!(err
            .to_string()
            .contains("Invalid multibase algorithm identifier 'Base58btc'"));
    }

    #[rstest]
    fn test_multibase_conversion_buffer_too_small() {
        let multibase = MultibaseEncoderDecoder::default();
        let encoded = multibase.encode_base58btc("helloworld".as_bytes()); // == "z6sBRWyteSSzHrs"

        // all it takes to reproduce the behaviour
        let mut buff = vec![0; 8]; // empirical size for "helloworld" (encoded)

        let res = multibase.decode_base58_onto(encoded.as_str(), &mut buff);
        assert!(res.is_err());
        let err = res.unwrap_err(); // panic-safe unwrap call (see the previous line)
        assert_eq!(err.kind(), DidSidekicksErrorKind::DeserializationFailed);
        assert!(err
            .to_string()
            .contains("buffer provided to decode base58 encoded string into was too small"));
    }

    #[rstest]
    #[case(
        // Example taken from https://multiformats.io/multihash/#sha2-256---256-bits-aka-sha256
        "Merkle–Damgård",
        "122041dd7b6443542e75701aa98a0c235951a28a0d851b11564d20022ab11d2589a8"
    )]
    fn test_encode_multihash_sha256(#[case] input: String, #[case] expected: String) {
        let hash = hex_encode(JcsSha256Hasher::default().encode_multihash(input));
        assert_eq!(hash, expected);
    }

    #[rstest]
    fn test_key_pair_multibase_conversion(
        ed25519_key_pair: &Ed25519KeyPair, // fixture
    ) -> Result<(), Box<dyn std::error::Error>> {
        let original_private = ed25519_key_pair.get_signing_key();
        let original_public = ed25519_key_pair.get_verifying_key();

        let new_private = Ed25519SigningKey::from_multibase(&original_private.to_multibase())?;
        let new_public = Ed25519VerifyingKey::from_multibase(&original_public.to_multibase())?;

        assert_eq!(original_private.to_multibase(), new_private.to_multibase());
        assert_eq!(original_public.to_multibase(), new_public.to_multibase());
        Ok(())
    }

    #[rstest]
    fn test_key_pair_creation_from_multibase(
        ed25519_key_pair: &Ed25519KeyPair, // fixture
    ) -> Result<(), Box<dyn std::error::Error>> {
        let new_ed25519_key_pair =
            Ed25519KeyPair::from(&ed25519_key_pair.get_signing_key().to_multibase())?;

        assert_eq!(ed25519_key_pair, &new_ed25519_key_pair);
        assert_eq!(
            ed25519_key_pair.get_signing_key().to_multibase(),
            new_ed25519_key_pair.signing_key.to_multibase()
        );
        assert_eq!(
            ed25519_key_pair.get_verifying_key().to_multibase(),
            new_ed25519_key_pair.verifying_key.to_multibase()
        );
        Ok(())
    }

    /// A rather trivial assertion helper around TrustDidWebError.
    pub fn assert_trust_did_web_error<T>(
        res: Result<T, DidSidekicksError>,
        expected_kind: DidSidekicksErrorKind,
        _error_contains: &str,
    ) {
        assert!(res.is_err());
        let err = res.err();
        assert!(err.is_some());
        let err = err.unwrap();
        assert_eq!(err.kind(), expected_kind);

        /*let err_to_string = err.to_string();
        assert!(
            err_to_string.contains(_error_contains),
            "expected '{}' is not mentioned in '{}'",
            _error_contains,
            err_to_string
        );*/
    }

    #[rstest]
    fn test_cryptosuite_add_and_verify_proof() -> Result<(), Box<dyn std::error::Error>> {
        // From https://www.w3.org/TR/vc-di-eddsa/#example-credential-without-proof-0
        let credential_without_proof = json!(
            {
                 "@context": [
                     "https://www.w3.org/ns/credentials/v2",
                     "https://www.w3.org/ns/credentials/examples/v2"
                 ],
                 "id": "urn:uuid:58172aac-d8ba-11ed-83dd-0b3aef56cc33",
                 "type": ["VerifiableCredential", "AlumniCredential"],
                 "name": "Alumni Credential",
                 "description": "A minimum viable example of an Alumni Credential.",
                 "issuer": "https://vc.example/issuers/5678",
                 "validFrom": "2023-01-01T00:00:00Z",
                 "credentialSubject": {
                     "id": "did:example:abcdefgh",
                     "alumniOf": "The School of Examples"
                 }
            }
        );

        let scid = JcsSha256Hasher::default()
            .base58btc_encode_multihash(&credential_without_proof)
            .unwrap();

        // From https://www.w3.org/TR/vc-di-eddsa/#example-proof-options-document-1
        let options = CryptoSuiteProofOptions::new(
            None,
            Some(DateTime::parse_from_rfc3339("2023-02-24T23:36:38Z").unwrap().to_utc()),
            "did:key:z6MkrJVnaZkeFzdQyMZu1cgjg7k1pZZ6pvBQ7XJPt4swbTQ2#z6MkrJVnaZkeFzdQyMZu1cgjg7k1pZZ6pvBQ7XJPt4swbTQ2".to_string(),
            Some("assertionMethod".to_string()),
            Some(vec![
                "https://www.w3.org/ns/credentials/v2".to_string(),
                "https://www.w3.org/ns/credentials/examples/v2".to_string(),
            ]),
            format!("1-{}", scid),
        );

        // From https://www.w3.org/TR/vc-di-eddsa/#example-private-and-public-keys-for-signature-1
        let suite = EddsaJcs2022Cryptosuite {
            verifying_key: Some(Ed25519VerifyingKey::from_multibase(
                "z6MkrJVnaZkeFzdQyMZu1cgjg7k1pZZ6pvBQ7XJPt4swbTQ2",
            )?),
            signing_key: Some(Ed25519SigningKey::from_multibase(
                "z3u2en7t5LR2WtQH5PfFqMqwVHBeXouLzo6haApm8XHqvjxq",
            )?),
        };

        let secured_document = suite.add_proof(&credential_without_proof, &options)?;

        assert!(
            !secured_document.is_null(),
            "'add_proof' method returned Value::Null"
        );
        let proof = &secured_document["proof"];
        assert!(proof.is_array(), "'proof' must be a JSON array");
        let proof_value = &proof[0]["proofValue"];
        assert!(proof_value.is_string(), "'proofValue' must be a string");

        // https://www.w3.org/TR/vc-di-eddsa/#example-signature-of-combined-hashes-base58-btc-1
        // CAUTION The value suggested in the spec (z2HnFSSPPBzR36zdDgK8PbEHeXbR56YF24jwMpt3R1eHXQzJDMWS93FCzpvJpwTWd3GAVFuUfjoJdcnTMuVor51aX)
        //         is irrelevant here since the add_proof method also computes a proof's challenge (if not supplied already)
        assert!(proof_value.to_string().contains("z3swhrb2DFocc562PATcKiv8YtjUzxLdfr4dhb9DidvG2BNkJqAXe65bsEMiNJdGKDdnYxiBa7cKXXw4cSKCvMcfm"));

        let doc_hash = JcsSha256Hasher::default().encode_hex(&credential_without_proof)?;
        // From https://www.w3.org/TR/vc-di-eddsa/#example-hash-of-canonical-credential-without-proof-hex-0
        assert_eq!(
            "59b7cb6251b8991add1ce0bc83107e3db9dbbab5bd2c28f687db1a03abc92f19",
            doc_hash
        );

        // sanity check
        let proof_as_string = serde_json::to_string(proof)?;
        let data_integrity_proof = DataIntegrityProof::from(proof_as_string)?;
        assert!(
            suite.verify_proof(&data_integrity_proof, &doc_hash).is_ok(),
            "Sanity check failed"
        );

        Ok(())
    }

}

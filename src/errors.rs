// SPDX-License-Identifier: MIT

//use std::cmp::PartialEq;

/// Yet another UniFFI-compliant error.
///
/// Resembles ssi::dids::resolution::Error
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DidSidekicksError {
    /// Failed to serialize ID document (to JSON)
    #[error("failed to serialize DID document (to JSON): {0}")]
    SerializationFailed(String),
    /// The supplied did doc is invalid or contains an argument which isn't part of the did specification/recommendation
    #[error("The supplied did doc is invalid or contains an argument which isn't part of the did specification/recommendation: {0}"
    )]
    DeserializationFailed(String),
    /// Invalid DID document
    #[error("invalid DID document: {0}")]
    InvalidDidDocument(String),
    /// Invalid DID log integration proof
    #[error("invalid DID log integration proof: {0}")]
    InvalidDataIntegrityProof(String),
    /// Invalid DID method parameter
    #[error("invalid DID method parameter: {0}")]
    InvalidDidMethodParameter(String),
}

impl DidSidekicksError {
    /// Returns the error kind.
    pub fn kind(&self) -> DidSidekicksErrorKind {
        match self {
            Self::SerializationFailed(_) => DidSidekicksErrorKind::SerializationFailed,
            Self::DeserializationFailed(_) => DidSidekicksErrorKind::DeserializationFailed,
            Self::InvalidDidDocument(_) => DidSidekicksErrorKind::InvalidDidDocument,
            Self::InvalidDataIntegrityProof(_) => DidSidekicksErrorKind::InvalidIntegrityProof,
            Self::InvalidDidMethodParameter(_) => DidSidekicksErrorKind::InvalidDidMethodParameter,
        }
    }
}

/// TrustDidWebError kind.
///
/// Each [`DidSidekicksError`] has a kind provided by the [`DidSidekicksErrorKind::kind`] method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DidSidekicksErrorKind {
    SerializationFailed,
    DeserializationFailed,
    InvalidDidDocument,
    InvalidIntegrityProof,
    InvalidDidMethodParameter,
}

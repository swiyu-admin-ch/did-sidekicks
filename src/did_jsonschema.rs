// SPDX-License-Identifier: MIT

use thiserror::Error;

use crate::custom_jsonschema_keywords::*;
use jsonschema::draft202012::meta as jsch_meta;
use jsonschema::{options as jsch_opts, Draft, Validator as ValidatorBase};
use serde_json::from_str as json_from_str;

/// Represents any error condition that might occur in conjunction with [`DidLogEntryValidator`].
///
/// Yet another UniFFI-compliant error.
#[derive(Error, Debug, PartialEq)]
pub enum DidLogEntryValidatorError {
    #[error("the supplied JSON instance is not a valid DID log: {0}")]
    ValidationError(String),
    #[error("the supplied JSON instance cannot be deserialized: {0}")]
    DeserializationError(String),
}

impl DidLogEntryValidatorError {
    /// Returns the error kind.
    pub fn kind(&self) -> DidLogEntryValidatorErrorKind {
        match self {
            Self::ValidationError(_) => DidLogEntryValidatorErrorKind::ValidationError,
            Self::DeserializationError(_) => DidLogEntryValidatorErrorKind::DeserializationError,
        }
    }
}

/// [`DidLogEntryValidatorError`] kind.
///
/// Each [`DidLogEntryValidatorError`] variant has a kind provided by the [`DidLogEntryValidatorError::kind`] method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DidLogEntryValidatorErrorKind {
    ValidationError,
    DeserializationError,
}

/*
///
pub trait DidLogEntryJsonSchema {
    fn get_json_schema(&self) -> &str;
}
 */

/// A compiled JSON Schema validator.
///
/// This structure represents a JSON Schema that has been parsed and compiled into
/// an efficient internal representation for validation. It contains the root node
/// of the schema tree and the configuration options used during compilation.
//#[derive(Debug, Default, PartialEq)]
#[derive(Debug)]
pub struct DidLogEntryValidator {
    validator: ValidatorBase,
}

impl DidLogEntryValidator {
    /// Validate `instance` against `schema` and return the first error if any.
    ///
    /// A UniFFI-compliant method.
    pub fn validate(&self, instance: String) -> Result<(), DidLogEntryValidatorError> {
        self.validate_str(&instance)
    }

    /// Validate `instance` against `schema` and return the first error if any.
    pub fn validate_str(&self, instance: &str) -> Result<(), DidLogEntryValidatorError> {
        match json_from_str(instance) {
            Ok(val) => match self.validator.validate(&val) {
                Ok(_) => Ok(()),
                Err(e) => Err(DidLogEntryValidatorError::ValidationError(e.to_string())),
            },
            Err(e) => Err(DidLogEntryValidatorError::DeserializationError(
                e.to_string(),
            )),
        }
    }
}

/*
impl From<&dyn DidLogEntryJsonSchema> for DidLogEntryValidator {
    /// Create a new JSON Schema validator using `JSON Schema Draft 2020-12` specifications
    /// and supplied [`DidLogEntryJsonSchema`].
    ///
    /// Relies heavily on custom [`jsonschema::Keyword`] trait implementation like:
    /// - [`DidLogEntryKeyword`] and
    /// - [`DidVersionTimeKeyword`].
    fn from(value: &dyn DidLogEntryJsonSchema) -> Self {
        Self::from(value.get_json_schema())
    }
}
 */

impl From<String> for DidLogEntryValidator {
    /// Create a new JSON Schema validator using `JSON Schema Draft 2020-12` specifications
    /// and supplied [`String`].
    ///
    /// Relies heavily on custom [`jsonschema::Keyword`] trait implementation like:
    /// - [`DidLogEntryKeyword`] and
    /// - [`DidVersionTimeKeyword`].
    ///
    /// A UniFFI-compliant constructor.
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for DidLogEntryValidator {
    /// Create a new JSON Schema validator using `JSON Schema Draft 2020-12` specifications
    /// and a schema supplied as `&str`.
    ///
    /// Relies heavily on custom [`jsonschema::Keyword`] trait implementation like:
    /// - [`DidLogEntryKeyword`] and
    /// - [`DidVersionTimeKeyword`].
    fn from(value: &str) -> Self {
        match json_from_str(value) {
            Ok(sch) => {
                let _ = jsch_meta::validate(&sch).is_err_and(|e| panic!("{}", e.to_string()));
                match jsch_opts()
                    .with_draft(Draft::Draft202012)
                    .with_keyword(
                        DidLogEntryKeyword::KEYWORD_NAME,
                        DidLogEntryKeyword::factory,
                    )
                    .with_keyword(
                        DidVersionTimeKeyword::KEYWORD_NAME,
                        DidVersionTimeKeyword::factory,
                    )
                    .build(&sch)
                {
                    Ok(validator) => DidLogEntryValidator { validator },
                    Err(e) => panic!("{}", e.to_string()),
                }
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}

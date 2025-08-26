// SPDX-License-Identifier: MIT

use crate::errors::DidSidekicksError;
use serde_json::{from_str as json_from_str, to_string as json_to_string, Number, Value};

/// A generic DID method parameter as seen from the perspective of a JSON deserializer.
/// The value returned by [`DidMethodParameter::as_json_string`] must always be deserializable into a JSON object.
#[derive(Debug, Clone)]
pub struct DidMethodParameter {
    name: String,
    json_text: String,
    is_bool: bool,
    is_string: bool,
    is_number: bool,
    is_object: bool,
    is_array: bool,
    is_null: bool,
    bool_value: Option<bool>,
    string_value: Option<String>,
    number_value: Option<Number>,
}

impl DidMethodParameter {
    /// Yet another non-empty constructor of the type.
    pub fn new_bool(name: &str, value: bool) -> Self {
        // panic-safe unwrap call
        Self::new(name, value.to_string()).unwrap()
    }

    /// Yet another non-empty constructor of the type.
    pub fn new_bool_from_option(name: &str, value: Option<bool>) -> Self {
        if let Some(v) = value {
            return Self::new_bool(name, v);
        }
        Self::new_false(name) // fallback for an optional param
    }

    /// Yet another non-empty constructor of the type.
    pub fn new_true(name: &str) -> Self {
        // panic-safe unwrap call
        Self::new(name, "true".to_string()).unwrap()
    }

    /// Yet another non-empty constructor of the type.
    pub fn new_false(name: &str) -> Self {
        // panic-safe unwrap call
        Self::new(name, "false".to_string()).unwrap()
    }

    /// Yet another non-empty constructor of the type.
    pub fn new_string(name: &str, value: String) -> Result<Self, DidSidekicksError> {
        Self::new(name, format!("\"{value}\""))
    }

    /// Yet another non-empty constructor of the type.
    pub fn new_string_from_option(
        name: &str,
        value: Option<String>,
    ) -> Result<Self, DidSidekicksError> {
        if let Some(v) = value {
            return Self::new_string(name, v);
        }
        Err(DidSidekicksError::InvalidDidMethodParameter(format!(
            "DID method parameter omitted: {name}"
        )))
    }

    /// Yet another non-empty constructor of the type.
    pub fn new_string_array_from_option(
        name: &str,
        value: Option<Vec<String>>,
    ) -> Result<Self, DidSidekicksError> {
        if let Some(v) = value {
            // panic-safe unwrap call
            return Self::new(name, json_to_string(&v).unwrap());
        }
        Err(DidSidekicksError::InvalidDidMethodParameter(format!(
            "DID method parameter omitted: {name}"
        )))
    }

    /// Yet another non-empty constructor of the type.
    pub fn new_number_from_option(
        name: &str,
        value: Option<usize>,
    ) -> Result<Self, DidSidekicksError> {
        if let Some(v) = value {
            return Self::new(name, v.to_string());
        }
        Err(DidSidekicksError::InvalidDidMethodParameter(format!(
            "DID method parameter omitted: {name}"
        )))
    }

    /// The only non-empty constructor of the type.
    ///
    /// The supplied string of JSON text (`json_text`) must be deserializable into a JSON object.
    fn new(name: &str, json_text: String) -> Result<Self, DidSidekicksError> {
        let mut v = Self {
            name: name.to_string(),
            json_text: json_text.clone(),
            is_bool: false,
            is_string: false,
            is_number: false,
            is_object: false,
            is_array: false,
            is_null: false,
            bool_value: None,
            string_value: None,
            number_value: None,
        };

        match json_from_str::<Value>(json_text.as_str()) {
            Ok(Value::Bool(entry)) => {
                v.is_bool = true;
                v.bool_value = Some(entry);
            }
            Ok(Value::String(entry)) => {
                v.is_string = true;
                v.string_value = Some(entry);
            }
            Ok(Value::Number(entry)) => {
                v.is_number = true;
                v.number_value = Some(entry);
            }
            Ok(Value::Object(_)) => {
                v.is_object = true;
            }
            Ok(Value::Array(_)) => {
                v.is_array = true;
            }
            Ok(Value::Null) => {
                v.is_null = true;
            }
            Err(err) => {
                return Err(DidSidekicksError::InvalidDidMethodParameter(format!(
                    "'{json_text}' denoting the DID method parameter '{name}' is not a valid JSON text: {err}"
                )))
            }
        };

        Ok(v)
    }

    /// A UniFFI-compliant getter.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// A UniFFI-compliant getter.
    pub fn get_json_text(&self) -> String {
        self.json_text.clone()
    }

    /// A UniFFI-compliant getter.
    pub fn is_bool(&self) -> bool {
        self.is_bool
    }

    /// A UniFFI-compliant getter.
    pub fn is_string(&self) -> bool {
        self.is_string
    }

    /// A UniFFI-compliant getter.
    pub fn is_number(&self) -> bool {
        self.is_number
    }

    /// A UniFFI-compliant getter.
    pub fn is_object(&self) -> bool {
        self.is_object
    }

    /// A UniFFI-compliant getter.
    pub fn is_array(&self) -> bool {
        self.is_array
    }

    /// A UniFFI-compliant getter.
    pub fn is_null(&self) -> bool {
        self.is_null
    }

    /// A UniFFI-compliant getter.
    pub fn get_bool_value(&self) -> bool {
        if let Some(x) = &self.bool_value {
            return *x;
        }

        false
    }

    /// A UniFFI-compliant getter.
    pub fn get_string_value(&self) -> String {
        if let Some(x) = &self.string_value {
            return x.to_string();
        }

        "".to_string()
    }

    /// A UniFFI-compliant getter.
    pub fn get_f64_value(&self) -> f64 {
        if let Some(x) = &self.number_value {
            if x.is_f64() {
                return x.as_f64().unwrap_or(0.0);
            }
        }

        0.0
    }

    /// A UniFFI-compliant getter.
    pub fn get_i64_value(&self) -> i64 {
        if let Some(x) = &self.number_value {
            if x.is_i64() {
                return x.as_i64().unwrap_or(0);
            }
        }

        0
    }

    /// A UniFFI-compliant getter.
    pub fn get_u64_value(&self) -> u64 {
        if let Some(x) = &self.number_value {
            if x.is_u64() {
                return x.as_u64().unwrap_or(0);
            }
        }

        0
    }
}

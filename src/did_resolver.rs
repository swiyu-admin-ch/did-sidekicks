// SPDX-License-Identifier: MIT

use crate::did_doc::DidDoc;
use crate::did_method_parameters::DidMethodParameter;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;

/// A simple model of a generic DID resolver regardless of specification
pub trait DidResolver: Sized {
    type Error;

    /// The single (as well as non-empty) constructor
    fn resolve(did: String, did_log: String) -> Result<Self, Self::Error>;

    /// The getter for [`DidDoc`] object as outcome of calling [`DidResolver::resolve`] constructor
    fn get_did_doc_obj(&self) -> DidDoc;

    /// The getter for the map of [`DidMethodParameter`] as outcome of calling [`DidResolver::resolve`] constructor
    fn get_did_method_parameters_map(
        &self,
    ) -> impl TryInto<HashMap<String, Arc<DidMethodParameter>>>;
}

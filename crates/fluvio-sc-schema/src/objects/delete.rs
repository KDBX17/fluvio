//!
//! # Delete object
//!
//!

use std::fmt::Debug;

use dataplane::core::{Encoder, Decoder};
use dataplane::api::Request;

use crate::{AdminSpec};
use crate::Status;
use crate::AdminPublicApiKey;
use super::{ObjectApiEnum};

ObjectApiEnum!(DeleteRequest);

// This can be auto generated by enum derive later
#[derive(Debug, Default, Encoder, Decoder)]
pub struct DeleteRequest<S: AdminSpec> {
    key: S::DeleteKey,
}

impl<S> DeleteRequest<S>
where
    S: AdminSpec,
{
    pub fn new(key: S::DeleteKey) -> Self {
        Self { key }
    }

    pub fn key(self) -> S::DeleteKey {
        self.key
    }
}

impl Request for ObjectApiDeleteRequest {
    const API_KEY: u16 = AdminPublicApiKey::Delete as u16;
    const DEFAULT_API_VERSION: i16 = 1;
    type Response = Status;
}

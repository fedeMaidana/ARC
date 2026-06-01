// ─── < Imports > ────────────────────────────────────────────────────

use serde::Deserialize;

use crate::request::{Request, RequestMode};

use super::error::JsonApiError;

// ─── < Input Structs > ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct JsonRequestInput {
    pub action: String,

    #[serde(default)]
    pub resource: Option<String>,

    #[serde(default)]
    pub command: Option<Vec<String>>,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn request_from_json(input: &str) -> Result<Request, JsonApiError> {
    let input: JsonRequestInput = serde_json::from_str(input).map_err(|source| JsonApiError::InvalidJson { source })?;

    input.into_request()
}

// ─── < Implementations > ────────────────────────────────────────────

impl JsonRequestInput {
    fn into_request(self) -> Result<Request, JsonApiError> {
        if self.action.trim().is_empty() {
            return Err(JsonApiError::MissingAction);
        }

        if self.action == "run" {
            return self.into_run_request();
        }

        self.into_resource_request()
    }

    fn into_run_request(self) -> Result<Request, JsonApiError> {
        let Self { action, resource, command } = self;

        if resource.is_some() {
            return Err(JsonApiError::ResourceNotAllowedForRun);
        }

        let Some(command_parts) = command else {
            return Err(JsonApiError::MissingCommand);
        };

        if command_parts.is_empty() {
            return Err(JsonApiError::EmptyCommand);
        }

        Ok(Request::new(RequestMode::Check, action, command_parts))
    }

    fn into_resource_request(self) -> Result<Request, JsonApiError> {
        let Self { action, resource, command } = self;

        if command.is_some() {
            return Err(JsonApiError::CommandOnlyAllowedForRun);
        }

        let command_parts = match resource {
            Some(resource) => vec![resource],
            None => Vec::new(),
        };

        Ok(Request::new(RequestMode::Check, action, command_parts))
    }
}

// ─── < Imports > ────────────────────────────────────────────────────

use serde::Deserialize;

use crate::request::{Request, RequestMode};

use super::error::JsonApiError;

// ─── < Input Structs > ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
        let normalized_action = normalize_action(&self.action)?;

        if normalized_action == "run" {
            return self.into_run_request(normalized_action);
        }

        self.into_resource_request(normalized_action)
    }

    fn into_run_request(self, action: String) -> Result<Request, JsonApiError> {
        let Self { resource, command, .. } = self;

        if resource.is_some() {
            return Err(JsonApiError::ResourceNotAllowedForRun);
        }

        let Some(command_parts) = command else {
            return Err(JsonApiError::MissingCommand);
        };

        let command_parts = validate_command_parts(command_parts)?;

        if command_parts.is_empty() {
            return Err(JsonApiError::EmptyCommand);
        }

        Ok(Request::new(RequestMode::Check, action, command_parts))
    }

    fn into_resource_request(self, action: String) -> Result<Request, JsonApiError> {
        let Self { resource, command, .. } = self;

        if command.is_some() {
            return Err(JsonApiError::CommandOnlyAllowedForRun);
        }

        let command_parts = match resource {
            Some(resource) => vec![normalize_resource(&resource)?],
            None => Vec::new(),
        };

        Ok(Request::new(RequestMode::Check, action, command_parts))
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn normalize_action(action: &str) -> Result<String, JsonApiError> {
    let action = action.trim();

    if action.is_empty() {
        return Err(JsonApiError::MissingAction);
    }

    Ok(action.to_string())
}

fn normalize_resource(resource: &str) -> Result<String, JsonApiError> {
    let resource = resource.trim();

    if resource.is_empty() {
        return Err(JsonApiError::EmptyResource);
    }

    Ok(resource.to_string())
}

fn validate_command_parts(command_parts: Vec<String>) -> Result<Vec<String>, JsonApiError> {
    for (index, command_part) in command_parts.iter().enumerate() {
        if command_part.trim().is_empty() {
            return Err(JsonApiError::EmptyCommandPart { index });
        }
    }

    Ok(command_parts)
}

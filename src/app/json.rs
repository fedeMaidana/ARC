// ─── < Imports > ────────────────────────────────────────────────────

use std::io::{self, Read};

use anyhow::{Context, Result};
use thiserror::Error;

use crate::json_api;
use crate::request::Request;

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum JsonReadError {
    #[error("could not read JSON request from stdin")]
    ReadStdin {
        #[source]
        source: std::io::Error,
    },

    #[error(transparent)]
    Parse(#[from] json_api::JsonApiError),
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn read_request_from_stdin() -> Result<Request, JsonReadError> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .map_err(|source| JsonReadError::ReadStdin { source })?;

    json_api::request_from_json(&input).map_err(JsonReadError::Parse)
}

pub fn print_response(response: &json_api::JsonDecisionResponse) -> Result<()> {
    let serialized = serde_json::to_string(response).context("could not serialize JSON response")?;

    println!("{serialized}");

    Ok(())
}

pub fn print_read_error(error: &JsonReadError) {
    let response = match error {
        JsonReadError::ReadStdin { .. } => json_api::error_response_with_code("stdin_read_error", error),
        JsonReadError::Parse(error) => json_api::error_response_with_code(error.code(), error),
    };

    print_error_response(&response);
}

pub fn print_error(error: &impl std::fmt::Display) {
    let response = json_api::error_response(error);

    print_error_response(&response);
}

// ─── < Private Functions > ──────────────────────────────────────────

fn print_error_response(response: &json_api::JsonErrorResponse) {
    match serde_json::to_string(response) {
        Ok(serialized) => println!("{serialized}"),
        Err(_) => {
            println!(
                r#"{{"ok":false,"api_version":"{}","kind":"error","error_code":"json_serialization_error","error":"could not serialize JSON error"}}"#,
                json_api::JSON_API_VERSION
            );
        }
    }
}

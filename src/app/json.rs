// ─── < Imports > ────────────────────────────────────────────────────

use std::io::{self, Read};

use anyhow::{Context, Result};

use crate::json_api;
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn read_request_from_stdin() -> Result<Request> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .context("could not read JSON request from stdin")?;

    json_api::request_from_json(&input).context("could not parse JSON request")
}

pub fn print_response(response: &json_api::JsonDecisionResponse) -> Result<()> {
    let serialized = serde_json::to_string(response).context("could not serialize JSON response")?;

    println!("{serialized}");

    Ok(())
}

pub fn print_error(error: &impl std::fmt::Display) {
    let response = json_api::error_response(error);

    match serde_json::to_string(&response) {
        Ok(serialized) => println!("{serialized}"),
        Err(_) => println!(r#"{{"ok":false,"error":"could not serialize JSON error"}}"#),
    }
}

// ─── < Modules > ────────────────────────────────────────────────────

mod error;
mod input;
mod output;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::error::JsonApiError;
pub use self::input::{JsonRequestInput, request_from_json};
pub use self::output::{
    JSON_API_VERSION, JsonDecisionOutput, JsonDecisionResponse, JsonErrorResponse, JsonExecutionOutput, JsonRequestOutput, decision_response_from_parts,
    error_response, error_response_with_code,
};

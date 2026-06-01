// ─── < Modules > ────────────────────────────────────────────────────

mod error;
mod event;
mod path;
mod sanitizer;
mod writer;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::error::AuditError;
pub use self::event::{AuditEvent, AuditExecution};
pub use self::writer::{ensure_audit_log_is_writable, record_event};

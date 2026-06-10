// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::request::Request;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct PolicyInput<'a> {
    request: &'a Request,
    config: &'a Config,
}

// ─── < Implementations > ────────────────────────────────────────────

impl<'a> PolicyInput<'a> {
    pub fn new(request: &'a Request, config: &'a Config) -> Self {
        Self { request, config }
    }

    pub fn request(&self) -> &'a Request {
        self.request
    }

    pub fn config(&self) -> &'a Config {
        self.config
    }
}

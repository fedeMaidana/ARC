// ─── < Traits (Domain Port) > ───────────────────────────────────────

pub trait PolicyRules {
    // ─── Actions ──────────────

    fn is_blocked_action(&self, action: &str) -> bool;

    fn is_allowed_action(&self, action: &str) -> bool;

    fn action_needs_resource(&self, action: &str) -> bool;

    fn action_should_ask(&self, action: &str) -> bool;

    fn default_action(&self) -> DefaultPolicyAction;

    // ─── Console commands ───────────

    fn is_blocked_command(&self, command: &str) -> bool;

    fn command_policy(&self, command: &str) -> ConsoleCommandPolicy;

    fn command_argument_should_ask(&self, command: &str, argument: &str) -> bool;

    fn is_blocked_console_argument(&self, argument: &str) -> bool;

    fn is_blocked_command_argument(&self, command: &str, argument: &str) -> bool;

    fn subcommand_policy(&self, command: &str, subcommand: Option<&str>) -> Option<ConsoleSubcommandPolicy>;

    // ─── Resources ──────────

    fn is_protected_resource(&self, resource: &str) -> bool;

    fn is_blocked_path(&self, resource: &str) -> bool;

    // ─── HTTP ───────────

    fn is_blocked_http_target(&self, resource: &str) -> bool;
}

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultPolicyAction {
    Allow,
    Ask,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleCommandPolicy {
    Allow,
    Ask,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleSubcommandPolicy {
    Allowed,
    Ask,
    Blocked,
    NotAllowed,
    Required,
}

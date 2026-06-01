// ─── < Imports > ────────────────────────────────────────────────────

use crate::{http_target, resource};

use super::{ActionsConfig, ConsoleConfig, HttpConfig, ResourcesConfig};

// ─── < Implementations > ────────────────────────────────────────────

impl ActionsConfig {
    pub fn is_allowed_action(&self, action: &str) -> bool {
        self.allowed.iter().any(|allowed_action| allowed_action == action)
    }

    pub fn is_blocked_action(&self, action: &str) -> bool {
        self.blocked.iter().any(|blocked_action| blocked_action == action)
    }

    pub fn action_needs_resource(&self, action: &str) -> bool {
        self.need_resource
            .iter()
            .any(|action_that_needs_resource| action_that_needs_resource == action)
    }

    pub fn action_should_ask(&self, action: &str) -> bool {
        self.ask.iter().any(|ask_action| ask_action == action)
    }
}

impl ResourcesConfig {
    pub fn is_blocked_path(&self, resource: &str) -> bool {
        self.blocked_path_prefixes
            .iter()
            .any(|blocked_prefix| resource::matches_path_prefix(resource, blocked_prefix))
    }

    pub fn is_protected_resource(&self, resource: &str) -> bool {
        self.protected
            .iter()
            .any(|protected_resource| resource::matches_resource_name(resource, protected_resource))
    }
}

impl HttpConfig {
    pub fn is_blocked_target(&self, resource: &str) -> bool {
        self.blocked_targets
            .iter()
            .any(|blocked_target| http_target::matches_blocked_target(resource, blocked_target))
    }
}

impl ConsoleConfig {
    pub fn is_allowed_command(&self, command_name: &str) -> bool {
        self.allowed_commands.iter().any(|command| command == command_name)
    }

    pub fn is_blocked_command(&self, command_name: &str) -> bool {
        self.blocked_commands.iter().any(|command| command == command_name)
    }

    pub fn is_blocked_argument(&self, argument: &str) -> bool {
        self.blocked_arguments.iter().any(|blocked_argument| blocked_argument == argument)
    }

    pub fn command_should_ask(&self, command_name: &str) -> bool {
        self.ask_commands.iter().any(|command| command == command_name)
    }
}

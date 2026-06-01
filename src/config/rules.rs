// ─── < Imports > ────────────────────────────────────────────────────

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
            .any(|blocked_prefix| resource.starts_with(blocked_prefix))
    }

    pub fn is_protected_resource(&self, resource: &str) -> bool {
        for protected_resource in &self.protected {
            if resource == protected_resource {
                return true;
            }

            let protected_resource_inside_folder = format!("/{protected_resource}");

            if resource.ends_with(&protected_resource_inside_folder) {
                return true;
            }
        }

        false
    }
}

impl HttpConfig {
    pub fn is_blocked_target(&self, resource: &str) -> bool {
        for blocked_target in &self.blocked_targets {
            if resource == blocked_target {
                return true;
            }

            let blocked_target_with_slash = format!("{blocked_target}/");
            let blocked_target_with_port = format!("{blocked_target}:");

            if resource.starts_with(&blocked_target_with_slash) {
                return true;
            }

            if resource.starts_with(&blocked_target_with_port) {
                return true;
            }
        }

        false
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

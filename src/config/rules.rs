// ─── < Imports > ────────────────────────────────────────────────────

use crate::{http_target, resource};

use super::{ActionsConfig, ConsoleCommandRule, ConsoleConfig, HttpConfig, ResourcesConfig};

// ─── < Implementations > ────────────────────────────────────────────

impl ActionsConfig {
    pub fn is_allowed_action(&self, action: &str) -> bool {
        self.allowed.iter().any(|allowed_action| allowed_action == action)
    }

    pub fn is_blocked_action(&self, action: &str) -> bool {
        self.blocked.iter().any(|blocked_action| blocked_action == action)
    }

    pub fn action_needs_resource(&self, action: &str) -> bool {
        self.need_resource.iter().any(|action_that_needs_resource| action_that_needs_resource == action)
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
        http_target::is_blocked_by_config(resource, self)
    }

    pub fn is_allowed_scheme(&self, scheme: &str) -> bool {
        self.allowed_schemes.iter().any(|allowed_scheme| allowed_scheme.eq_ignore_ascii_case(scheme))
    }

    pub fn is_blocked_host(&self, host: &str) -> bool {
        self.blocked_hosts.iter().any(|blocked_host| blocked_host.eq_ignore_ascii_case(host))
    }
}

impl ConsoleConfig {
    pub fn is_allowed_command(&self, command_name: &str) -> bool {
        if self.command_rule(command_name).is_some_and(|rule| rule.policy() == ConsoleCommandPolicy::Deny) {
            return false;
        }

        if self.allowed_commands.iter().any(|command| command == command_name) {
            return true;
        }

        self.command_rule(command_name)
            .is_some_and(|rule| matches!(rule.policy(), ConsoleCommandPolicy::Allow | ConsoleCommandPolicy::Ask))
    }

    pub fn is_blocked_command(&self, command_name: &str) -> bool {
        if self.blocked_commands.iter().any(|command| command == command_name) {
            return true;
        }

        self.command_rule(command_name).is_some_and(|rule| rule.policy() == ConsoleCommandPolicy::Deny)
    }

    pub fn is_blocked_argument(&self, argument: &str) -> bool {
        self.blocked_arguments.iter().any(|blocked_argument| blocked_argument == argument)
    }

    pub fn command_should_ask(&self, command_name: &str) -> bool {
        if self.ask_commands.iter().any(|command| command == command_name) {
            return true;
        }

        self.command_rule(command_name).is_some_and(|rule| rule.policy() == ConsoleCommandPolicy::Ask)
    }

    pub fn command_rule(&self, command_name: &str) -> Option<&ConsoleCommandRule> {
        self.command_rules.iter().find(|rule| rule.name == command_name)
    }

    pub fn is_blocked_command_argument(&self, command_name: &str, argument: &str) -> bool {
        self.command_rule(command_name).is_some_and(|rule| rule.is_blocked_argument(argument))
    }

    pub fn command_argument_should_ask(&self, command_name: &str, argument: &str) -> bool {
        self.command_rule(command_name).is_some_and(|rule| rule.argument_should_ask(argument))
    }
}

impl ConsoleCommandRule {
    pub fn policy(&self) -> ConsoleCommandPolicy {
        match self.mode.as_str() {
            "deny" => ConsoleCommandPolicy::Deny,
            "ask" => ConsoleCommandPolicy::Ask,
            "allow" => ConsoleCommandPolicy::Allow,
            _ => ConsoleCommandPolicy::Deny,
        }
    }

    pub fn subcommand_policy(&self, subcommand: Option<&str>) -> ConsoleSubcommandPolicy {
        if self.policy() == ConsoleCommandPolicy::Deny {
            return ConsoleSubcommandPolicy::Blocked;
        }

        if let Some(subcommand) = subcommand {
            if self.is_blocked_subcommand(subcommand) {
                return ConsoleSubcommandPolicy::Blocked;
            }

            if self.subcommand_should_ask(subcommand) {
                return ConsoleSubcommandPolicy::Ask;
            }

            if !self.allowed_subcommands.is_empty() && !self.is_allowed_subcommand(subcommand) {
                return ConsoleSubcommandPolicy::NotAllowed;
            }

            return ConsoleSubcommandPolicy::Allowed;
        }

        if !self.allowed_subcommands.is_empty() {
            return ConsoleSubcommandPolicy::Required;
        }

        ConsoleSubcommandPolicy::Allowed
    }

    fn is_allowed_subcommand(&self, subcommand: &str) -> bool {
        self.allowed_subcommands.iter().any(|allowed| allowed == subcommand)
    }

    fn is_blocked_subcommand(&self, subcommand: &str) -> bool {
        self.blocked_subcommands.iter().any(|blocked| blocked == subcommand)
    }

    fn subcommand_should_ask(&self, subcommand: &str) -> bool {
        self.ask_subcommands.iter().any(|ask| ask == subcommand)
    }

    fn is_blocked_argument(&self, argument: &str) -> bool {
        self.blocked_arguments.iter().any(|blocked| blocked == argument)
    }

    fn argument_should_ask(&self, argument: &str) -> bool {
        self.ask_arguments.iter().any(|ask| ask == argument)
    }
}

// ─── < Enums > ──────────────────────────────────────────────────────

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

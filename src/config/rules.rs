// ─── < Imports > ────────────────────────────────────────────────────

use crate::policy::{ConsoleCommandPolicy, ConsoleSubcommandPolicy, DefaultPolicyAction, PolicyRules};
use crate::{http_target, resource};

use super::{ActionsConfig, Config, ConsoleCommandRule, ConsoleConfig, HttpConfig, PolicyConfig, ResourcesConfig};

// ─── < Policy Rules Port > ──────────────────────────────────────────

impl PolicyRules for Config {
    fn is_blocked_action(&self, action: &str) -> bool {
        self.actions.is_blocked_action(action)
    }

    fn is_allowed_action(&self, action: &str) -> bool {
        self.actions.is_allowed_action(action)
    }

    fn action_needs_resource(&self, action: &str) -> bool {
        self.actions.action_needs_resource(action)
    }

    fn action_should_ask(&self, action: &str) -> bool {
        self.actions.action_should_ask(action)
    }

    fn default_action(&self) -> DefaultPolicyAction {
        self.policy.default_action_policy()
    }

    fn is_blocked_command(&self, command: &str) -> bool {
        self.console.is_blocked_command(command)
    }

    fn command_policy(&self, command: &str) -> ConsoleCommandPolicy {
        self.console.command_policy(command)
    }

    fn command_argument_should_ask(&self, command: &str, argument: &str) -> bool {
        self.console.command_argument_should_ask(command, argument)
    }

    fn is_blocked_console_argument(&self, argument: &str) -> bool {
        self.console.is_blocked_argument(argument)
    }

    fn is_blocked_command_argument(&self, command: &str, argument: &str) -> bool {
        self.console.is_blocked_command_argument(command, argument)
    }

    fn subcommand_policy(&self, command: &str, subcommand: Option<&str>) -> Option<ConsoleSubcommandPolicy> {
        self.console.command_rule(command).map(|rule| rule.subcommand_policy(subcommand))
    }

    fn is_protected_resource(&self, resource: &str) -> bool {
        self.resources.is_protected_resource(resource)
    }

    fn is_blocked_path(&self, resource: &str) -> bool {
        self.resources.is_blocked_path(resource)
    }

    fn is_blocked_http_target(&self, resource: &str) -> bool {
        self.http.is_blocked_target(resource)
    }
}

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

impl PolicyConfig {
    pub fn default_action_policy(&self) -> DefaultPolicyAction {
        match self.default_action.as_str() {
            "allow" => DefaultPolicyAction::Allow,
            "ask" => DefaultPolicyAction::Ask,
            "deny" => DefaultPolicyAction::Deny,
            _ => DefaultPolicyAction::Deny,
        }
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
        let Some(target) = http_target::parse(resource) else {
            return false;
        };

        if !self.is_allowed_scheme(target.scheme()) {
            return true;
        }

        if self.is_blocked_host(target.host()) {
            return true;
        }

        if self.block_localhost && target.is_loopback_or_unspecified() {
            return true;
        }

        if self.block_private_networks && target.is_private_network() {
            return true;
        }

        if self.block_link_local && target.is_link_local() {
            return true;
        }

        if self.block_metadata_services && target.is_metadata_service() {
            return true;
        }

        if self.blocked_cidrs.iter().any(|cidr| target.is_in_cidr(cidr)) {
            return true;
        }

        self.blocked_targets
            .iter()
            .any(|blocked_target| http_target::matches_blocked_target(resource, blocked_target))
    }

    pub fn is_allowed_scheme(&self, scheme: &str) -> bool {
        self.allowed_schemes
            .iter()
            .any(|allowed_scheme| allowed_scheme.eq_ignore_ascii_case(scheme))
    }

    pub fn is_blocked_host(&self, host: &str) -> bool {
        self.blocked_hosts
            .iter()
            .any(|blocked_host| blocked_host.eq_ignore_ascii_case(host))
    }
}

impl ConsoleConfig {
    pub fn is_allowed_command(&self, command_name: &str) -> bool {
        matches!(self.command_policy(command_name), ConsoleCommandPolicy::Allow | ConsoleCommandPolicy::Ask)
    }

    pub fn is_blocked_command(&self, command_name: &str) -> bool {
        if self.blocked_commands.iter().any(|command| command == command_name) {
            return true;
        }

        self.command_rule(command_name)
            .is_some_and(|rule| rule.policy() == ConsoleCommandPolicy::Deny)
    }

    pub fn is_blocked_argument(&self, argument: &str) -> bool {
        self.blocked_arguments.iter().any(|blocked_argument| blocked_argument == argument)
    }

    pub fn command_should_ask(&self, command_name: &str) -> bool {
        self.command_policy(command_name) == ConsoleCommandPolicy::Ask
    }

    pub fn command_rule(&self, command_name: &str) -> Option<&ConsoleCommandRule> {
        self.command_rules.iter().find(|rule| rule.name == command_name)
    }

    pub fn is_blocked_command_argument(&self, command_name: &str, argument: &str) -> bool {
        self.command_rule(command_name)
            .is_some_and(|rule| rule.is_blocked_argument(argument))
    }

    pub fn command_argument_should_ask(&self, command_name: &str, argument: &str) -> bool {
        self.command_rule(command_name)
            .is_some_and(|rule| rule.argument_should_ask(argument))
    }

    pub fn command_policy(&self, command_name: &str) -> ConsoleCommandPolicy {
        if self.is_blocked_command(command_name) {
            return ConsoleCommandPolicy::Deny;
        }

        if self.ask_commands.iter().any(|command| command == command_name) {
            return ConsoleCommandPolicy::Ask;
        }

        if self
            .command_rule(command_name)
            .is_some_and(|rule| rule.policy() == ConsoleCommandPolicy::Ask)
        {
            return ConsoleCommandPolicy::Ask;
        }

        if self.allowed_commands.iter().any(|command| command == command_name) {
            return ConsoleCommandPolicy::Allow;
        }

        if self
            .command_rule(command_name)
            .is_some_and(|rule| rule.policy() == ConsoleCommandPolicy::Allow)
        {
            return ConsoleCommandPolicy::Allow;
        }

        self.default_command_policy()
    }

    fn default_command_policy(&self) -> ConsoleCommandPolicy {
        match self.default_command_policy.as_str() {
            "allow" => ConsoleCommandPolicy::Allow,
            "ask" => ConsoleCommandPolicy::Ask,
            "deny" => ConsoleCommandPolicy::Deny,
            _ => ConsoleCommandPolicy::Deny,
        }
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

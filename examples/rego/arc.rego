package arc

import rego.v1

default decision := {
	"status": "deny",
	"reason_code": "action_not_configured",
	"risk": "high",
}

decision := {
	"status": "allow",
	"reason_code": "action_allowed",
	"risk": "low",
} if {
	input.request.action == "run"
	input.request.command.name == "echo"
}

decision := {
	"status": "allow",
	"reason_code": "action_allowed",
	"risk": "low",
} if {
	input.request.action == "run"
	input.request.command.name == "git"
	input.request.command.args[0] == "status"
}

decision := {
	"status": "ask",
	"reason_code": "console_subcommand_requires_approval",
	"risk": "low",
} if {
	input.request.action == "run"
	input.request.command.name == "git"
	input.request.command.args[0] == "commit"
}

decision := {
	"status": "deny",
	"reason_code": "console_subcommand_blocked",
	"risk": "critical",
} if {
	input.request.action == "run"
	input.request.command.name == "git"
	input.request.command.args[0] == "push"
}

decision := {
	"status": "deny",
	"reason_code": "console_command_blocked",
	"risk": "critical",
} if {
	input.request.action == "run"
	input.request.command.name == "rm"
}

decision := {
	"status": "deny",
	"reason_code": "console_argument_blocked",
	"risk": "critical",
} if {
	input.request.action == "run"
	input.request.command.parts[_] == "-rf"
}

decision := {
	"status": "allow",
	"reason_code": "action_allowed",
	"risk": "low",
} if {
	input.request.action == "http_get"
	startswith(input.request.resource, "https://")
}

decision := {
	"status": "deny",
	"reason_code": "invalid_http_url",
	"risk": "medium",
} if {
	input.request.action == "http_get"
	not startswith(input.request.resource, "https://")
}
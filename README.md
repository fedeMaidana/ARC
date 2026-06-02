# ARC

**ARC** stands for **Action Review Controller**.

ARC is a command-line security layer for AI agents, automation scripts, and developer tools. It reviews actions before they run, decides whether they should be allowed, denied, or manually approved, and writes every decision to an audit log.

---

## Table of contents

* [Installation](#installation)
* [Quick start](#quick-start)
* [Basic usage](#basic-usage)
* [How ARC works](#how-arc-works)
* [JSON API](#json-api)
* [Configuration](#configuration)
* [Audit log](#audit-log)
* [OpenCode integration](#opencode-integration)
* [Development](#development)
* [License](#license)

---

## Installation

ARC is currently a Rust project. The recommended way to try it is from source.

### Requirements

You need:

* Rust and Cargo
* Git
* `jq` for reading audit logs nicely
* Bun, only if you want to use the OpenCode integration

Optional, but recommended:

```bash
cargo install cargo-nextest --locked
```

### Download the project

Clone the repository:

```bash
git clone https://github.com/fedeMaidana/ARC.git
cd ARC
```

### Run without installing

During development, you can run ARC directly with Cargo:

```bash
cargo run -q -- help
```

Most development examples use:

```bash
cargo run -q --
```

For example:

```bash
cargo run -q -- run echo hola
```

Once ARC is installed as a binary, the same command becomes:

```bash
arc run echo hola
```

### Install locally

From the project root:

```bash
cargo install --path .
```

Then run:

```bash
arc help
```

If Cargo binaries are not available in your shell, add them to your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Initialize config

Create the default user config:

```bash
arc init
```

Or, without installing:

```bash
cargo run -q -- init
```

Check where ARC is reading config from:

```bash
arc config path
```

Show the active config:

```bash
arc config show
```

---

## Quick start

Run a command through ARC:

```bash
arc run echo hola
```

Or during development:

```bash
cargo run -q -- run echo hola
```

Check a command without executing it:

```bash
arc check run echo hola
```

Ask ARC for a JSON decision:

```bash
echo '{"action":"run","command":["echo","hola"]}' | arc decide --json
```

Open the interactive monitor:

```bash
arc monitor
```

`arc tui` is also supported as a technical alias.

View recent audit events:

```bash
tail -n 10 ~/.local/share/arc/audit.log | jq .
```

---

## Basic usage

### Run a command

```bash
arc run echo hola
```

Example output:

```txt
🛡️  ARC
────────

▶ Decision
  action:   run
  resource: echo hola
  result:   ✅ allow
  risk:     low
  reason:   request matches an allowed policy

▶ Execution
  running: echo hola
  status:  exit status: 0

▶ Output
    hola
```

### Check without running

```bash
arc check run echo hola
```

This asks ARC what it would do, but skips execution.

### Require approval

You can configure commands to require human approval.

Example:

```toml
[console]
ask_commands = ["echo"]
```

Then:

```bash
arc run echo hola
```

ARC will ask before executing:

```txt
◇ Approval required:
│  ARC wants to execute `echo hola`
│
◆ Select an option:
│  ○ Yes
│  ● No
└
```

### Block commands

If a command is blocked by policy, ARC will not execute it.

Example:

```bash
arc run curl --version
```

Possible result:

```txt
result:   ⛔ deny
risk:     critical
reason:   command is explicitly blocked by console policy
```

---

## How ARC works

ARC sits between an agent and the system.

The basic flow is:

```txt
request → review → decide → audit → execute safely
```

A request can end in one of three decisions:

```txt
allow  -> the action can run
deny   -> the action is blocked
ask    -> human approval is required
```

Each decision also has a risk level:

```txt
low
medium
high
critical
```

Decision and risk are separate.

For example:

```txt
command:  echo hola
decision: ask
risk:     low
```

A command can require approval without being dangerous.

Another example:

```txt
command:  rm -rf /
decision: deny
risk:     critical
```

That one is not a debate. That one gets shown the door.

---

## JSON API

ARC can receive requests as JSON. This is useful for agents, tools, editor integrations, and automation scripts.

The main JSON command is:

```bash
arc decide --json
```

It reads a JSON request from `stdin`, prints a JSON response to `stdout`, and exits with a machine-readable status code.

Example:

```bash
echo '{"action":"run","command":["echo","hola"]}' | arc decide --json
```

During development:

```bash
echo '{"action":"run","command":["echo","hola"]}' | cargo run -q -- decide --json
```

### Important behavior

`arc decide --json` is **decision-only**.

It does not execute commands directly. It checks what ARC would decide and returns a structured response.

That makes it safe to use from agents before execution.

### Request format

There are two supported request shapes.

#### Run command request

Use this shape when asking about a shell command:

```json
{
  "action": "run",
  "command": ["echo", "hola"]
}
```

Rules:

```txt
action  -> required
command -> required for action "run"
resource -> not allowed for action "run"
```

`command` must be a non-empty array of non-empty strings.

Good:

```json
{
  "action": "run",
  "command": ["git", "status"]
}
```

Rejected:

```json
{
  "action": "run",
  "command": []
}
```

Rejected:

```json
{
  "action": "run",
  "command": ["echo", ""]
}
```

Rejected:

```json
{
  "action": "run",
  "resource": "echo hola"
}
```

#### Resource request

Use this shape when asking about an action that targets a resource, such as an HTTP URL:

```json
{
  "action": "http_get",
  "resource": "https://example.com"
}
```

Rules:

```txt
action -> required
resource -> optional, but required by some policies
command -> only allowed for action "run"
```

Rejected:

```json
{
  "action": "http_get",
  "command": ["echo", "hola"]
}
```

### Strict input validation

The JSON API is intentionally strict.

ARC rejects:

```txt
unknown fields
missing action
empty action
empty resource
missing command for run
empty command array
empty command parts
command field on non-run actions
resource field on run actions
invalid JSON
```

Unknown fields are rejected to avoid silently accepting malformed agent requests.

Rejected:

```json
{
  "action": "run",
  "command": ["echo", "hola"],
  "unexpected": true
}
```

### Successful response

Example request:

```bash
echo '{"action":"run","command":["echo","hola"]}' | arc decide --json
```

Example response:

```json
{
  "ok": true,
  "api_version": "1",
  "kind": "decision",
  "request": {
    "mode": "check",
    "action": "run",
    "resource": "echo hola"
  },
  "decision": {
    "status": "allow",
    "reason": "request matches an allowed policy",
    "reason_code": "action_allowed",
    "risk": "low"
  },
  "execution": {
    "kind": "check_mode",
    "allowed": true,
    "executed": false,
    "exit_code": 0
  }
}
```

### Error response

Example invalid request:

```bash
echo '{"action":"run"}' | arc decide --json
```

Example response:

```json
{
  "ok": false,
  "api_version": "1",
  "kind": "error",
  "error_code": "missing_command",
  "error": "run action requires a command array"
}
```

### Response fields

Every successful decision response contains:

```txt
ok           -> true
api_version  -> JSON API version
kind         -> "decision"
request      -> normalized request ARC evaluated
decision     -> policy decision
execution    -> execution metadata
```

Every error response contains:

```txt
ok           -> false
api_version  -> JSON API version
kind         -> "error"
error_code   -> stable machine-readable error code
error        -> human-readable error message
```

### Decision object

```json
{
  "status": "allow",
  "reason": "request matches an allowed policy",
  "reason_code": "action_allowed",
  "risk": "low"
}
```

Supported decision statuses:

```txt
allow
deny
ask
```

Supported risk levels:

```txt
low
medium
high
critical
```

`reason_code` is intended for agents and scripts.

Example reason codes:

```txt
action_allowed
action_blocked
action_requires_approval
action_allowed_by_default
action_requires_approval_by_default
resource_required
console_command_required
console_command_blocked
console_command_not_allowed
console_command_requires_approval
console_subcommand_required
console_subcommand_blocked
console_subcommand_not_allowed
console_subcommand_requires_approval
console_argument_blocked
console_argument_requires_approval
resource_protected
path_blocked
invalid_http_url
http_target_blocked
action_not_configured
```

### Execution object

For `arc decide --json`, execution usually looks like this:

```json
{
  "kind": "check_mode",
  "allowed": true,
  "executed": false,
  "exit_code": 0
}
```

`executed` should be `false` for JSON decisions.

Execution kinds include:

```txt
check_mode
skipped_denied
ask_required
ask_declined
no_execution_needed
missing_command
command_finished
command_timed_out
command_failed
```

### Exit codes

`arc decide --json` uses exit codes that agents can rely on:

```txt
0 -> valid request, decision allows or asks
1 -> valid request, decision denies
2 -> invalid request or application/config error
```

Examples:

```bash
echo '{"action":"run","command":["echo","hola"]}' | arc decide --json
echo $?
```

```txt
0
```

```bash
echo '{"action":"run","command":["rm","-rf","/"]}' | arc decide --json
echo $?
```

```txt
1
```

```bash
echo '{"action":"run"}' | arc decide --json
echo $?
```

```txt
2
```

### Stable contract

The JSON API currently uses:

```txt
api_version: "1"
```

The goal is to keep this contract stable for agents.

Breaking changes should require a new `api_version`.

---

## Configuration

ARC uses a TOML configuration file.

Show the active config:

```bash
arc config show
```

Show the config path:

```bash
arc config path
```

Minimal example:

```toml
[policy]
default_action = "deny"

[actions]
allowed = ["run", "http_get"]
blocked = []
need_resource = ["run", "http_get"]
ask = []

[resources]
protected = [".env", "id_rsa", "secrets.txt"]
blocked_path_prefixes = ["/etc/", "/root/"]

[http]
allowed_schemes = ["http", "https"]
block_localhost = true
block_private_networks = true
block_link_local = true
block_metadata_services = true
blocked_hosts = ["localhost"]
blocked_cidrs = [
  "0.0.0.0/8",
  "10.0.0.0/8",
  "127.0.0.0/8",
  "169.254.0.0/16",
  "172.16.0.0/12",
  "192.168.0.0/16",
  "::1/128",
  "fc00::/7",
  "fe80::/10"
]
blocked_targets = [
  "http://localhost",
  "https://localhost",
  "http://127.0.0.1",
  "https://127.0.0.1"
]

[console]
default_command_policy = "deny"
allow_path_resolution = true
allowed_commands = ["cargo", "git", "rg", "ls", "pwd", "cat", "echo", "whoami", "date"]
blocked_commands = ["rm", "sudo", "su", "sh", "bash", "zsh", "curl", "wget", "ssh", "scp"]
blocked_arguments = ["-rf", "--no-preserve-root", "/", "/etc", "/root", "..", "~"]
ask_commands = []

[audit]
enabled = true
path = "~/.local/share/arc/audit.log"

[execution]
timeout_seconds = 10
max_output_bytes = 100000
inherit_environment = false
environment = []
```

---

## Audit log

ARC writes audit events as JSON Lines.

Default path:

```txt
~/.local/share/arc/audit.log
```

View recent events:

```bash
tail -n 10 ~/.local/share/arc/audit.log | jq .
```

Example event:

```json
{
  "timestamp_unix_seconds": 1780264155,
  "source": "opencode",
  "mode": "execute",
  "action": "run",
  "resource": "echo hola",
  "decision": "allow",
  "reason": "request matches an allowed policy",
  "risk": "low",
  "executed": true,
  "exit_code": 0,
  "execution": {
    "type": "command_finished",
    "command_line": "echo hola",
    "status": "exit status: 0",
    "success": true,
    "stdout_truncated": false,
    "stderr_truncated": false
  }
}
```

Current audit sources:

```txt
cli
json_api
opencode
```

That makes it possible to see whether a request came from a human CLI call, the JSON API, or an agent integration.

---

## OpenCode integration

ARC includes an experimental OpenCode integration.

The custom tool lives at:

```txt
.opencode/tools/bash.ts
```

It intercepts OpenCode bash commands and routes them through ARC.

Flow:

```txt
OpenCode
  ↓
custom bash tool
  ↓
ARC decide --json
  ↓
ARC policy
  ↓
allow / deny / ask
  ↓
ARC run
  ↓
audit log
```

Example blocked command:

```txt
Command: curl --version
Decision: deny
Risk: critical
Reason: command is explicitly blocked by console policy
```

This integration proves ARC can sit between a real agent and shell execution, but it is still experimental.

### OpenCode tool dependencies

OpenCode tool dependencies are isolated inside `.opencode`:

```txt
.opencode/
  tools/
    bash.ts
  bunfig.toml
  tsconfig.json
```

Install them with Bun:

```bash
cd .opencode
bun install --frozen-lockfile
cd ..
```

Do not commit:

```txt
node_modules/
.opencode/node_modules/
```

---

## Development

Run formatting:

```bash
cargo fmt --all
```

Run Clippy:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Run tests:

```bash
cargo nextest run
```

Run all checks:

```bash
cargo fmt --all \
&& cargo fmt --all --check \
&& cargo clippy --workspace --all-targets -- -D warnings \
&& cargo nextest run
```

Tests are organized outside `src`:

```txt
tests/
  unit/
  integration/
  e2e/
```

Run by type:

```bash
cargo nextest run --test unit
cargo nextest run --test integration
cargo nextest run --test e2e
```

---

## License

This project is licensed under the Apache License 2.0.
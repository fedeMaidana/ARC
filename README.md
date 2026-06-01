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
🛡️  ARC v0.2.1
────────────────

▶ Decision
  action:   run
  resource: echo hola
  result:   ✅ allow
  risk:     low
  reason:   action is allowed

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
reason:   console command is blocked
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

ARC can receive requests as JSON. This is useful for agents, tools, and automation scripts.

```bash
echo '{"action":"run","command":["echo","hola"]}' | arc decide --json
```

Example response:

```json
{
  "ok": true,
  "request": {
    "mode": "check",
    "action": "run",
    "resource": "echo hola"
  },
  "decision": {
    "status": "allow",
    "reason": "action is allowed",
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

The JSON API is decision-only. It does not execute commands directly.

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
[actions]
allowed = ["run", "http_get"]
blocked = []
need_resource = ["run", "http_get"]
ask = []

[resources]
protected = [".env", "id_rsa", "secrets.txt"]
blocked_path_prefixes = ["/etc/", "/root/"]

[http]
blocked_targets = [
  "http://localhost",
  "https://localhost",
  "http://127.0.0.1",
  "https://127.0.0.1"
]

[console]
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
  "reason": "action is allowed",
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
Reason: console command is blocked
```

This integration proves ARC can sit between a real agent and shell execution, but it is still experimental.

### OpenCode tool dependencies

OpenCode tool dependencies are isolated inside `.opencode`:

```txt
.opencode/
  tools/
    bash.ts
  package.json
  bun.lock
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
cargo fmt
```

Run Clippy:

```bash
cargo clippy -- -D warnings
```

Run tests:

```bash
cargo nextest run
```

Run all checks:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo nextest run
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

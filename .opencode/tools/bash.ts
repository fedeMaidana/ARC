import { tool } from "@opencode-ai/plugin"

type OpenCodeToolContext = {
  directory: string
}

type AgentGateDecisionResponse = {
  ok: boolean
  error?: string
  decision?: {
    status: "allow" | "deny" | "ask"
    reason: string
    risk: string
  }
  request?: {
    action: string
    resource?: string | null
  }
  execution?: {
    kind: string
    allowed: boolean
    executed: boolean
    exit_code: number
  }
}

type ProcessResult = {
  exitCode: number
  stdout: string
  stderr: string
}

export default tool({
  description:
    "Execute shell commands through AgentGate policy checks and audited execution.",

  args: {
    command: tool.schema.string().describe("Shell command to execute"),
  },

  async execute(args: { command: string }, context: OpenCodeToolContext) {
    const command = args.command.trim()

    if (command.length === 0) {
      return "AgentGate blocked execution: empty command."
    }

    let commandParts: string[]

    try {
      commandParts = splitShellCommand(command)
    } catch (error) {
      return [
        "AgentGate blocked execution.",
        "",
        "Reason: unsupported shell syntax for the current OpenCode adapter.",
        `Command: ${command}`,
        `Details: ${String(error)}`,
        "",
        "For now, use simple commands like:",
        "  cargo fmt",
        "  cargo nextest run",
        "  rg AgentGate src",
        "",
        "Pipes, redirects, command substitution and chained commands are intentionally blocked in this spike.",
      ].join("\n")
    }

    const decision = await askAgentGate(commandParts, context.directory)

    if (!decision.ok) {
      return [
        "AgentGate error while checking command.",
        "",
        `Command: ${command}`,
        `Error: ${decision.error ?? "unknown error"}`,
      ].join("\n")
    }

    if (!decision.decision) {
      return [
        "AgentGate returned an invalid decision response.",
        "",
        `Command: ${command}`,
      ].join("\n")
    }

    if (decision.decision.status === "deny") {
      return [
        "AgentGate blocked this command.",
        "",
        `Command: ${command}`,
        `Decision: ${decision.decision.status}`,
        `Risk: ${decision.decision.risk}`,
        `Reason: ${decision.decision.reason}`,
      ].join("\n")
    }

    if (decision.decision.status === "ask") {
      return [
        "AgentGate requires human approval for this command.",
        "",
        `Command: ${command}`,
        `Decision: ${decision.decision.status}`,
        `Risk: ${decision.decision.risk}`,
        `Reason: ${decision.decision.reason}`,
        "",
        "This OpenCode adapter is running in safe spike mode, so it will not approve prompts automatically.",
        "Run it manually through AgentGate if you want to approve it:",
        "",
        `  agentgate run ${command}`,
      ].join("\n")
    }

    const execution = await runThroughAgentGate(commandParts, context.directory)

    return formatExecutionResult(command, execution)
  },
})

async function askAgentGate(
  commandParts: string[],
  cwd: string,
): Promise<AgentGateDecisionResponse> {
  const input = JSON.stringify({
    action: "run",
    command: commandParts,
  })

  const result = await runProcess(
    [...agentgateCommand(), "decide", "--json"],
    cwd,
    input,
  )

  try {
    return JSON.parse(result.stdout) as AgentGateDecisionResponse
  } catch {
    return {
      ok: false,
      error: [
        "could not parse AgentGate JSON response",
        `stdout: ${result.stdout}`,
        `stderr: ${result.stderr}`,
        `exit code: ${result.exitCode}`,
      ].join("\n"),
    }
  }
}

async function runThroughAgentGate(
  commandParts: string[],
  cwd: string,
): Promise<ProcessResult> {
  return runProcess([...agentgateCommand(), "run", ...commandParts], cwd)
}

function agentgateCommand(): string[] {
  const configuredBinary = Bun.env.ARC_BIN

  if (configuredBinary && configuredBinary.trim().length > 0) {
    return [configuredBinary]
  }

  return ["cargo", "run", "-q", "--"]
}

async function runProcess(
  argv: string[],
  cwd: string,
  stdin?: string,
): Promise<ProcessResult> {
  const child = Bun.spawn(argv, {
    cwd,
    env: {
      ...Bun.env,
      ARC_SOURCE: "opencode",
    },
    stdin: stdin === undefined ? "ignore" : "pipe",
    stdout: "pipe",
    stderr: "pipe",
  })

  if (stdin !== undefined && child.stdin) {
    child.stdin.write(stdin)
    child.stdin.end()
  }

  const stdoutPromise = new Response(child.stdout).text()
  const stderrPromise = new Response(child.stderr).text()

  const [exitCode, stdout, stderr] = await Promise.all([
    child.exited,
    stdoutPromise,
    stderrPromise,
  ])

  return {
    exitCode,
    stdout,
    stderr,
  }
}

function formatExecutionResult(command: string, result: ProcessResult): string {
  const output = [
    "AgentGate allowed and executed this command.",
    "",
    `Command: ${command}`,
    `Exit code: ${result.exitCode}`,
  ]

  if (result.stdout.trim().length > 0) {
    output.push("", "Output:", result.stdout.trim())
  }

  if (result.stderr.trim().length > 0) {
    output.push("", "Error output:", result.stderr.trim())
  }

  return output.join("\n")
}

function splitShellCommand(command: string): string[] {
  const parts: string[] = []
  let current = ""
  let quote: "'" | '"' | null = null
  let escaping = false

  for (const char of command) {
    if (escaping) {
      current += char
      escaping = false
      continue
    }

    if (char === "\\") {
      escaping = true
      continue
    }

    if (quote) {
      if (char === quote) {
        quote = null
      } else {
        current += char
      }

      continue
    }

    if (char === "'" || char === '"') {
      quote = char
      continue
    }

    if (isUnsupportedShellOperator(char)) {
      throw new Error(`unsupported shell operator '${char}'`)
    }

    if (/\s/.test(char)) {
      if (current.length > 0) {
        parts.push(current)
        current = ""
      }

      continue
    }

    current += char
  }

  if (escaping) {
    current += "\\"
  }

  if (quote) {
    throw new Error(`unterminated ${quote} quote`)
  }

  if (current.length > 0) {
    parts.push(current)
  }

  if (parts.length === 0) {
    throw new Error("empty command")
  }

  return parts
}

function isUnsupportedShellOperator(char: string): boolean {
  return ["|", ";", "&", "<", ">", "`", "$", "(", ")"].includes(char)
}
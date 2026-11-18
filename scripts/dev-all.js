#!/usr/bin/env bun
import { spawn } from "bun";

const args = process.argv.slice(2);

if (args.includes("-h") || args.includes("--help")) {
  console.log(`
Umuryango Concurrent Development Runner

Usage:
  bun run dev:all            Run backend and web frontend concurrently (default)
  bun run dev:native:all     Run backend and native desktop app concurrently
  bun run dev:all --all      Run backend, web frontend, and native app concurrently
  bun run dev:all --raw      Run without prefixed log output

Options:
  --native, --app   Run backend and native app
  --all             Run backend, web frontend, and native app
  --raw             Inherit stdout/stderr directly (no prefixing)
  -h, --help        Show this help message
`);
  process.exit(0);
}

const runNative = args.includes("--native") || args.includes("--app");
const runAll = args.includes("--all");
const rawOutput = args.includes("--raw");

const colors = {
  reset: "\x1b[0m",
  bold: "\x1b[1m",
  dim: "\x1b[2m",
  cyan: "\x1b[36m",
  green: "\x1b[32m",
  magenta: "\x1b[35m",
  yellow: "\x1b[33m",
  red: "\x1b[31m",
};

const bunExe = process.execPath;

const servicesToStart = [
  {
    name: "backend",
    color: colors.cyan,
    cmd: [bunExe, "run", "scripts/dev-backend.js"],
    desc: "Actix-web API (watch mode, http://127.0.0.1:4433)",
  },
];

if (runAll) {
  servicesToStart.push({
    name: "frontend",
    color: colors.green,
    cmd: [bunExe, "run", "--cwd", "web", "dev"],
    desc: "Next.js Web Portal (http://localhost:3000)",
  });
  servicesToStart.push({
    name: "native",
    color: colors.magenta,
    cmd: [bunExe, "run", "scripts/dev-native.js"],
    desc: "GPUI Native Client (watch mode)",
  });
} else if (runNative) {
  servicesToStart.push({
    name: "native",
    color: colors.magenta,
    cmd: [bunExe, "run", "scripts/dev-native.js"],
    desc: "GPUI Native Client (watch mode)",
  });
} else {
  servicesToStart.push({
    name: "frontend",
    color: colors.green,
    cmd: [bunExe, "run", "--cwd", "web", "dev"],
    desc: "Next.js Web Portal (http://localhost:3000)",
  });
}

console.log(`${colors.bold}============================================================${colors.reset}`);
console.log(`${colors.bold}  Umuryango Concurrent Development Runner${colors.reset}`);
for (const svc of servicesToStart) {
  console.log(`  - ${svc.color}${svc.name.padEnd(10)}${colors.reset}: ${svc.desc}`);
}
console.log(`  Press ${colors.yellow}Ctrl+C${colors.reset} to stop all services.`);
console.log(`${colors.bold}============================================================\n${colors.reset}`);

const runningChildren = [];
let stopping = false;

async function streamLines(readable, prefix, color, isError = false) {
  if (!readable) return;
  const reader = readable.getReader();
  const decoder = new TextDecoder();
  let buffer = "";

  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      buffer += decoder.decode(value, { stream: true });
      const lines = buffer.split("\n");
      buffer = lines.pop() ?? "";
      for (const rawLine of lines) {
        const line = rawLine.endsWith("\r") ? rawLine.slice(0, -1) : rawLine;
        const formatted = `${color}[${prefix}]${colors.reset} ${line}`;
        if (isError) {
          console.error(formatted);
        } else {
          console.log(formatted);
        }
      }
    }
    if (buffer.length > 0) {
      const line = buffer.endsWith("\r") ? buffer.slice(0, -1) : buffer;
      const formatted = `${color}[${prefix}]${colors.reset} ${line}`;
      if (isError) {
        console.error(formatted);
      } else {
        console.log(formatted);
      }
    }
  } catch {
    // Stream closed when process killed
  }
}

for (const svc of servicesToStart) {
  const child = spawn(svc.cmd, {
    stdin: "inherit",
    stdout: rawOutput ? "inherit" : "pipe",
    stderr: rawOutput ? "inherit" : "pipe",
    env: {
      ...process.env,
      FORCE_COLOR: "1",
    },
  });

  runningChildren.push({ ...svc, process: child });

  if (!rawOutput) {
    void streamLines(child.stdout, svc.name, svc.color, false);
    void streamLines(child.stderr, svc.name, svc.color, true);
  }

  void child.exited.then((code) => {
    if (!stopping) {
      if (code !== 0) {
        console.error(
          `${svc.color}[${svc.name}]${colors.reset} ${colors.red}Process exited with code ${code}${colors.reset}`
        );
      } else {
        console.log(
          `${svc.color}[${svc.name}]${colors.reset} Process exited successfully`
        );
      }
    }
  });
}

async function shutdown() {
  if (stopping) return;
  stopping = true;
  console.log(`\n${colors.yellow}[system] Shutting down development services...${colors.reset}`);

  for (const svc of runningChildren) {
    try {
      svc.process.kill("SIGINT");
    } catch {
      // Ignore if already dead
    }
  }

  const forceTimeout = setTimeout(() => {
    for (const svc of runningChildren) {
      try {
        svc.process.kill("SIGKILL");
      } catch {}
    }
    process.exit(0);
  }, 3000);

  await Promise.all(
    runningChildren.map((svc) => svc.process.exited.catch(() => {}))
  );
  clearTimeout(forceTimeout);
  process.exit(0);
}

process.on("SIGINT", shutdown);
process.on("SIGTERM", shutdown);

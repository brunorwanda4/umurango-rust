#!/usr/bin/env bun
import { spawn } from "bun";

const args = process.argv.slice(2);

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

const SERVICE_DEFINITIONS = {
  backend: {
    name: "backend",
    color: colors.cyan,
    cmd: [bunExe, "run", "scripts/dev-backend.js"],
    desc: "Actix-web API (watch mode, http://127.0.0.1:4433)",
  },
  frontend: {
    name: "frontend",
    color: colors.green,
    cmd: [bunExe, "run", "--cwd", "web", "dev"],
    desc: "Next.js Web Portal (http://localhost:3000)",
  },
  native: {
    name: "native",
    color: colors.magenta,
    cmd: [bunExe, "run", "scripts/dev-native.js"],
    desc: "GPUI Native Client (watch mode)",
  },
};

function printHelp() {
  console.log(`
Umuryango Concurrent Development Runner

Usage:
  bun run dev:all                Run backend and web frontend concurrently (default)
  bun run dev:native:all         Run backend and native desktop app concurrently
  bun run dev:all --native       Run backend and native desktop app
  bun run dev:all not --web      Run backend and native desktop app (exclude web)
  bun run dev:all --no-web       Run backend and native desktop app (exclude web)
  bun run dev:all --all          Run backend, web frontend, and native app concurrently
  bun run dev:all --only <list>  Run only specified services (e.g. --only app,backend)
  bun run dev:all --raw          Run without prefixed log output

Service names / aliases:
  - backend (or: api, server)
  - web     (or: frontend, portal)
  - app     (or: native, desktop)

Options:
  --native, --app     Run backend and native desktop app
  --web, --frontend   Run backend and web frontend
  --all               Run backend, web frontend, and native app concurrently
  not --web, --no-web Exclude web frontend (runs backend + native app)
  not --app, --no-app Exclude native app (runs backend + web frontend)
  --no-backend        Exclude backend API
  --only <services>   Run only specified comma-separated services
  --raw               Inherit stdout/stderr directly (no prefixing)
  -h, --help          Show this help message
`);
}

function resolveService(str) {
  if (!str) return null;
  const s = str.toLowerCase().replace(/^-+/, "").trim();
  if (["backend", "api", "server", "actix"].includes(s)) return "backend";
  if (["frontend", "web", "next", "portal"].includes(s)) return "frontend";
  if (["native", "app", "desktop", "gpui", "client"].includes(s)) return "native";
  return null;
}

const excluded = new Set();
const explicitlyIncluded = new Set();
let runAll = false;
let rawOutput = false;
let onlyMode = false;

for (let i = 0; i < args.length; i++) {
  const arg = args[i].trim();
  const lower = arg.toLowerCase();

  if (arg === "-h" || arg === "--help") {
    printHelp();
    process.exit(0);
  }

  if (arg === "--raw") {
    rawOutput = true;
    continue;
  }

  if (arg === "--all") {
    runAll = true;
    continue;
  }

  // Exclusion patterns: "not <svc>", "without <svc>", "--not <svc>", "--without <svc>"
  if (
    ["not", "without", "--not", "--without", "exclude", "--exclude", "skip", "--skip"].includes(
      lower,
    )
  ) {
    const nextArg = args[i + 1];
    if (nextArg) {
      const svc = resolveService(nextArg);
      if (svc) {
        excluded.add(svc);
        i++;
        continue;
      }
    }
  }

  // Negation prefix patterns: --no-web, --not-web, --without-web, etc.
  if (
    lower.startsWith("--no-") ||
    lower.startsWith("--not-") ||
    lower.startsWith("--without-") ||
    lower.startsWith("-no-")
  ) {
    const rawName = lower.replace(/^--?(no|not|without)-/, "");
    const svc = resolveService(rawName);
    if (svc) {
      excluded.add(svc);
      continue;
    }
  }

  // --only patterns: --only=app,backend or --only app
  if (lower.startsWith("--only=") || lower.startsWith("--services=")) {
    onlyMode = true;
    const parts = lower.split("=")[1].split(/[,+]/);
    for (const part of parts) {
      const svc = resolveService(part);
      if (svc) explicitlyIncluded.add(svc);
    }
    continue;
  }

  if (lower === "--only" || lower === "only") {
    onlyMode = true;
    const nextArg = args[i + 1];
    if (nextArg && !nextArg.startsWith("-")) {
      const parts = nextArg.split(/[,+]/);
      for (const part of parts) {
        const svc = resolveService(part);
        if (svc) explicitlyIncluded.add(svc);
      }
      i++;
    }
    continue;
  }

  // Direct positive service flags: --native, --app, --web, --frontend, --backend, --api
  if (arg.startsWith("-")) {
    const directSvc = resolveService(arg);
    if (directSvc) {
      explicitlyIncluded.add(directSvc);
      continue;
    }
  }
}

let activeServices = new Set();

if (onlyMode) {
  activeServices = new Set(explicitlyIncluded);
} else if (runAll) {
  activeServices = new Set(["backend", "frontend", "native"]);
} else if (explicitlyIncluded.size > 0) {
  if (explicitlyIncluded.has("native") && !explicitlyIncluded.has("frontend")) {
    activeServices.add("backend");
    activeServices.add("native");
  } else if (explicitlyIncluded.has("frontend") && !explicitlyIncluded.has("native")) {
    activeServices.add("backend");
    activeServices.add("frontend");
  } else {
    for (const s of explicitlyIncluded) {
      activeServices.add(s);
    }
    if (!excluded.has("backend")) {
      activeServices.add("backend");
    }
  }
} else if (excluded.size > 0) {
  // Excluding services from dev:all defaults to all other services
  activeServices = new Set(["backend", "frontend", "native"]);
} else {
  // Default: backend + frontend (web)
  activeServices = new Set(["backend", "frontend"]);
}

// Remove any explicitly excluded services
for (const exc of excluded) {
  activeServices.delete(exc);
}

const serviceOrder = ["backend", "frontend", "native"];
const servicesToStart = serviceOrder
  .filter((key) => activeServices.has(key))
  .map((key) => SERVICE_DEFINITIONS[key]);

if (servicesToStart.length === 0) {
  console.error(`${colors.red}Error: No services selected to run.${colors.reset}`);
  console.error(`Available services: backend, frontend (web), native (app). Use --help for usage.`);
  process.exit(1);
}

console.log(
  `${colors.bold}============================================================${colors.reset}`,
);
console.log(`${colors.bold}  Umuryango Concurrent Development Runner${colors.reset}`);
for (const svc of servicesToStart) {
  console.log(`  - ${svc.color}${svc.name.padEnd(10)}${colors.reset}: ${svc.desc}`);
}
console.log(`  Press ${colors.yellow}Ctrl+C${colors.reset} to stop all services.`);
console.log(
  `${colors.bold}============================================================\n${colors.reset}`,
);

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
          `${svc.color}[${svc.name}]${colors.reset} ${colors.red}Process exited with code ${code}${colors.reset}`,
        );
      } else {
        console.log(`${svc.color}[${svc.name}]${colors.reset} Process exited successfully`);
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

  await Promise.all(runningChildren.map((svc) => svc.process.exited.catch(() => {})));
  clearTimeout(forceTimeout);
  process.exit(0);
}

process.on("SIGINT", shutdown);
process.on("SIGTERM", shutdown);

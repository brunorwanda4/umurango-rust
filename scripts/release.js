#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import path from "node:path";

const arg = (process.argv[2] || "patch").trim().toLowerCase();

if (arg === "-h" || arg === "--help") {
  console.log(`
Umuryango Self-Update Release Publisher

Usage:
  bun run release              Bump patch version (e.g. 0.1.0 -> 0.1.1), commit, tag, and push
  bun run release patch        Bump patch version
  bun run release minor        Bump minor version (e.g. 0.1.0 -> 0.2.0)
  bun run release major        Bump major version (e.g. 0.1.0 -> 1.0.0)
  bun run release <x.y.z>      Set explicit version (e.g. 0.2.5)
`);
  process.exit(0);
}

const rootDir = process.cwd();
const cargoPath = path.join(rootDir, "Cargo.toml");
const pkgPath = path.join(rootDir, "package.json");

const cargoToml = readFileSync(cargoPath, "utf8");
const versionMatch = cargoToml.match(/^version\s*=\s*"(\d+)\.(\d+)\.(\d+)"/m);

if (!versionMatch) {
  console.error("Failed to locate workspace version in Cargo.toml");
  process.exit(1);
}

let major = Number(versionMatch[1]);
let minor = Number(versionMatch[2]);
let patch = Number(versionMatch[3]);
const currentVersion = `${major}.${minor}.${patch}`;

let nextVersion = "";
if (arg === "patch") {
  nextVersion = `${major}.${minor}.${patch + 1}`;
} else if (arg === "minor") {
  nextVersion = `${major}.${minor + 1}.0`;
} else if (arg === "major") {
  nextVersion = `${major + 1}.0.0`;
} else if (/^v?\d+\.\d+\.\d+$/.test(arg)) {
  nextVersion = arg.replace(/^v/, "");
} else {
  console.error(`Invalid release argument '${arg}'. Expected patch, minor, major, or x.y.z.`);
  process.exit(1);
}

const tag = `v${nextVersion}`;
console.log(`Bumping Umuryango version: v${currentVersion} -> ${tag}`);

const updatedCargo = cargoToml.replace(
  /^version\s*=\s*"\d+\.\d+\.\d+"/m,
  `version = "${nextVersion}"`,
);
writeFileSync(cargoPath, updatedCargo, "utf8");

const pkgJson = JSON.parse(readFileSync(pkgPath, "utf8"));
pkgJson.version = nextVersion;
writeFileSync(pkgPath, `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

function runGit(args) {
  const res = spawnSync("git", args, { stdio: "inherit" });
  if (res.status !== 0) {
    console.error(`git ${args.join(" ")} failed with exit code ${res.status}`);
    process.exit(res.status ?? 1);
  }
}

runGit(["add", "Cargo.toml", "package.json"]);
runGit(["commit", "-m", `chore(release): ${tag}`]);
runGit(["tag", "-a", tag, "-m", `Release ${tag}`]);
runGit(["push", "origin", "HEAD", "--follow-tags"]);

console.log(
  `\nPublished ${tag}! GitHub Actions will build and attach the binary so running apps self-update.`,
);

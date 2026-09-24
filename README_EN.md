<div align="center">

# CC Switch Custom Edition

### Windows-focused configuration manager for Claude Code, Codex, and Pi

[![Version](https://img.shields.io/github/v/release/chansanya/cc-switch?color=blue&label=version)](https://github.com/chansanya/cc-switch/releases)
[![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)](https://github.com/chansanya/cc-switch/releases)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%202-orange.svg)](https://tauri.app/)

English | [中文](README.md) | [Custom Edition Notes](docs/custom-edition-zh.md) | [Changelog](CHANGELOG.md)

</div>

> This repository is a personal custom build based on upstream [farion1231/cc-switch](https://github.com/farion1231/cc-switch). This document describes only the features exposed by this branch. Upstream multi-app, cross-platform, routing, and managed-auth documentation does not apply to this edition.

## Purpose

This edition removes unused modules and focuses on a small Windows development workflow:

- **Claude Code** provider, Prompt, Skill, MCP, and directory management.
- **Codex** provider, model catalog, Prompt, Skill, MCP, and separate Windows / WSL configuration.
- **Pi** provider, Prompt, and Skill management.
- **One-way Windows → WSL mirroring**, with Windows remaining authoritative.

## Retained Features

### Provider management

- Add, edit, switch, sort, import, and export Claude Code, Codex, and Pi providers.
- Manage Codex custom models and `cc-switch-model-catalog.json`.
- SQLite persistence, atomic writes, and configuration backups.

### Windows / WSL mode

Configure a WSL UNC directory for Claude Code or Codex, for example:

```text
\\wsl.localhost\Ubuntu\home\dev\.codex
```

Behavior:

- Windows is the primary configuration; WSL changes are not backfilled.
- Codex providers can store separate Windows and WSL `config.toml` content.
- Windows-only sections such as `notify`, `desktop`, `windows`, `marketplaces`, and `plugins` are excluded from WSL.
- `model_catalog_json` and its generated catalog are synchronized when model mappings are enabled.
- MCP servers explicitly target Windows, WSL, or both. WSL unwraps supported Node commands from `cmd /c`; file paths are never rewritten.
- Skills are copied into WSL instead of using cross-filesystem symbolic links.

See [Windows / WSL dual-mode design](docs/win-wsl-dual-mode-design.md) for details.

### MCP, Prompts, and Skills

- Unified MCP management for Claude Code and Codex, with per-runtime targeting.
- Prompt management for `CLAUDE.md` and Codex / Pi `AGENTS.md`.
- Skill installation from repositories or ZIP files and projection into enabled apps.

## Removed or Hidden

This edition does not expose:

- Claude Desktop, Gemini CLI, Grok Build, OpenCode, OpenClaw, Hermes, or MiniMax Code on the main screen.
- Local proxy, routing takeover, failover controls, or their settings pages.
- The managed authentication center and account-management panels.
- In-app update checks, update badges, or updater actions in the About page.
- macOS, Linux, or Windows ARM64 release artifacts.

Provider API keys remain ordinary provider configuration and are not the removed managed-auth feature.

## Installation

Only unsigned **Windows x86_64** test artifacts are published:

- `.msi` installer
- `Windows-Portable.zip`

Download them from [Releases](https://github.com/chansanya/cc-switch/releases). Windows may display an unknown publisher warning because these personal builds are unsigned.

Requirements: Windows 10 or later. WSL mirroring requires an installed and accessible WSL2 distribution.

## Quick Start

1. Confirm the Windows Claude Code and Codex directories in Settings.
2. Optionally enter their WSL UNC mirror directories.
3. Add and switch providers from the Claude Code, Codex, or Pi page.
4. Edit separate Windows and WSL `config.toml` content in a Codex provider.
5. Choose Windows, WSL, or both for each MCP server.

## Development

The React renderer lives in `src/`; the Rust / Tauri backend lives in `src-tauri/`. Tests are under `tests/` and `src-tauri/tests/`.

```bash
pnpm install
pnpm dev
pnpm typecheck
pnpm test:unit
```

Rust checks:

```bash
cd src-tauri
cargo fmt --check
cargo clippy
cargo test
```

GitHub Actions for this branch validate Windows only and publish unsigned Windows x86_64 prerelease artifacts.

## Security

Never publish API keys, GitHub tokens, database passwords, SSH passwords, or OAuth tokens in issues, logs, screenshots, or example configurations. Revoke any credential immediately after accidental exposure.

## Upstream and License

Based on [farion1231/cc-switch](https://github.com/farion1231/cc-switch), licensed under the repository's [MIT License](LICENSE). Refer to the upstream repository for the original complete product documentation.

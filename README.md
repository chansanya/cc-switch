<div align="center">

# CC Switch 定制版

### 面向 Windows 的 Claude Code、Codex 与 Pi 配置管理工具

[![Version](https://img.shields.io/github/v/release/chansanya/cc-switch?color=blue&label=version)](https://github.com/chansanya/cc-switch/releases)
[![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)](https://github.com/chansanya/cc-switch/releases)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%202-orange.svg)](https://tauri.app/)

[English](README_EN.md) | 中文 | [定制版说明](docs/custom-edition-zh.md) | [更新日志](CHANGELOG.md)

</div>

> 本仓库是基于上游 [farion1231/cc-switch](https://github.com/farion1231/cc-switch) 的个人定制版本。当前文档只描述本分支实际保留的功能；上游的多应用、跨平台、代理路由与托管认证文档不适用于本版本。

## 定制目标

这个版本不是“全家桶”，而是把日常不用的模块收掉，只保留 Windows 开发环境中最常用的配置管理能力：

- **Claude Code**：供应商、Prompts、Skills、MCP 与配置目录管理。
- **Codex**：供应商、模型映射、Prompts、Skills、MCP，以及 Windows / WSL 双配置。
- **Pi**：供应商、Prompts 与 Skills 管理。
- **Windows → WSL 单向镜像**：Windows 保持主配置，WSL 使用独立目录与独立 Codex `config.toml`。

## 保留功能

### 供应商管理

- Claude Code、Codex、Pi 供应商的新增、编辑、切换、排序与导入导出。
- Codex 自定义模型与 `cc-switch-model-catalog.json` 管理。
- SQLite 持久化、原子写入与配置备份。

### Windows / WSL 双模式

在设置中为 Claude Code 或 Codex 配置 WSL UNC 目录，例如：

```text
\\wsl.localhost\Ubuntu\home\dev\.codex
```

同步规则：

- Windows 是主配置，不从 WSL 反向回填。
- Codex Provider 可分别编辑 Windows 和 WSL 的 `config.toml`。
- WSL 自动过滤 `notify`、`desktop`、`windows`、`marketplaces`、`plugins` 等 Windows 专属配置。
- `model_catalog_json` 与模型目录按模型映射同步。
- MCP 通过 `Windows / WSL` 运行环境标记决定写入目标；WSL 仅对受控 Node 命令移除 `cmd /c`，不转换文件路径。
- Skills 写入 WSL 时使用文件复制，不创建跨文件系统符号链接。

详细规则见 [Windows / WSL 双模式设计](docs/win-wsl-dual-mode-design.md)。

### MCP、Prompts 与 Skills

- MCP：统一管理 Claude Code 与 Codex 服务，并分别选择 Windows、WSL 或双端运行。
- Prompts：管理 `CLAUDE.md`、Codex / Pi 的 `AGENTS.md`。
- Skills：从仓库或 ZIP 安装，并投影到已启用的应用目录。

## 已移除或隐藏的功能

此定制版本不提供以下入口：

- Claude Desktop、Gemini CLI、Grok Build、OpenCode、OpenClaw、Hermes、MiniMax Code 主页面入口。
- 本地代理、路由接管、故障转移与相关设置页面。
- 托管认证中心及供应商表单中的账号管理入口。
- 应用内检查更新、自动更新提示与“关于”页面更新操作。
- macOS、Linux 与 Windows ARM64 发布制品。

API Key 等供应商基础凭据仍属于配置内容，不等同于已移除的“托管认证中心”。

## 安装

仅提供 **Windows x86_64** 无签名测试制品：

- `.msi` 安装包
- `Windows-Portable.zip` 便携版

从 [Releases](https://github.com/chansanya/cc-switch/releases) 下载。由于是个人无签名构建，Windows 可能显示未知发布者提示；请只使用本仓库生成的制品。

系统要求：Windows 10 或更高版本。WSL 镜像功能需要已安装并可访问的 WSL2 发行版。

## 快速使用

1. 打开设置，在“配置目录”中确认 Claude Code、Codex 的 Windows 主目录。
2. 如需 WSL，同一区域填写对应的 WSL UNC 镜像目录。
3. 在 Claude Code、Codex 或 Pi 页面新增供应商并切换。
4. Codex 供应商编辑页可分别维护 Windows 与 WSL `config.toml`。
5. 在 MCP 列表中为每个服务选择 Windows、WSL 或双端运行环境。

## 开发

前端位于 `src/`，Rust / Tauri 后端位于 `src-tauri/`，测试位于 `tests/` 与 `src-tauri/tests/`。

```bash
pnpm install
pnpm dev
pnpm typecheck
pnpm test:unit
```

Rust 检查：

```bash
cd src-tauri
cargo fmt --check
cargo clippy
cargo test
```

本定制分支的 GitHub Actions 仅验证 Windows，并生成 Windows x86_64 无签名 prerelease 制品。

## 安全提醒

不要在 Issue、日志、截图或示例配置中提交 API Key、GitHub Token、数据库密码、SSH 密码或 OAuth Token。凭据一旦公开，应立即撤销并重新生成。

## 上游与许可证

本项目基于 [farion1231/cc-switch](https://github.com/farion1231/cc-switch) 修改，遵循仓库中的 [MIT License](LICENSE)。上游历史说明与完整功能文档可在上游仓库查看。

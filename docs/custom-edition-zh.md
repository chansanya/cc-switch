# CC Switch 定制版范围

本文说明 `dev` 定制线与上游完整版的差异，适用于 2026-09-24 之后的 Windows 测试构建。

## 当前保留

| 模块 | Claude Code | Codex | Pi |
|---|---:|---:|---:|
| 供应商管理 | 支持 | 支持 | 支持 |
| Prompts | `CLAUDE.md` | `AGENTS.md` | `AGENTS.md` |
| Skills | 支持 | 支持 | 支持 |
| MCP | 支持 | 支持 | 无原生 MCP 注册表 |
| Windows / WSL 镜像 | 支持 | 支持 | 支持 |
| 供应商级 WSL 独立配置 | JSON | TOML | JSON |

## 供应商级 Windows / WSL 配置

Claude、Codex、Pi 的每个 Provider 都可保存 `wslEnabled` 与 `wslConfig`。Codex 结构为：

```text
settingsConfig.config     Windows config.toml
settingsConfig.wslConfig  WSL config.toml（可选）
```

供应商关闭 `wslEnabled` 时完全不写 WSL，但保留已保存的 `wslConfig`。Codex 开启后只使用独立 TOML，并在写入前移除以下 Windows 专属内容：

```text
notify
[desktop]
[windows]
[marketplaces.*]
[plugins.*]
[mcp_servers.*]
```

`[mcp_servers]` 由统一 MCP 数据库根据 `runtimeTargets.wsl` 单独生成；不做 Windows 路径到 `/mnt/*` 的自动转换。

## 已移除入口

- 其他应用主页面入口。
- 本地代理、路由接管与故障转移。
- 托管认证中心和账号管理面板。
- 应用内更新检查、更新徽标和 updater 操作。
- macOS、Linux、Windows ARM64 发布任务。

相关后端代码可能仍为兼容旧数据而存在，但定制版 UI 不再暴露这些能力。后续删除代码时，应按模块逐步清理，不能一把梭把数据库迁移和旧配置兼容也干碎了。

## 发布方式

通过 `v*` 测试标签触发 Windows x86_64 无签名 prerelease。发布不生成 updater 签名或 `latest.json`，仅用于个人下载安装测试。

# CC Switch 定制版用户手册

> 本手册入口仅描述当前 Windows 定制版。目录中的其他旧页面来自上游完整版，涉及代理路由、托管认证、其他应用或跨平台安装的内容不适用于本版本。

## 使用范围

- Claude Code、Codex、Pi 供应商管理
- Claude Code / Codex MCP 管理
- Prompts 与 Skills
- Codex Windows / WSL 独立配置
- Windows → WSL 单向镜像
- 配置备份、导入导出与云目录同步

## 推荐阅读

1. [定制版范围](../../custom-edition-zh.md)
2. [Windows / WSL 双模式设计](../../win-wsl-dual-mode-design.md)
3. [安装指南](./1-getting-started/1.2-installation.md)（仅参考 Windows 部分）
4. [添加供应商](./2-providers/2.1-add.md)（仅参考 Claude Code、Codex、Pi）
5. [MCP 管理](./3-extensions/3.1-mcp.md)
6. [Prompts 管理](./3-extensions/3.2-prompts.md)
7. [Skills 管理](./3-extensions/3.3-skills.md)

## 不适用内容

以下上游文档保留用于历史参考，但当前定制版没有对应入口：

- Claude Desktop、Gemini、Grok Build、OpenCode、OpenClaw、Hermes、MiniMax Code
- 本地代理、路由接管、故障转移、熔断与代理请求统计
- 托管认证中心
- 应用内检查更新和自动更新
- macOS、Linux 安装与发布

## 安全

示例、截图和 Issue 中不要包含真实 API Key、GitHub Token、SSH 密码、数据库密码或 OAuth Token。已经公开的凭据不能继续凑合用，必须撤销并重新生成。

# CC Switch - Windows / WSL 双模式兼容与同步技术方案 (V1)

## 1. 背景与核心问题

在 Windows 平台上开发时，开发者经常在 **Windows 宿主系统** 与 **WSL (Windows Subsystem for Linux)** 之间交替使用命令行工具（如 Claude Code、Codex CLI）。

CC Switch 在双模式场景下存在两项核心诉求：
1. **统一供应商与配置同步**：在 Windows 端切换 Provider、更新 Prompt 或安装 Skill 时，WSL 环境下的对应目录能够实时镜像生效，无需每次手动修改配置目录。
2. **MCP 命令平台适配与运行环境隔离**：
   - Windows 宿主环境下，Node CLI（如 `npx`）需通过 `cmd /c` 执行；WSL 下原生支持 POSIX，需移除 `cmd /c`。
   - 不同环境使用的本地路径或工作区各不相同（例如 Windows 本地工具路径在 WSL 内并不适用），因此不能也不应对路径进行盲目映射，而应由 MCP 条目显式标记运行环境（Windows / WSL）。

---

## 2. 方案选型与边界设计

经过架构评估，本方案采用 **方案 A：单向镜像模式 (Mirror Mode)**，并在 V1 严格限定实现边界：

| 维度 | V1 设计决策 | 理由与收益 |
| :--- | :--- | :--- |
| **支持范围** | 仅支持 **Claude Code**、**Codex** 与 **Pi** | 聚焦最高频开发工具，避免扩大改动面 |
| **同步流向** | **单向镜像：Windows 主目录 → WSL 镜像目录** | 保持单一事实源（SSOT），坚决杜绝双向回填引发的版本脑裂与锁竞争 |
| **错误隔离** | **容错降级 (Soft Fail)** | WSL 离线或 UNC 路径超时绝不阻断或回滚 Windows 主写入 |
| **MCP 环境标记** | 独立 `runtimeTargets: { windows, wsl }` 字段 | 解耦“应用维度 (apps)”与“平台环境维度”，列表与编辑页直接切换 |
| **供应商 WSL 配置** | Claude、Codex、Pi 按供应商独立启用，分别保存 JSON / TOML / JSON | 避免 Windows 与 WSL 路径、运行时字段互相污染 |
| **MCP 命令包装** | Windows 补齐/保留 `cmd /c`，WSL 仅脱壳受控 Node 命令 | 针对 `npx`, `npm`, `yarn`, `pnpm`, `node`, `bun`, `deno` 处理，保留非 Node 命令的原意 |
| **Skills 同步** | 镜像到 WSL 时**强制文件递归复制** | 跨 Windows NTFS 与 WSL 9P/ext4 文件系统的符号链接不稳定，文件复制最可靠 |
| **Prompts 同步** | 分别写入 `CLAUDE.md` 与 `AGENTS.md` | 两边均可直接被对应 CLI 识别生效 |
| **构建与验证** | 不在本地安装开发环境，使用 Docker 或 GitHub CI 验证 | 保持宿主机纯净，发布制品完全由 GitHub Actions 统一生成 |

---

## 3. 详细设计规范

### 3.1 总体架构

```text
┌────────────────────────────────────────────────────────┐
│                      CC Switch                         │
│            SQLite / UI (单源事实标准)                  │
└──────────────────────────┬─────────────────────────────┘
                           │ 触发写入 (Provider / MCP / Skill / Prompt)
                           ▼
               ┌───────────────────────┐
               │    WSL Mirror Layer   │
               └───┬───────────────┬───┘
                   │               │
      [Target 1: Windows (主)]     [Target 2: WSL (镜像, 可选)]
                   │               │
                   ▼               ▼
          【Windows 配置目录】     【WSL UNC 镜像目录】
          C:\Users\...\.codex      \\wsl.localhost\...\.codex
          -------------------      --------------------------
          • 写入 config.toml       • 写入 config.toml
          • 写入 auth.json         • 写入 auth.json (有有效凭证时)
          • 写入 model-catalog     • 按模型映射生成 model-catalog
          • MCP: 保持 cmd /c       • MCP: 移除 cmd /c (受控命令)
          • Prompts: AGENTS.md     • Prompts: AGENTS.md
          • Skills: Symlink/Copy   • Skills: 强制递归复制
```

### 3.2 配置项扩展
在 `AppSettings` 及前端 `Settings` 中新增：
- `claudeWslMirrorDir`: 字符串，如 `\\wsl.localhost\Ubuntu\home\dev\.claude`
- `codexWslMirrorDir`: 字符串，如 `\\wsl.localhost\Ubuntu\home\dev\.codex`
- `piWslMirrorDir`: 字符串，如 `\\wsl.localhost\Ubuntu\home\dev\.pi\agent`

设置界面在对应目录项旁提供输入框与 WSL 占位示例，支持保存时自动创建目标目录。

### 3.3 供应商级 WSL 独立配置

每个 Claude、Codex、Pi Provider 保存 `wslEnabled` 与 `wslConfig`。开启前要求对应镜像目录已配置；关闭时不写 WSL，并保留独立配置内容。Claude/Pi 使用 JSON，Codex 使用 TOML。

### 3.4 Codex WSL 投影约束

Codex Provider 的 `settingsConfig.wslConfig` 保存 WSL 专用 TOML。启用后只使用这份独立配置，不再从 Windows `config.toml` 回退生成。

写入前会过滤 `notify`、`desktop`、`windows`、`marketplaces`、`plugins` 与整个 `[mcp_servers]`。`model_catalog_json` 会保留，并在引用 `cc-switch-model-catalog.json` 时同步模型目录。WSL MCP 仅由 `runtimeTargets.wsl` 重新投影。

### 3.5 MCP 运行环境自适应
统一 MCP 数据模型扩展：
```typescript
interface McpRuntimeTargets {
  windows: boolean;
  wsl: boolean;
}

interface McpServer {
  // ... 现有字段 (apps, server 等)
  runtimeTargets: McpRuntimeTargets; // 历史数据缺省时默认 { windows: true, wsl: false }
}
```
- 投影逻辑：
  - Windows 端仅投影 `runtimeTargets.windows == true` 的服务，并在 stdio 模式下确保 Node CLI 命令包含 `cmd /c`。
  - WSL 端仅投影 `runtimeTargets.wsl == true` 的服务，并在识别到 `cmd /c <node_cli>` 时自动脱壳为原生命令。
  - 两侧相互独立：若某环境未启用该 MCP，则从对应配置文件中移除该条目。
  - 至少保留一个运行环境目标，禁止将所有目标同时置空。

### 3.6 异常处理与容错原则
- WSL 镜像写入采用独立异常捕获，若网络共享未就绪、路径不存在或权限不足，输出 `log::warn!`，主操作照常向前端返回成功。
- WSL 侧文件的任何外部手工修改不进行反向同步与回填，下次 CC Switch 执行写入时将覆盖相关管理文件。

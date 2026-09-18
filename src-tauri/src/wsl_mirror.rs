//! Windows -> WSL 单向镜像服务
//!
//! 负责将 Windows 本地的主配置（Provider、MCP、Skills、Prompts）
//! 镜像投递至指定的 WSL UNC 目录。
//! 遵循容错降级原则：WSL 目录写入失败仅记录警告日志，绝不回滚或阻塞 Windows 主写入。

use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

use crate::app_config::AppType;
use crate::error::AppError;

pub const CONTROLLED_NODE_COMMANDS: &[&str] =
    &["npx", "npm", "yarn", "pnpm", "node", "bun", "deno"];

/// 检测路径是否为 WSL UNC 路径（如 \\wsl$\Ubuntu\... 或 \\wsl.localhost\Ubuntu\...）
pub fn is_wsl_mirror_path(path: &Path) -> bool {
    let s = path.to_string_lossy();
    let lower = s.to_ascii_lowercase();
    lower.starts_with(r"\\wsl$\")
        || lower.starts_with(r"\\wsl.localhost\")
        || lower.starts_with("//wsl$/")
        || lower.starts_with("//wsl.localhost/")
}

/// 确保 WSL 镜像目录存在
pub fn ensure_wsl_mirror_dir(dir: &Path) -> Result<(), AppError> {
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| AppError::io(dir, e))?;
    }
    Ok(())
}

/// WSL 平台：对受控的 Node CLI 命令进行脱壳（去掉外层的 `cmd /c`）
///
/// 规则：
/// command == "cmd" / "cmd.exe" 且 args[0] == "/c" 且 args[1] 为已知 Node 命令时，
/// 提取 args[1] 作为 command，args[2..] 作为 args。
pub fn unwrap_command_for_wsl(obj: &mut Map<String, Value>) {
    let server_type = obj.get("type").and_then(|v| v.as_str()).unwrap_or("stdio");
    if server_type != "stdio" {
        return;
    }

    let Some(cmd) = obj.get("command").and_then(|v| v.as_str()) else {
        return;
    };

    if !cmd.eq_ignore_ascii_case("cmd") && !cmd.eq_ignore_ascii_case("cmd.exe") {
        return;
    }

    let Some(args) = obj.get("args").and_then(|v| v.as_array()) else {
        return;
    };

    if args.len() < 2 {
        return;
    }

    let Some(first_arg) = args[0].as_str() else {
        return;
    };
    if !first_arg.eq_ignore_ascii_case("/c") {
        return;
    }

    let Some(second_arg) = args[1].as_str() else {
        return;
    };

    let inner_stem = Path::new(second_arg)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(second_arg);

    if !CONTROLLED_NODE_COMMANDS
        .iter()
        .any(|&c| inner_stem.eq_ignore_ascii_case(c))
    {
        return;
    }

    let remaining_args = args[2..].to_vec();
    obj.insert("command".into(), Value::String(second_arg.to_string()));
    obj.insert("args".into(), Value::Array(remaining_args));
}

/// Windows 平台：将受控 Node CLI 包装为 `cmd /c`
pub fn wrap_command_for_windows(obj: &mut Map<String, Value>) {
    let server_type = obj.get("type").and_then(|v| v.as_str()).unwrap_or("stdio");
    if server_type != "stdio" {
        return;
    }

    let Some(cmd) = obj.get("command").and_then(|v| v.as_str()) else {
        return;
    };

    if cmd.eq_ignore_ascii_case("cmd") || cmd.eq_ignore_ascii_case("cmd.exe") {
        return;
    }

    let cmd_name = Path::new(cmd)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(cmd);

    let needs_wrap = CONTROLLED_NODE_COMMANDS
        .iter()
        .any(|&c| cmd_name.eq_ignore_ascii_case(c));

    if !needs_wrap {
        return;
    }

    let original_args = obj
        .get("args")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut new_args = vec![Value::String("/c".into()), Value::String(cmd.into())];
    new_args.extend(original_args);

    obj.insert("command".into(), Value::String("cmd".into()));
    obj.insert("args".into(), Value::Array(new_args));
}

/// 递归复制目录
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// 镜像 Claude Live 配置到 WSL（若已配置）
pub fn mirror_claude_live_if_enabled(settings: &Value) {
    let Some(wsl_dir) = crate::settings::get_claude_wsl_mirror_dir() else {
        return;
    };

    if let Err(e) = ensure_wsl_mirror_dir(&wsl_dir) {
        log::warn!("创建 Claude WSL 镜像目录失败: {}: {e}", wsl_dir.display());
        return;
    }

    let target_file = wsl_dir.join("settings.json");
    if let Err(e) = crate::config::write_json_file(&target_file, settings) {
        log::warn!(
            "写入 Claude WSL 镜像配置失败: {}: {e}",
            target_file.display()
        );
    } else {
        log::info!("已同步 Claude 配置到 WSL 镜像: {}", target_file.display());
    }
}

/// 镜像 Codex Live 配置到 WSL（若已配置）
pub fn mirror_codex_live_if_enabled(
    auth: Option<&Value>,
    remove_auth: bool,
    config_text: Option<&str>,
) {
    let Some(wsl_dir) = crate::settings::get_codex_wsl_mirror_dir() else {
        return;
    };

    if let Err(e) = ensure_wsl_mirror_dir(&wsl_dir) {
        log::warn!("创建 Codex WSL 镜像目录失败: {}: {e}", wsl_dir.display());
        return;
    }

    let config_path = wsl_dir.join("config.toml");
    let auth_path = wsl_dir.join("auth.json");

    if let Some(text) = config_text {
        if let Err(e) = crate::config::write_text_file(&config_path, text) {
            log::warn!(
                "写入 Codex WSL 镜像 config.toml 失败: {}: {e}",
                config_path.display()
            );
        } else {
            log::info!(
                "已同步 Codex config.toml 到 WSL 镜像: {}",
                config_path.display()
            );
        }

        // 同步模型目录（若 config.toml 引用了 cc-switch-model-catalog.json）
        let catalog_filename = crate::codex_config::CC_SWITCH_CODEX_MODEL_CATALOG_FILENAME;
        if text.contains(catalog_filename) {
            let src_catalog = crate::codex_config::get_codex_model_catalog_path();
            if src_catalog.exists() {
                let dst_catalog = wsl_dir.join(catalog_filename);
                if let Ok(content) = fs::read(&src_catalog) {
                    if let Err(e) = crate::config::atomic_write(&dst_catalog, &content) {
                        log::warn!(
                            "同步 Codex model catalog 到 WSL 镜像失败: {}: {e}",
                            dst_catalog.display()
                        );
                    }
                }
            }
        }
    }

    if let Some(auth_val) = auth {
        if let Err(e) = crate::config::write_json_file(&auth_path, auth_val) {
            log::warn!(
                "写入 Codex WSL 镜像 auth.json 失败: {}: {e}",
                auth_path.display()
            );
        } else {
            log::info!(
                "已同步 Codex auth.json 到 WSL 镜像: {}",
                auth_path.display()
            );
        }
    } else if remove_auth && auth_path.exists() {
        let _ = crate::config::delete_file(&auth_path);
    }
}

/// 镜像 Prompt 内容到 WSL（若已配置）
pub fn mirror_prompt_if_enabled(app: &AppType, content: &str) {
    let (target_file, app_name) = match app {
        AppType::Claude => {
            let Some(wsl_dir) = crate::settings::get_claude_wsl_mirror_dir() else {
                return;
            };
            (wsl_dir.join("CLAUDE.md"), "Claude")
        }
        AppType::Codex => {
            let Some(wsl_dir) = crate::settings::get_codex_wsl_mirror_dir() else {
                return;
            };
            (wsl_dir.join("AGENTS.md"), "Codex")
        }
        _ => return,
    };

    if let Some(parent) = target_file.parent() {
        if let Err(e) = ensure_wsl_mirror_dir(parent) {
            log::warn!(
                "创建 {app_name} WSL 提示词镜像目录失败: {}: {e}",
                parent.display()
            );
            return;
        }
    }

    if let Err(e) = crate::config::write_text_file(&target_file, content) {
        log::warn!(
            "同步 Prompt 到 {app_name} WSL 镜像失败: {}: {e}",
            target_file.display()
        );
    } else {
        log::info!(
            "已同步 Prompt 到 {app_name} WSL 镜像: {}",
            target_file.display()
        );
    }
}

/// 镜像 Skill 到 WSL 镜像目录（强制使用文件复制，避免跨文件系统符号链接）
pub fn mirror_skill_if_enabled(app: &AppType, directory: &str, source_dir: &Path) {
    let mirror_dir = match app {
        AppType::Claude => crate::settings::get_claude_wsl_mirror_dir(),
        AppType::Codex => crate::settings::get_codex_wsl_mirror_dir(),
        _ => None,
    };

    let Some(wsl_dir) = mirror_dir else {
        return;
    };

    let skills_dir = wsl_dir.join("skills");
    if let Err(e) = ensure_wsl_mirror_dir(&skills_dir) {
        log::warn!(
            "创建 WSL skills 镜像目录失败: {}: {e}",
            skills_dir.display()
        );
        return;
    }

    let dest = skills_dir.join(directory);
    if let Err(e) = copy_dir_recursive(source_dir, &dest) {
        log::warn!(
            "复制 Skill 到 WSL 镜像失败: {} -> {}: {e}",
            source_dir.display(),
            dest.display()
        );
    } else {
        log::info!("已复制 Skill {} 到 WSL 镜像: {}", directory, dest.display());
    }
}

/// 从 WSL 镜像目录中删除指定 Skill
pub fn remove_skill_mirror_if_enabled(app: &AppType, directory: &str) {
    let mirror_dir = match app {
        AppType::Claude => crate::settings::get_claude_wsl_mirror_dir(),
        AppType::Codex => crate::settings::get_codex_wsl_mirror_dir(),
        _ => None,
    };

    let Some(wsl_dir) = mirror_dir else {
        return;
    };

    let dest = wsl_dir.join("skills").join(directory);
    if dest.exists() {
        if let Err(e) = fs::remove_dir_all(&dest) {
            log::warn!("从 WSL 镜像删除 Skill 失败: {}: {e}", dest.display());
        } else {
            log::info!("已从 WSL 镜像删除 Skill {}: {}", directory, dest.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_is_wsl_mirror_path() {
        assert!(is_wsl_mirror_path(Path::new(
            r"\\wsl$\Ubuntu\home\user\.codex"
        )));
        assert!(is_wsl_mirror_path(Path::new(
            r"\\wsl.localhost\Ubuntu\home\user\.claude"
        )));
        assert!(is_wsl_mirror_path(Path::new(r"//wsl$/Ubuntu/home/user")));
        assert!(is_wsl_mirror_path(Path::new(
            r"//wsl.localhost/Ubuntu/home/user"
        )));
        assert!(!is_wsl_mirror_path(Path::new(r"C:\Users\user\.codex")));
        assert!(!is_wsl_mirror_path(Path::new(r"/home/user/.codex")));
    }

    #[test]
    fn test_unwrap_command_for_wsl() {
        let mut obj = json!({
            "type": "stdio",
            "command": "cmd",
            "args": ["/c", "npx", "-y", "@modelcontextprotocol/server-filesystem", "/home/dev/file"]
        })
        .as_object()
        .unwrap()
        .clone();

        unwrap_command_for_wsl(&mut obj);

        assert_eq!(obj["command"], "npx");
        assert_eq!(
            obj["args"],
            json!([
                "-y",
                "@modelcontextprotocol/server-filesystem",
                "/home/dev/file"
            ])
        );
    }

    #[test]
    fn test_unwrap_command_for_wsl_ignores_non_node_cmds() {
        let mut obj = json!({
            "type": "stdio",
            "command": "cmd",
            "args": ["/c", "my-custom-script.bat"]
        })
        .as_object()
        .unwrap()
        .clone();

        unwrap_command_for_wsl(&mut obj);

        // 保持原样，不脱壳非受控 Node 命令
        assert_eq!(obj["command"], "cmd");
        assert_eq!(obj["args"], json!(["/c", "my-custom-script.bat"]));
    }

    #[test]
    fn test_wrap_command_for_windows() {
        let mut obj = json!({
            "type": "stdio",
            "command": "npx",
            "args": ["-y", "@upstash/context7-mcp"]
        })
        .as_object()
        .unwrap()
        .clone();

        wrap_command_for_windows(&mut obj);

        assert_eq!(obj["command"], "cmd");
        assert_eq!(
            obj["args"],
            json!(["/c", "npx", "-y", "@upstash/context7-mcp"])
        );
    }
}

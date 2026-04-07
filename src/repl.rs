/*!
 * REPL 交互界面模块
 *
 * 对应源码: src/main.tsx - REPL 实现
 *
 * 功能：
 * - 交互式命令行界面
 * - 对话历史管理
 * - 命令处理
 */

use crate::api::ClaudeClient;
use crate::types::{ConversationHistory, Message};
use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustylineResult};
use std::env;

/// 启动 REPL 交互界面
pub async fn start_repl(api_key: &str) -> Result<()> {
    // 打印使用说明
    print_instructions(api_key);

    // 创建 API 客户端
    let client = ClaudeClient::new(api_key)?;

    // 创建对话历史
    let mut history = ConversationHistory::new();

    // 创建 readline 编辑器
    let mut rl = DefaultEditor::new()?;

    // 主循环
    loop {
        // 读取用户输入
        let readline = rl.readline(&format!("\n{} ", "💬 You >".green().bold()));

        match readline {
            Ok(line) => {
                let input = line.trim();

                // 跳过空输入
                if input.is_empty() {
                    continue;
                }

                // 添加到 readline 历史
                let _ = rl.add_history_entry(input);

                // 处理命令
                if input.starts_with('/') {
                    if handle_command(input, &mut history).await {
                        break; // 退出
                    }
                    continue;
                }

                // 发送消息到 Claude
                if let Err(e) = send_message(&client, input, &mut history).await {
                    eprintln!("\n{} {}", "❌ 错误:".red().bold(), e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                println!("\n{}", "使用 /exit 或 /quit 退出".yellow());
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!("\n{}", "👋 再见!".cyan());
                break;
            }
            Err(err) => {
                eprintln!("{} {:?}", "❌ 读取输入错误:".red().bold(), err);
                break;
            }
        }
    }

    Ok(())
}

/// 打印使用说明
fn print_instructions(api_key: &str) {
    println!("{}", "📝 使用说明:".cyan().bold());
    println!("  - 输入消息后按 Enter 发送");
    println!("  - 输入 {} 或 {} 退出", "/exit".yellow(), "/quit".yellow());
    println!("  - 输入 {} 清空对话历史", "/clear".yellow());
    println!("  - 输入 {} 查看对话历史", "/history".yellow());
    println!("  - 输入 {} 显示帮助", "/help".yellow());
    println!();

    let model = env::var("CLAUDE_MODEL").unwrap_or_else(|_| "claude-opus-4-20250514".to_string());
    println!("{} {}", "🔧 当前模型:".cyan().bold(), model.white());

    let masked_key = if api_key.len() > 8 {
        format!("{}...", &api_key[..8])
    } else {
        "***".to_string()
    };
    println!("{} {}", "🔑 API Key:".cyan().bold(), masked_key.white());
    println!();
}

/// 发送消息到 Claude
async fn send_message(
    client: &ClaudeClient,
    input: &str,
    history: &mut ConversationHistory,
) -> Result<()> {
    // 添加用户消息到历史
    history.add_user_message(input);

    println!("\n{}", "🤖 Claude 正在思考...\n".yellow());

    // 调用 API (流式响应)
    let response = client.query_streaming(input, history.get_messages()).await?;

    // 添加助手响应到历史
    history.add_assistant_message(&response);

    println!(); // 换行
    Ok(())
}

/// 处理命令
/// 返回 true 表示退出
async fn handle_command(command: &str, history: &mut ConversationHistory) -> bool {
    match command.to_lowercase().as_str() {
        "/exit" | "/quit" => {
            println!("\n{}", "👋 再见!".cyan());
            return true;
        }

        "/clear" => {
            history.clear();
            println!("\n{}", "✅ 对话历史已清空".green());
        }

        "/history" => {
            println!("\n{}", "📜 对话历史:".cyan().bold());
            if history.is_empty() {
                println!("  {}", "(空)".dimmed());
            } else {
                match history.to_json() {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("{} {}", "❌ 序列化失败:".red(), e),
                }
            }
        }

        "/help" => {
            print_help();
        }

        "/count" => {
            println!(
                "\n{} {}",
                "📊 消息数量:".cyan().bold(),
                history.len()
            );
        }

        _ => {
            println!("\n{} {}", "❌ 未知命令:".red().bold(), command);
            println!("输入 {} 查看可用命令", "/help".yellow());
        }
    }

    false
}

/// 打印帮助信息
fn print_help() {
    println!("\n{}", "📖 可用命令:".cyan().bold());
    println!();
    println!("  {}          - 退出程序", "/exit, /quit".yellow());
    println!("  {}            - 清空对话历史", "/clear".yellow());
    println!("  {}          - 查看对话历史 (JSON 格式)", "/history".yellow());
    println!("  {}             - 显示此帮助信息", "/help".yellow());
    println!("  {}            - 显示消息数量", "/count".yellow());
    println!();
}

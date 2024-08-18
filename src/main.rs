use anyhow::{Context, Result};
use c2p::template::{
    copy_to_clipboard, handle_undefined_variables, handlebars_setup, render_template,
    template_contains_variables, write_to_file,
};
use clap::Parser;
use colored::*;
use env_logger::Builder;
use inquire::{Select, Text};
use log::LevelFilter;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_TEMPLATE_NAME: &str = "default";
const CUSTOM_TEMPLATE_NAME: &str = "custom";

const TEMPLATES: &[(&str, &str, &str)] = &[
    (
        "write-git-commit",
        include_str!("../templates/write-git-commit.hbs"),
        "撰寫 Git Commit 的摘要",
    ),
    (
        "write-github-pull-request",
        include_str!("../templates/write-github-pull-request.hbs"),
        "撰寫 Git Pull Request 的摘要",
    ),
    (
        "write-github-readme",
        include_str!("../templates/write-github-readme.hbs"),
        "撰寫 README 文件",
    ),
    (
        "write-github-changelog-daily",
        include_str!("../templates/write-github-changelog-daily.hbs"),
        "撰寫「以每天總結一次」ChangeLog 文件",
    ),
    (
        "write-github-changelog-biweekly",
        include_str!("../templates/write-github-changelog-biweekly.hbs"),
        "撰寫「以每兩周總結一次」ChangeLog 文件",
    ),
    (
        "document-the-code",
        include_str!("../templates/document-the-code.hbs"),
        "生成代碼文檔",
    ),
    (
        "find-security-vulnerabilities",
        include_str!("../templates/find-security-vulnerabilities.hbs"),
        "查找安全漏洞",
    ),
    (
        "clean-up-code",
        include_str!("../templates/clean-up-code.hbs"),
        "清理代碼",
    ),
    (
        "binary-exploitation-ctf-solver",
        include_str!("../templates/binary-exploitation-ctf-solver.hbs"),
        "解決二進制利用 CTF 問題",
    ),
    (
        "cryptography-ctf-solver",
        include_str!("../templates/cryptography-ctf-solver.hbs"),
        "解決密碼學 CTF 問題",
    ),
    (
        "reverse-engineering-ctf-solver",
        include_str!("../templates/reverse-engineering-ctf-solver.hbs"),
        "解決逆向工程 CTF 問題",
    ),
    (
        "web-ctf-solver",
        include_str!("../templates/web-ctf-solver.hbs"),
        "解決 Web CTF 問題",
    ),
    (
        "fix-bugs",
        include_str!("../templates/fix-bugs.hbs"),
        "修復 Bug",
    ),
    (
        "improve-performance",
        include_str!("../templates/improve-performance.hbs"),
        "提升性能",
    ),
    (
        "refactor",
        include_str!("../templates/refactor.hbs"),
        "重構代碼",
    ),
];

#[derive(Parser)]
#[clap(
    name = "c2p",
    version = "2.1.1",
    author = "Mufeed VH & Olivier D & Jerome Leong"
)]
struct Cli {
    /// Path to the codebase directory
    #[arg()]
    path: PathBuf,

    /// Patterns to include
    #[clap(long)]
    include: Option<String>,

    /// Patterns to exclude
    #[clap(long)]
    exclude: Option<String>,

    /// Include files in case of conflict between include and exclude patterns
    #[clap(long)]
    include_priority: bool,

    /// Exclude files/folders from the source tree based on exclude patterns
    #[clap(long)]
    exclude_from_tree: bool,

    /// Optional tokenizer to use for token count
    ///
    /// Supported tokenizers: o200k(default), cl100k, p50k, p50k_edit, r50k, gpt2
    #[clap(short = 'c', long)]
    encoding: Option<String>,

    /// Optional output file path
    #[clap(short, long)]
    output: Option<String>,

    /// Add line numbers to the source code
    #[clap(short = 'n', long)]
    line_number: bool,

    /// Disable wrapping code inside markdown code blocks
    #[clap(long)]
    no_codeblock: bool,

    /// Use relative paths instead of absolute paths, including the parent directory
    #[clap(long)]
    relative_paths: bool,

    /// Optional Disable copying to clipboard
    #[clap(long)]
    no_clipboard: bool,

    /// Optional String to a default template
    #[clap(short, long, num_args = 0..=1)]
    template: Option<Option<String>>,

    /// Optional Path to a custom Handlebars template
    #[clap(long)]
    hbs: Option<PathBuf>,

    /// Print output as JSON
    #[clap(long)]
    json: bool,

    /// Language to use for the response
    #[clap(short, long)]
    lang: Option<String>,
}

fn get_predefined_template(template_name: &str) -> Result<(String, String)> {
    TEMPLATES
        .iter()
        .find(|(name, _, _)| *name == template_name)
        .map(|(name, content, _)| (content.to_string(), name.to_string()))
        .ok_or_else(|| anyhow::anyhow!("預定義模板 '{}' 未找到", template_name))
}

fn get_custom_template(template_path: &Path) -> Result<(String, String)> {
    let content = fs::read_to_string(template_path)
        .with_context(|| format!("無法讀取自定義模板文件: {:?}", template_path))?;
    Ok((content, CUSTOM_TEMPLATE_NAME.to_string()))
}

fn parse_patterns(patterns: &Option<String>) -> Vec<String> {
    patterns
        .as_ref()
        .filter(|p| !p.is_empty())
        .map(|p| p.split(',').map(str::trim).map(String::from).collect())
        .unwrap_or_default()
}

fn get_git_diff_branch(args: &Cli, template_content: &str) -> Result<String> {
    if template_contains_variables(template_content, &["git_diff_branch"]) {
        log::info!("生成兩個分支之間的 git diff...");
        let branches = prompt_for_branches();
        if branches.len() != 2 {
            return Err(anyhow::anyhow!("請提供兩個分支，以逗號分隔。"));
        }
        Ok(
            c2p::git::get_git_diff_between_branches(&args.path, &branches[0], &branches[1])
                .unwrap_or_default(),
        )
    } else {
        Ok(String::new())
    }
}

fn get_git_log_date(path: &Path, template_content: &str) -> Result<String> {
    if !template_contains_variables(template_content, &["git_log_date"]) {
        return Ok(String::new());
    }

    log::info!("正在獲取指定日期範圍的 git log...");
    let date_range = prompt_for_date_range();

    log::info!("正在處理 git log...");
    c2p::git::get_git_log_by_date_range(path, &date_range)
}

fn print_json_output(
    rendered: &str,
    path: &PathBuf,
    token_count: usize,
    model_info: &str,
    paths: &[String],
) -> Result<String> {
    let json_output = json!({
        "prompt": rendered,
        "directory_name": c2p::path::label(path),
        "token_count": token_count,
        "model_info": model_info,
        "files": paths,
    });
    let json_string = serde_json::to_string_pretty(&json_output)?;
    Ok(json_string)
}

fn print_normal_output(token_count: usize, model_info: &str) {
    println!(
        "{}{}{} Token 數量: {}, 模型資訊: {}",
        "[".bold().white(),
        "i".bold().blue(),
        "]".bold().white(),
        token_count.to_string().bold().yellow(),
        model_info
    );
}

fn copy_to_clipboard_with_feedback(rendered: &str) {
    match copy_to_clipboard(rendered) {
        Ok(_) => {
            println!(
                "{}{}{} {}",
                "[".bold().white(),
                "✓".bold().green(),
                "]".bold().white(),
                "成功複製到剪貼板。".green()
            );
        }
        Err(e) => {
            eprintln!(
                "{}{}{} {}",
                "[".bold().white(),
                "!".bold().red(),
                "]".bold().white(),
                format!("複製到剪貼板失敗: {}", e).red()
            );
            println!("{}", rendered);
        }
    }
}

fn prompt_for_branches() -> Vec<String> {
    let branch1 = Text::new("請輸入第一個分支名稱:")
        .prompt()
        .unwrap_or_default();
    let branch2 = Text::new("請輸入第二個分支名稱:")
        .prompt()
        .unwrap_or_default();
    vec![branch1, branch2]
}

// 添加新的函數來提示用戶輸入日期範圍
fn prompt_for_date_range() -> String {
    let start_date = Text::new("請輸入開始日期 (YYYY-MM-DD):")
        .prompt()
        .unwrap_or_default();
    let end_date = Text::new("請輸入結束日期 (YYYY-MM-DD):")
        .prompt()
        .unwrap_or_default();
    format!("{}..{}", start_date, end_date)
}

fn select_template() -> Result<(String, String)> {
    let max_name_length = TEMPLATES
        .iter()
        .map(|(name, _, _)| name.len())
        .max()
        .unwrap_or(0);

    let options: Vec<String> = TEMPLATES
        .iter()
        .map(|(name, _, description)| {
            let padding = " ".repeat(max_name_length - name.len());
            format!("{} {} - {}", name, padding, description)
        })
        .collect();

    let selection = Select::new("請選擇一個模板:", options).prompt()?;

    // 從選擇中提取模板名稱
    let template_name = selection
        .split(" - ")
        .next()
        .unwrap_or_default()
        .trim()
        .to_string();

    get_predefined_template(&template_name)
}

fn main() -> Result<()> {
    Builder::new().filter_level(LevelFilter::Info).init();
    let args = Cli::parse();

    let (template_content, template_name) = if let Some(hbs_path) = &args.hbs {
        // 使用自定義模板文件
        get_custom_template(Path::new(hbs_path))?
    } else if let Some(Some(template_name)) = &args.template {
        // 直接使用 -t <template name> 指定的模板
        get_predefined_template(template_name)?
    } else if args.template.is_some() {
        // 當 -t 參數存在時，顯示模板表格並讓用戶選擇
        select_template()?
    } else {
        // 使用默認模板
        (
            include_str!("default_template.hbs").to_string(),
            DEFAULT_TEMPLATE_NAME.to_string(),
        )
    };

    let handlebars = handlebars_setup(&template_content, &template_name)?;

    log::info!("遍歷目錄並構建樹...");

    let include_patterns = parse_patterns(&args.include);
    let exclude_patterns = parse_patterns(&args.exclude);

    let (tree, files) = c2p::path::traverse_directory(
        &args.path,
        &include_patterns,
        &exclude_patterns,
        args.include_priority,
        args.line_number,
        args.relative_paths,
        args.exclude_from_tree,
        args.no_codeblock,
    )
    .map_err(|e| {
        log::error!("失敗!");
        anyhow::anyhow!("無法構建目錄樹: {}", e)
    })?;

    let git_diff = if template_contains_variables(&template_content, &["git_diff"]) {
        log::info!("生成 git diff...");
        match c2p::git::get_git_diff(&args.path) {
            Ok(diff) => {
                if diff.is_empty() {
                    log::info!("沒有檢測到未暫存的更改。");
                } else {
                    log::info!("成功獲取 git diff 的內容。");
                }
                diff
            }
            Err(e) => {
                log::error!("獲取 git diff 時出錯: {}", e);
                String::new()
            }
        }
    } else {
        String::new()
    };

    let git_diff_branch = get_git_diff_branch(&args, &template_content)?;
    let git_log_date = get_git_log_date(&args.path, &template_content)?;

    log::info!("完成!");

    let mut data = json!({
        "absolute_code_path": c2p::path::label(&args.path),
        "source_tree": tree,
        "files": files,
        "git_diff": git_diff,
        "git_diff_branch": git_diff_branch,
        "git_log_date": git_log_date
    });

    log::debug!("JSON 數據: {}", serde_json::to_string_pretty(&data)?);

    handle_undefined_variables(&mut data, &template_content)?;

    let mut rendered = render_template(&handlebars, &template_name, &data)?;

    if let Some(lang) = &args.lang {
        rendered.push_str(&format!("\nYou must use {} language to reply", lang));
    }

    let token_count = {
        let bpe = c2p::token::get_tokenizer(&args.encoding);
        bpe.encode_with_special_tokens(&rendered).len()
    };

    let paths: Vec<String> = files
        .iter()
        .filter_map(|file| file.get("path").and_then(|p| p.as_str()).map(String::from))
        .collect();

    let model_info = c2p::token::get_model_info(&args.encoding);

    let rendered = if args.json {
        print_json_output(&rendered, &args.path, token_count, model_info, &paths)?
    } else {
        rendered
    };

    print_normal_output(token_count, model_info);

    if !args.no_clipboard {
        copy_to_clipboard_with_feedback(&rendered);
    }

    if let Some(output_path) = &args.output {
        write_to_file(output_path, &rendered)?;
    }

    Ok(())
}

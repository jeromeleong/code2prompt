use anyhow::{Context, Result};
use c2p::{
    copy_to_clipboard, get_git_diff, get_git_diff_between_branches, get_git_log, get_model_info,
    get_tokenizer, handle_undefined_variables, handlebars_setup, label, render_template,
    traverse_directory, write_to_file,
};
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_TEMPLATE_NAME: &str = "default";
const CUSTOM_TEMPLATE_NAME: &str = "custom";

const TEMPLATES: &[(&str, &str, &str)] = &[
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
        "write-github-pull-request",
        include_str!("../templates/write-github-pull-request.hbs"),
        "撰寫 GitHub Pull Request",
    ),
    (
        "write-git-commit",
        include_str!("../templates/write-git-commit.hbs"),
        "撰寫 Git Commit",
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
        "write-github-readme",
        include_str!("../templates/write-github-readme.hbs"),
        "撰寫 GitHub README",
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
#[clap(name = "c2p", version = "2.0.0", author = "Mufeed VH")]
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

    /// Display the token count of the generated prompt
    #[clap(long)]
    tokens: bool,

    /// Optional tokenizer to use for token count
    ///
    /// Supported tokenizers: cl100k (default), p50k, p50k_edit, r50k, gpt2
    #[clap(short = 'c', long)]
    encoding: Option<String>,

    /// Optional output file path
    #[clap(short, long)]
    output: Option<String>,

    /// Include git diff
    #[clap(short, long)]
    diff: bool,

    /// Generate git diff between two branches
    #[clap(long, value_name = "BRANCHES")]
    git_diff_branch: Option<String>,

    /// Retrieve git log between two branches
    #[clap(long, value_name = "BRANCHES")]
    git_log_branch: Option<String>,

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

fn show_available_templates() {
    println!("可用模板:");
    for (template_name, _, description) in TEMPLATES.iter() {
        println!("  - {} ({})", template_name, description);
    }
}

fn setup_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(120));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"])
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner
}

fn parse_patterns(patterns: &Option<String>) -> Vec<String> {
    patterns
        .as_ref()
        .filter(|p| !p.is_empty())
        .map(|p| p.split(',').map(str::trim).map(String::from).collect())
        .unwrap_or_default()
}

fn get_git_diff_branch(args: &Cli, spinner: &ProgressBar) -> Result<String> {
    if let Some(branches) = &args.git_diff_branch {
        spinner.set_message("生成兩個分支之間的 git diff...");
        let branches = parse_patterns(&Some(branches.to_string()));
        if branches.len() != 2 {
            return Err(anyhow::anyhow!("請提供兩個分支，以逗號分隔。"));
        }
        Ok(
            get_git_diff_between_branches(&args.path, &branches[0], &branches[1])
                .unwrap_or_default(),
        )
    } else {
        Ok(String::new())
    }
}

fn get_git_log_branch(args: &Cli, spinner: &ProgressBar) -> Result<String> {
    if let Some(branches) = &args.git_log_branch {
        spinner.set_message("生成兩個分支之間的 git log...");
        let branches = parse_patterns(&Some(branches.to_string()));
        if branches.len() != 2 {
            return Err(anyhow::anyhow!("請提供兩個分支，以逗號分隔。"));
        }
        Ok(get_git_log(&args.path, &branches[0], &branches[1]).unwrap_or_default())
    } else {
        Ok(String::new())
    }
}

fn print_json_output(
    rendered: &str,
    path: &PathBuf,
    token_count: usize,
    model_info: &str,
    paths: &[String],
) -> Result<()> {
    let json_output = json!({
        "prompt": rendered,
        "directory_name": label(path),
        "token_count": token_count,
        "model_info": model_info,
        "files": paths,
    });
    println!("{}", serde_json::to_string_pretty(&json_output)?);
    Ok(())
}

fn print_normal_output(token_count: usize, model_info: &str, args: &Cli) {
    if args.tokens {
        println!(
            "{}{}{} Token 數量: {}, 模型資訊: {}",
            "[".bold().white(),
            "i".bold().blue(),
            "]".bold().white(),
            token_count.to_string().bold().yellow(),
            model_info
        );
    }
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

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

    let (template_content, template_name) = if let Some(hbs_path) = &args.hbs {
        // 使用自定義模板文件
        get_custom_template(Path::new(hbs_path))?
    } else if let Some(template_option) = &args.template {
        match template_option {
            Some(template_name) => {
                // 使用預定義模板
                get_predefined_template(template_name)?
            }
            None => {
                // 當 -t 參數存在但沒有提供值時，顯示可用的模板並退出
                show_available_templates();
                std::process::exit(0);
            }
        }
    } else {
        // 使用默認模板
        (
            include_str!("default_template.hbs").to_string(),
            DEFAULT_TEMPLATE_NAME.to_string(),
        )
    };

    let handlebars = handlebars_setup(&template_content, &template_name)?;

    let spinner = setup_spinner("遍歷目錄並構建樹...");

    let include_patterns = parse_patterns(&args.include);
    let exclude_patterns = parse_patterns(&args.exclude);

    let (tree, files) = traverse_directory(
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
        spinner.finish_with_message("失敗!".red().to_string());
        anyhow::anyhow!("無法構建目錄樹: {}", e)
    })?;

    let git_diff = if args.diff {
        spinner.set_message("生成 git diff...");
        match get_git_diff(&args.path) {
            Ok(diff) => {
                if diff.is_empty() {
                    println!("沒有檢測到未暫存的更改。");
                } else {
                    println!("成功獲取 git diff 的內容。");
                }
                diff
            }
            Err(e) => {
                eprintln!("獲取 git diff 時出錯: {}", e);
                String::new()
            }
        }
    } else {
        String::new()
    };

    let git_diff_branch = get_git_diff_branch(&args, &spinner)?;
    let git_log_branch = get_git_log_branch(&args, &spinner)?;

    spinner.finish_with_message("完成!".green().to_string());

    let mut data = json!({
        "absolute_code_path": label(&args.path),
        "source_tree": tree,
        "files": files,
        "git_diff": git_diff,
        "git_diff_branch": git_diff_branch,
        "git_log_branch": git_log_branch
    });

    debug!("JSON 數據: {}", serde_json::to_string_pretty(&data)?);

    handle_undefined_variables(&mut data, &template_content)?;

    let mut rendered = render_template(&handlebars, &template_name, &data)?;

    if let Some(lang) = &args.lang {
        rendered.push_str(&format!("\nYou must use {} language to reply", lang));
    }

    let token_count = if args.tokens {
        let bpe = get_tokenizer(&args.encoding);
        bpe.encode_with_special_tokens(&rendered).len()
    } else {
        0
    };

    let paths: Vec<String> = files
        .iter()
        .filter_map(|file| file.get("path").and_then(|p| p.as_str()).map(String::from))
        .collect();

    let model_info = get_model_info(&args.encoding);

    if args.json {
        print_json_output(&rendered, &args.path, token_count, &model_info, &paths)?;
    } else {
        print_normal_output(token_count, &model_info, &args);
    }

    if !args.no_clipboard {
        copy_to_clipboard_with_feedback(&rendered);
    }

    if let Some(output_path) = &args.output {
        write_to_file(output_path, &rendered)?;
    }

    Ok(())
}

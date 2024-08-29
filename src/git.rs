//! This module handles git operations.
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use git2::{Commit, Diff, DiffOptions, Repository};
use log::info;
use std::path::Path;

/// Generates a git diff for the repository at the provided path
///
/// # Arguments
///
/// * `repo_path` - A reference to the path of the git repository
///
/// # Returns
///
/// * `Result<String, git2::Error>` - The generated git diff as a string or an error
pub fn get_git_diff(repo_path: &Path) -> Result<String> {
    info!("正在打開倉庫,路徑:{:?}", repo_path);
    let repo = Repository::open(repo_path).context("無法打開倉庫")?;

    let head_commit = repo.head()?.peel_to_commit()?;
    let head_tree = head_commit.tree()?;

    let mut index = repo.index()?;
    index.update_all(["*"].iter(), None)?;
    let work_tree_oid = index.write_tree()?;
    let work_tree = repo.find_tree(work_tree_oid)?;

    let mut diff_opts = DiffOptions::new();
    diff_opts.ignore_whitespace(true);

    let diff = repo
        .diff_tree_to_tree(Some(&head_tree), Some(&work_tree), Some(&mut diff_opts))
        .context("Failed to generate diff")?;

    let filtered_diff = filter_diff(&diff)?;

    Ok(filtered_diff)
}

fn filter_diff(diff: &Diff) -> Result<String> {
    let mut diff_text = String::new();

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        let file_name = delta
            .new_file()
            .path()
            .and_then(|p| p.file_name())
            .and_then(|f| f.to_str())
            .unwrap_or("");
        let is_ignored_file = file_name.eq_ignore_ascii_case("readme.md")
            || file_name.eq_ignore_ascii_case("changelog.md");

        if line.origin() == 'F' {
            // 總是顯示文件標頭
            let new_file = delta.new_file();
            let old_file = delta.old_file();
            if let (Some(old_path), Some(new_path)) = (old_file.path(), new_file.path()) {
                diff_text.push_str(&format!(
                    "diff --git a/{} b/{}\n",
                    old_path.display(),
                    new_path.display()
                ));
            }
        } else if !is_ignored_file
            && (line.origin() == '+' || line.origin() == '-' || line.origin() == ' ')
        {
            // 只包含非忽略文件的修改行
            let content = std::str::from_utf8(line.content()).unwrap_or("無法解碼的內容");
            diff_text.push(line.origin());
            diff_text.push_str(content);
        }
        true
    })
    .context("Failed to print diff")?;

    Ok(diff_text)
}

/// Generates a git diff between two branches for the repository at the provided path
///
/// # Arguments
///
/// * `repo_path` - A reference to the path of the git repository
/// * `branch1` - The name of the first branch
/// * `branch2` - The name of the second branch
///
/// # Returns
///
/// * `Result<String, git2::Error>` - The generated git diff as a string or an error
pub fn get_git_diff_between_branches(
    repo_path: &Path,
    branch1: &str,
    branch2: &str,
) -> Result<String> {
    info!("正在打開倉庫,路徑:{:?}", repo_path);
    let repo = Repository::open(repo_path).context("無法打開倉庫")?;

    for branch in [branch1, branch2].iter() {
        if !branch_exists(&repo, branch) {
            return Err(anyhow::anyhow!("分支 {} 不存在！", branch));
        }
    }

    let branch1_commit = repo.revparse_single(branch1)?.peel_to_commit()?;
    let branch2_commit = repo.revparse_single(branch2)?.peel_to_commit()?;

    let branch1_tree = branch1_commit.tree()?;
    let branch2_tree = branch2_commit.tree()?;

    let diff = repo
        .diff_tree_to_tree(
            Some(&branch1_tree),
            Some(&branch2_tree),
            Some(DiffOptions::new().ignore_whitespace(true)),
        )
        .context("Failed to generate diff between branches")?;

    let mut diff_text = Vec::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        diff_text.extend_from_slice(line.content());
        true
    })
    .context("Failed to print diff")?;

    info!("成功生成分支之間的 git diff。");
    Ok(String::from_utf8_lossy(&diff_text).into_owned())
}

/// Retrieves the git log between two branches for the repository at the provided path
///
/// # Arguments
///
/// * `repo_path` - A reference to the path of the git repository
/// * `branch1` - The name of the first branch (e.g., "master")
/// * `branch2` - The name of the second branch (e.g., "migrate-manifest-v3")
///
/// # Returns
///
/// * `Result<String, git2::Error>` - The git log as a string or an error
///
/// Checks if a local branch exists in the given repository
///
/// # Arguments
///
/// * `repo` - A reference to the `Repository` where the branch should be checked
/// * `branch_name` - A string slice that holds the name of the branch to check
///
/// # Returns
///
/// * `bool` - `true` if the branch exists, `false` otherwise
fn branch_exists(repo: &Repository, branch_name: &str) -> bool {
    repo.find_branch(branch_name, git2::BranchType::Local)
        .is_ok()
}

pub fn get_git_log_by_date_range(repo_path: &Path, date_range: &str) -> Result<String> {
    info!("正在打開倉庫,路徑: {:?}", repo_path);
    let repo = Repository::open(repo_path).context("無法打開倉庫")?;

    let dates: Vec<&str> = date_range.split("..").collect();
    if dates.len() != 2 {
        return Err(anyhow::anyhow!(
            "無效的日期範圍格式,應為 'YYYY-MM-DD..YYYY-MM-DD'"
        ));
    }

    let start_date = NaiveDate::parse_from_str(dates[0], "%Y-%m-%d").context("無法解析開始日期")?;
    let end_date = NaiveDate::parse_from_str(dates[1], "%Y-%m-%d").context("無法解析結束日期")?;

    let mut revwalk = repo.revwalk().context("無法創建 revwalk")?;
    revwalk.push_head().context("無法推送 HEAD 到 revwalk")?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut log_text = String::new();
    for oid in revwalk {
        let oid = oid.context("無法從 revwalk 獲取 OID")?;
        let commit = repo.find_commit(oid).context("無法找到提交")?;
        let commit_time = commit.time().seconds();
        let commit_date = DateTime::<Utc>::from_timestamp(commit_time, 0)
            .map(|dt| dt.naive_utc().date())
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        if commit_date >= start_date && commit_date <= end_date {
            log_text.push_str(&format_commit_with_diff(&repo, &commit)?);
        } else if commit_date < start_date {
            break;
        }
    }

    info!("成功獲取 git log");
    Ok(log_text)
}

fn format_commit_with_diff(repo: &Repository, commit: &Commit) -> Result<String> {
    let mut output = String::new();

    // Add commit information
    output.push_str(&format!("commit {}\n", commit.id()));
    output.push_str(&format!("Author: {}\n", commit.author()));
    output.push_str(&format!(
        "Date:   {}\n\n",
        DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %z")
    ));
    output.push_str(&format!(
        "    {}\n\n",
        commit.message().unwrap_or("無提交信息")
    ));

    // Get changes
    let parent = if commit.parent_count() > 0 {
        Some(commit.parent(0)?)
    } else {
        None
    };

    let parent_tree = parent.as_ref().and_then(|c| c.tree().ok());
    let commit_tree = commit.tree()?;

    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        parent_tree.as_ref(),
        Some(&commit_tree),
        Some(&mut diff_opts),
    )?;

    // Convert diff to text
    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        let file_name = delta
            .new_file()
            .path()
            .and_then(|p| p.file_name())
            .and_then(|f| f.to_str())
            .unwrap_or("");
        let is_ignored_file = file_name.eq_ignore_ascii_case("readme.md")
            || file_name.eq_ignore_ascii_case("changelog.md");

        if line.origin() == 'F' {
            // Always show file header
            let new_file = delta.new_file();
            let old_file = delta.old_file();
            if let (Some(old_path), Some(new_path)) = (old_file.path(), new_file.path()) {
                output.push_str(&format!(
                    "diff --git a/{} b/{}\n",
                    old_path.display(),
                    new_path.display()
                ));
            }
        } else if !is_ignored_file
            && (line.origin() == '+' || line.origin() == '-' || line.origin() == ' ')
        {
            // Only include non-ignored file changes
            let content = std::str::from_utf8(line.content()).unwrap_or("無法解碼的內容");
            output.push(line.origin());
            output.push_str(content);
        }
        true
    })?;

    output.push('\n');
    Ok(output)
}

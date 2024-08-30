use log::{debug, error, warn};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Determines whether a file should be included based on include and exclude patterns.
///
/// # Arguments
///
/// * `path` - The path to the file to be checked.
/// * `include_patterns` - A slice of strings representing the include patterns.
/// * `exclude_patterns` - A slice of strings representing the exclude patterns.
/// * `include_priority` - A boolean indicating whether to give priority to include patterns if both include and exclude patterns match.
///
/// # Returns
///
/// * `bool` - `true` if the file should be included, `false` otherwise.
pub fn should_include_file(
    path: &Path,
    include_patterns: &[String],
    exclude_patterns: &[String],
    include_priority: bool,
) -> bool {
    let path_str = match fs::canonicalize(path) {
        Ok(canonical_path) => canonical_path.to_string_lossy().into_owned(),
        Err(e) => {
            warn!("無法正規化路徑: {}, 使用原始路徑", e);
            path.to_string_lossy().into_owned()
        }
    };

    let included = if include_patterns.is_empty() {
        true // 如果沒有包含模式，默認包含所有文件
    } else {
        include_patterns.iter().any(|pattern| {
            let matches = matches_pattern(&path_str, pattern);
            debug!(
                "Include pattern '{}' matches path '{}': {}",
                pattern, path_str, matches
            );
            matches
        })
    };

    let excluded = exclude_patterns.iter().any(|pattern| {
        let matches = matches_pattern(&path_str, pattern);
        debug!(
            "Exclude pattern '{}' matches path '{}': {}",
            pattern, path_str, matches
        );
        matches
    });

    debug!(
        "Path: {}, Included: {}, Excluded: {}, Include Priority: {}",
        path_str, included, excluded, include_priority
    );

    match (included, excluded) {
        (true, true) => include_priority,
        (true, false) => true,
        (false, true) => false,
        (false, false) => include_patterns.is_empty(),
    }
}

fn matches_pattern(path: &str, pattern: &str) -> bool {
    let regex_pattern = convert_wildcard_to_regex(pattern);
    Regex::new(&regex_pattern)
        .map(|re| re.is_match(path))
        .unwrap_or_else(|e| {
            error!("無效的正則表達式 '{}': {}", regex_pattern, e);
            false
        })
}

fn convert_wildcard_to_regex(pattern: &str) -> String {
    pattern
        .replace(".", r"\.")
        .replace("*", ".*")
        .replace("?", ".")
}

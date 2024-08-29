//! This module contains the logic for filtering files based on include and exclude patterns.

use log::error;
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
    let canonical_path = match fs::canonicalize(path) {
        Ok(path) => path,
        Err(e) => {
            error!("無法正規化路徑: {}", e);
            return false;
        }
    };
    let path_str = canonical_path.to_str().unwrap_or("");

    let included = include_patterns.is_empty() || include_patterns
        .iter()
        .any(|pattern| matches_regex(path_str, pattern));
    let excluded = exclude_patterns
        .iter()
        .any(|pattern| matches_regex(path_str, pattern));

    match (included, excluded) {
        (true, true) => include_priority,
        (true, false) => true,
        (false, true) => false,
        (false, false) => include_patterns.is_empty(),
    }
}

fn matches_regex(path: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .map(|re| re.is_match(path))
        .unwrap_or_else(|e| {
            error!("無效的正則表達式 '{}': {}", pattern, e);
            false
        })
}

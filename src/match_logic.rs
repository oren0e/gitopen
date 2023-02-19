use anyhow::anyhow;
use anyhow::Result as AnyhowResult;
use regex::Regex;
use std::process::{Command, Stdio};

#[derive(Debug, PartialEq)]
pub struct FileAtLine<'a> {
    pub filepath: &'a str,
    pub line_number: &'a str,
}

impl<'a> FileAtLine<'a> {
    pub fn new(filepath: &'a str, line_number: &'a str) -> Self {
        Self {
            filepath,
            line_number,
        }
    }
}

fn is_https(s: &str) -> bool {
    if s.starts_with("http") {
        return true;
    }
    false
}

fn remove_git_suffix(s: &str) -> &str {
    match s.strip_suffix(".git") {
        Some(value) => value,
        None => s,
    }
}

pub fn parse_url_from_git(s: &str) -> AnyhowResult<String> {
    let re = Regex::new(r"((git|ssh|http(s)?)|(git@[\w\.-]+))(:(//)?)([\w\.@:/\-~]+)(\.git)?(/)?")?;
    let url_parts = re
        .captures(s)
        .ok_or_else(|| anyhow!("Git repository not found"))?;
    if is_https(&url_parts[1]) {
        return Ok(s.trim().to_string());
    }
    let domain_re = Regex::new(r".*@(.*)$")?;
    let match_domain = domain_re
        .captures(&url_parts[1])
        .ok_or_else(|| anyhow!("Regex error capturing ssh domain"))?;

    let result: String = "https://".to_string()
        + &match_domain[1].to_string()
        + r"/"
        + remove_git_suffix(&url_parts[7]);
    Ok(result)
}

pub fn get_commit_link(repo_url: String, commit_sha: &str) -> String {
    repo_url + "/commit/" + commit_sha
}

pub fn parse_path_and_line_arg(arg: &str, split_char: char) -> AnyhowResult<FileAtLine<'_>> {
    if arg.contains(split_char) {
        let mut iterator = arg.split(split_char);
        let file_at_line = FileAtLine::new(
            iterator.next().ok_or_else(|| {
                anyhow!(format!(
                    "Error parsing input. Format is <path-to-file>{}<line-number>",
                    split_char
                ))
            })?,
            iterator.next().ok_or_else(|| {
                anyhow!(format!(
                    "Error parsing input. Format is <path-to-file>{}<line-number>",
                    split_char
                ))
            })?,
        );
        return Ok(file_at_line);
    }
    Err(anyhow!(format!(
        "Split character not found! Format is <path-to-file>{}<line-number>",
        split_char
    )))
}

fn get_current_branch_name() -> AnyhowResult<String> {
    let git_branch = Command::new("git")
        .args(&["symbolic-ref", "--short", "HEAD"])
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(git_branch.stdout)?.trim_end().to_string();

    Ok(stdout)
}

pub fn get_line_number_link(repo_url: &str, path: &str, line_number: &str) -> AnyhowResult<String> {
    let current_branch = get_current_branch_name()?;
    Ok(format!(
        "{}/blob/{}/{}#L{}",
        repo_url, current_branch, path, line_number,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_parsing() {
        let git_repo = "git@github.com:dtolnay/anyhow.git";
        let result_url = parse_url_from_git(git_repo).unwrap();
        assert_eq!(result_url, "https://github.com/dtolnay/anyhow");
    }

    #[test]
    fn test_github_https_parsing() {
        let git_repo = "https://github.com/oren0e/gitopen.git";
        let result_url = parse_url_from_git(git_repo).unwrap();
        assert_eq!(result_url, "https://github.com/oren0e/gitopen.git");
    }

    #[test]
    fn test_gitlab_parsing() {
        let git_repo = "git@git.foo.com:project/repo.git";
        let result_url = parse_url_from_git(git_repo).unwrap();
        assert_eq!(result_url, "https://git.foo.com/project/repo");
    }

    #[test]
    fn test_get_commit_link() {
        let git_repo = "git@git.foo.com:project/repo.git";
        let commit_sha = "998a1b33f600914";
        let git_url = parse_url_from_git(git_repo).unwrap();
        let commit_link = get_commit_link(git_url, commit_sha);
        assert_eq!(
            commit_link,
            "https://git.foo.com/project/repo/commit/998a1b33f600914"
        );
    }

    #[test]
    fn test_parse_path_and_line_arg_success() {
        let happy_case = "my-proj/src/var/main.rs:90";

        let happy_result = parse_path_and_line_arg(happy_case, ':').unwrap();
        assert_eq!(happy_result.filepath, "my-proj/src/var/main.rs");
        assert_eq!(happy_result.line_number, "90");
    }

    #[test]
    #[should_panic(expected = "Split character not found! Format is <path-to-file>:<line-number>")]
    fn test_parse_path_and_line_arg_failure() {
        let sad_case = "my-proj/src/var/main.rs90";
        let _sad_result = parse_path_and_line_arg(sad_case, ':').unwrap();
    }

    #[test]
    fn test_no_git_suffix() {
        let git_repo = "git@git.company.com:project/repo_name";
        let result_url = parse_url_from_git(git_repo).unwrap();
        assert_eq!(result_url, "https://git.company.com/project/repo_name");
    }

    #[test]
    fn test_dash_in_org_name() {
        let git_repo = "git@git.food-supplier.com:project/repo_name";
        let result_url = parse_url_from_git(git_repo).unwrap();
        assert_eq!(
            result_url,
            "https://git.food-supplier.com/project/repo_name"
        );
    }
}

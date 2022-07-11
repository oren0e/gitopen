use anyhow::anyhow;
use anyhow::Result as AnyhowResult;
use regex::Regex;

fn is_https(s: &str) -> bool {
    if s.starts_with("http") {
        return true;
    }
    false
}

pub fn parse_url_from_git(s: &str) -> AnyhowResult<String> {
    let re = Regex::new(r"((git|ssh|http(s)?)|(git@[\w\.]+))(:(//)?)([\w\.@:/\-~]+)(\.git)(/)?")?;
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

    let result: String =
        "https://".to_string() + &match_domain[1].to_string() + r"/" + &url_parts[7].to_string();

    Ok(result)
}

pub fn get_commit_link(repo_url: String, commit_sha: &str) -> String {
    repo_url + "/commit/" + commit_sha
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
}

use anyhow::anyhow;
use anyhow::Result as AnyhowResult;
use regex::Regex;

pub fn parse_url_from_git(s: String) -> AnyhowResult<String> {
    let re = Regex::new(r"((git|ssh|http(s)?)|(git@[\w\.]+))(:(//)?)([\w\.@:/\-~]+)(\.git)(/)?")?;
    let url_parts = re
        .captures(&s)
        .ok_or_else(|| anyhow!("Git repository not found"))?;
    let domain_re = Regex::new(r".*@(.*)$")?;
    let match_domain = domain_re
        .captures(&url_parts[1])
        .ok_or_else(|| anyhow!("Regex error capturing domain"))?;

    let result: String =
        "https://".to_string() + &match_domain[1].to_string() + r"/" + &url_parts[7].to_string();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_parsing() {
        let git_repo = "git@github.com:dtolnay/anyhow.git".to_string();
        let result_url = parse_url_from_git(git_repo).unwrap();
        assert_eq!(result_url, "https://github.com/dtolnay/anyhow");
    }

    #[test]
    fn test_gitlab_parsing() {
        let git_repo = "git@git.foo.com:project/repo.git".to_string();
        let result_url = parse_url_from_git(git_repo).unwrap();
        assert_eq!(result_url, "https://git.foo.com/project/repo");
    }
}

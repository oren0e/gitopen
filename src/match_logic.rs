use anyhow::anyhow;
use anyhow::Result as AnyhowResult;
use regex::Regex;

pub fn parse_url_from_git(s: String) -> AnyhowResult<String> {
    let re = Regex::new(r"((git|ssh|http(s)?)|(git@[\w\.]+))(:(//)?)([\w\.@:/\-~]+)(\.git)(/)?")?;
    let url_parts = re
        .captures(&s)
        .ok_or_else(|| anyhow!("Regex failed to capture groups"))?;
    let domain_re = Regex::new(r".*@(.*)$")?;
    let match_domain = domain_re
        .captures(&url_parts[1])
        .ok_or_else(|| anyhow!("Regex failed to capture groups"))?;

    let result: String =
        "https://".to_string() + &match_domain[1].to_string() + r"/" + &url_parts[7].to_string();

    Ok(result)
}

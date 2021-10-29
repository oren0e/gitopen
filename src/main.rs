use anyhow::Result as AnyhowResult;
use regex::Regex;
use std::process::{Command, Stdio};
use webbrowser;

pub fn parse_url_from_git(git_repo: String) -> AnyhowResult<String> {
    // git@github.com:oren0e/gitopen.git
    let re_start = Regex::new(r"^git@")?;
    let re_end = Regex::new(r"(.git$)|(\n$)")?;
    let re_colon = Regex::new(r":")?;

    let result = re_start.replace_all(&git_repo, "").to_string();
    let result = re_end.replace_all(&result, "").to_string();
    let result = re_colon.replace_all(&result, "/").to_string();
    let result = r"https://".to_string() + &result;

    Ok(result.to_string())
}

fn main() -> AnyhowResult<()> {
    // git config --get remote.origin.url
    let git_repo = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(git_repo.stdout).unwrap();
    let parsed_url = parse_url_from_git(stdout)?;
    println!("{:?}", parsed_url);
    if webbrowser::open(&parsed_url).is_ok() {
        println!("URL opened!")
    }
    Ok(())
}

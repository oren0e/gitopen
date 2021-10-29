use crate::match_logic::parse_url_from_git;
use anyhow::Result as AnyhowResult;
use std::process::{Command, Stdio};
use webbrowser;

mod match_logic;

fn main() -> AnyhowResult<()> {
    let git_repo = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(git_repo.stdout).unwrap();
    let parsed_url = parse_url_from_git(stdout)?;
    webbrowser::open(&parsed_url)?;
    Ok(())
}

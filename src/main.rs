use std::process::{Command, Stdio};
use webbrowser;

fn main() {
    // git config --get remote.origin.url
    let mut git_repo = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8(git_repo.stdout).unwrap();
    println!("{:?}", stdout);
    //let git_url = "https://git.infinidat.com/infradev/dispatcherr";

    //if webbrowser::open(git_url).is_ok() {
    //    println!("URL opened!")
    //}
}

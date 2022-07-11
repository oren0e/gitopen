use clap::{crate_version, App, Arg};

use crate::actions::{open_at_line_number, open_commit, open_repo, push_and_open_pr};
use anyhow::anyhow;
use anyhow::Result as AnyhowResult;

mod actions;
mod match_logic;

fn main() -> AnyhowResult<()> {
    let matches = App::new("Gitopen")
        .version(crate_version!())
        .author("Oren Epshtain")
        .about("Utility to open repo from terminal and pull requests after push")
        .arg(
            Arg::with_name("push_and_pr")
                .short("p")
                .long("push-open-pr")
                .help("Pushes to current branch and opens corresponding PR"),
        )
        .arg(
            Arg::with_name("open_commit")
                .short("c")
                .long("commit")
                .value_name("COMMIT")
                .takes_value(true)
                .help("Opens the specified commit"),
        )
        .arg(
            Arg::with_name("open_line_number")
                .short("l")
                .long("path-and-line")
                .value_name("PATH AND LINE")
                .takes_value(true)
                .help("Open the specified filepath at the specified line number"),
        )
        .get_matches();
    if matches.is_present("push_and_pr") {
        push_and_open_pr()?;
        Ok(())
    } else if matches.is_present("open_commit") {
        open_commit(
            matches
                .value_of("open_commit")
                .ok_or_else(|| anyhow!("Must supply a commit SHA"))?,
        )
    } else if matches.is_present("open_line_number") {
        open_at_line_number(
            matches
                .value_of("open_line_number")
                .ok_or_else(|| anyhow!("Please supply '<filepath>:<line-number>'"))?,
        )?;
        Ok(())
    } else {
        open_repo()?;
        Ok(())
    }
}

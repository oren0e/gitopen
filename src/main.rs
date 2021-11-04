extern crate clap;
use clap::{crate_version, App, Arg};

use crate::actions::{open_repo, push_and_open_pr};
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
        .get_matches();
    if matches.is_present("push_and_pr") {
        push_and_open_pr()?;
        Ok(())
    } else {
        open_repo()?;
        Ok(())
    }
}

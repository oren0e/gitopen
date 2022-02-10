![example workflow name](https://github.com/oren0e/gitopen/workflows/CI/badge.svg)

# Overview

A command line utility to open git repository page in the browser from the repository location in terminal.  
Currently tested only on mac (prior to M1 chip).

# Installation

Note: You have to have rust installed with cargo to be able to install this utility.

### From Github (this repository)

1. Clone the repo
2. Run `make install` from the repo's directory

### From crates.io with Cargo

1. Run `cargo install gitopen`  
   (From [The Book](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html): "All binaries installed with `cargo install` are stored in the installation root’s bin folder. If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be `$HOME/.cargo/bin`. Ensure that directory is in your `$PATH` to be able to run programs you’ve installed with `cargo install`.")

# Usage

Basic usages:

- When in git repository in terminal, run `gitopen`.
- After you've opened a branch and you are ready to push and open a PR, run `gitopen -p`. This will push the changes to the current branch and open the PR in the browser. This can be done at any stage during the work on the PR, not just when you're making the first push.
- You can open a specific commit. Use `gitopen -c COMMIT` where COMMIT is the commit SHA.

For help, use `gitopen --help`

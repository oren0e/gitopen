![example workflow name](https://github.com/oren0e/gitopen/workflows/CI/badge.svg)

# Overview

A command line utility to open git repository page in the browser from the repository location in terminal.  
Currently tested only on mac (prior to M1 chip).

# Installation

Note: You have to have rust installed with cargo to be able to install this utility.

1. Clone the repo
2. Run `make install` from the repo's directory

# Usage

There are 2 basic usages:

- When in git repository in terminal, run `gitopen`.
- After you've opened a branch and you are ready to push and open a PR, run `gitopen -p`. This will push the changes to the current branch and open the PR in the browser.

For help, use `gitopen --help`

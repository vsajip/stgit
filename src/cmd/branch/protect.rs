// SPDX-License-Identifier: GPL-2.0-only

//! `stg branch --protect` implementation.

use anyhow::Result;

use crate::{
    stack::{InitializationPolicy, Stack},
    wrap::PartialRefName,
};

pub(super) fn command() -> clap::Command {
    clap::Command::new("--protect")
        .short_flag('p')
        .override_usage("stg branch {--protect,-p} [branch]")
        .about("Prevent StGit from modifying a branch")
        .arg(
            clap::Arg::new("branch")
                .help("Branch to protect")
                .value_name("branch")
                .value_parser(clap::value_parser!(PartialRefName)),
        )
}

pub(super) fn dispatch(repo: &gix::Repository, matches: &clap::ArgMatches) -> Result<()> {
    let stack = Stack::from_branch(
        repo,
        matches.get_one::<PartialRefName>("branch"),
        InitializationPolicy::RequireInitialized,
    )?;
    stack.set_protected(true)
}

// SPDX-License-Identifier: GPL-2.0-only

//! `stg branch --unprotect` implementation.

use anyhow::Result;

use crate::{
    stack::{InitializationPolicy, Stack},
    wrap::PartialRefName,
};

pub(super) fn command() -> clap::Command {
    clap::Command::new("--unprotect")
        .short_flag('u')
        .override_usage("stg branch {--unprotect,-u} [branch]")
        .about("Allow StGit to modify a previously protected branch")
        .arg(
            clap::Arg::new("branch")
                .help("Branch to unprotect")
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
    stack.set_protected(false)
}

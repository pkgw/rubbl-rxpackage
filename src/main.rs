// Copyright 2017-2022 Peter Williams <peter@newton.cx> and collaborators
// Licensed under the MIT License.

//! The "rubbl rxpackage" command
//!
//! This provides swiss-army-knife access to various tasks that my reduction
//! scripts need that are implemented in the Rubbl framework.

use clap::{command, ArgMatches, Command};
use rubbl_core::{
    anyhow::Result,
    notify::{ClapNotificationArgsExt, NotificationBackend},
    rn_warning,
};
use std::process;

// Define this before the submodules are parsed.
macro_rules! err_msg {
    ($( $fmt_args:expr ),*) => {
        Err($crate::MiscellaneousError(format!($( $fmt_args ),*)).into())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct MiscellaneousError(String);

impl From<MiscellaneousError> for rubbl_casatables::TableError {
    fn from(e: MiscellaneousError) -> Self {
        rubbl_casatables::TableError::UserMessage(e.0)
    }
}

mod flagts;
mod peel;
mod spwglue;

fn main() {
    let matches = make_command().get_matches();

    process::exit(rubbl_core::notify::run_with_notifications(
        matches,
        |matches, nbe| -> Result<i32> {
            match matches.subcommand() {
                Some(("flagts", m)) => flagts::do_cli(m, nbe),
                Some(("peel", m)) => peel::do_cli(m, nbe),
                Some(("show", m)) => do_show_cli(m, nbe),
                Some(("spwglue", m)) => spwglue::do_cli(m, nbe),
                Some((unknown, _)) => {
                    return err_msg!("unrecognized sub-command \"{}\"", unknown);
                }
                None => {
                    // No sub-command provided.
                    make_command().print_long_help()?;
                    println!(); // print_long_help seems to not add a final newline?
                    Ok(0)
                }
            }
        },
    ));
}

/// It seems that the best way to re-print the help in the "help" subcommand
/// is to be able to make multiple App objects.
fn make_command() -> Command {
    command!()
        .rubbl_notify_args()
        .subcommand(flagts::make_command())
        .subcommand(peel::make_command())
        .subcommand(spwglue::make_command())
        .subcommand(make_show_command())
}

pub fn make_show_command() -> Command {
    Command::new("show")
        .bin_name("rubbl rxpackage show")
        .about("Show various pieces of ancillary information")
        .subcommand(
            Command::new("concept-doi")
                .bin_name("rubbl rxpackage show concept-doi")
                .about("Show the concept DOI of rubbl-rxpackage"),
        )
        .subcommand(
            Command::new("version-doi")
                .bin_name("rubbl rxpackage show version-doi")
                .about("Show the version DOI of rubbl-rxpackage"),
        )
}

pub fn do_show_cli(matches: &ArgMatches, nbe: &mut dyn NotificationBackend) -> Result<i32> {
    match matches.subcommand() {
        Some(("concept-doi", _)) => {
            // For releases, this will be rewritten to the real concept DOI:
            let doi = "10.5281/zenodo.3403263";

            if doi.starts_with("xx.") {
                rn_warning!(
                    nbe,
                    "you are running a development build; the printed value is not a real DOI"
                );
            }

            println!("{}", doi);
            Ok(0)
        }

        Some(("version-doi", _)) => {
            // For releases, this will be rewritten to the real version DOI:
            let doi = "10.5281/zenodo.15277584";

            if doi.starts_with("xx.") {
                rn_warning!(
                    nbe,
                    "you are running a development build; the printed value is not a real DOI"
                );
            }

            println!("{}", doi);
            Ok(0)
        }

        Some((unknown, _)) => {
            return err_msg!("unrecognized sub-command \"{}\"", unknown);
        }

        None => {
            // No sub-command provided.
            make_show_command().print_long_help()?;
            println!(); // print_long_help seems to not add a final newline?
            Ok(0)
        }
    }
}

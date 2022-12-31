// Copyright 2017-2022 Peter Williams <peter@newton.cx> and collaborators
// Licensed under the MIT License.

//! The "rubbl rxpackage" command
//!
//! This provides swiss-army-knife access to various tasks that my reduction
//! scripts need that are implemented in the Rubbl framework.

use clap::{crate_version, App, AppSettings, ArgMatches, SubCommand};
use failure::Fail;
use rubbl_core::{
    notify::{ClapNotificationArgsExt, NotificationBackend},
    rn_warning, Result,
};
use std::process;

// Define this before the submodules are parsed.
macro_rules! err_msg {
    ($( $fmt_args:expr ),*) => {
        Err($crate::MiscellaneousError(format!($( $fmt_args ),*)).into())
    }
}

#[derive(Fail, Debug)]
#[fail(display = "{}", _0)]
pub struct MiscellaneousError(String);

mod flagts;
mod peel;
mod spwglue;

fn main() {
    let matches = make_app().get_matches();

    process::exit(rubbl_core::notify::run_with_notifications(
        matches,
        |matches, nbe| -> Result<i32> {
            match matches.subcommand() {
                ("flagts", Some(m)) => flagts::do_cli(m, nbe),
                ("peel", Some(m)) => peel::do_cli(m, nbe),
                ("show", Some(m)) => do_show_cli(m, nbe),
                ("spwglue", Some(m)) => spwglue::do_cli(m, nbe),
                (unknown, Some(_)) => {
                    return err_msg!("unrecognized sub-command \"{}\"", unknown);
                }
                (_, None) => {
                    // No sub-command provided.
                    make_app().print_long_help()?;
                    println!(); // print_long_help seems to not add a final newline?
                    Ok(0)
                }
            }
        },
    ));
}

/// It seems that the best way to re-print the help in the "help" subcommand
/// is to be able to make multiple App objects.
fn make_app<'a, 'b>() -> App<'a, 'b> {
    App::new("rubbl-rxpackage")
        .version(crate_version!())
        .bin_name("rubbl rxpackage")
        .setting(AppSettings::DisableHelpSubcommand)
        .rubbl_notify_args()
        .subcommand(flagts::make_app())
        .subcommand(peel::make_app())
        .subcommand(spwglue::make_app())
        .subcommand(make_show_app())
}

pub fn make_show_app<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .bin_name("rubbl rxpackage show")
        .about("Show various pieces of ancillary information")
        .subcommand(
            SubCommand::with_name("concept-doi")
                .bin_name("rubbl rxpackage show concept-doi")
                .about("Show the concept DOI of rubbl-rxpackage"),
        )
        .subcommand(
            SubCommand::with_name("version-doi")
                .bin_name("rubbl rxpackage show version-doi")
                .about("Show the version DOI of rubbl-rxpackage"),
        )
}

pub fn do_show_cli(matches: &ArgMatches, nbe: &mut dyn NotificationBackend) -> Result<i32> {
    match matches.subcommand() {
        ("concept-doi", Some(_)) => {
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

        ("version-doi", Some(_)) => {
            // For releases, this will be rewritten to the real version DOI:
            let doi = "10.5281/zenodo.7497313";

            if doi.starts_with("xx.") {
                rn_warning!(
                    nbe,
                    "you are running a development build; the printed value is not a real DOI"
                );
            }

            println!("{}", doi);
            Ok(0)
        }

        (unknown, Some(_)) => {
            return err_msg!("unrecognized sub-command \"{}\"", unknown);
        }

        (_, None) => {
            // No sub-command provided.
            make_show_app().print_long_help()?;
            println!(); // print_long_help seems to not add a final newline?
            Ok(0)
        }
    }
}

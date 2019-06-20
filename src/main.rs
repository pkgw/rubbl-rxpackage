// Copyright 2017-2019 Peter Williams <peter@newton.cx> and collaborators
// Licensed under the MIT License.

/*! The "rubbl rxpackage" command

This provides swiss-army-knife access to various tasks that my reduction
scripts need that are implemented in the Rubbl framework.

*/

extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate failure;
extern crate failure_derive;
extern crate itertools;
#[macro_use]
extern crate ndarray;
extern crate nom;
extern crate num_traits;
extern crate pbr;
extern crate rubbl_casatables;
#[macro_use]
extern crate rubbl_core;

use clap::{App, AppSettings};
use rubbl_core::notify::ClapNotificationArgsExt;
use rubbl_core::Result;
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
}

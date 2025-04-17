// Copyright 2018-2021 Peter Williams <peter@newton.cx> and collaborators
// Licensed under the MIT License.

//! Generate a time series of flagging percentages for a data set.

use clap::{App, Arg, ArgMatches, SubCommand};
use itertools::Itertools;
use ndarray::Ix2;
use pbr;
use rubbl_casatables::{Table, TableOpenMode};
use rubbl_core::{
    anyhow::{self, Result},
    ctry,
    notify::NotificationBackend,
    Array,
};
use std::{
    self,
    collections::HashMap,
    f64, io, mem,
    path::{Path, PathBuf},
};

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("flagts")
        .bin_name("rubbl rxpackage flagts")
        .about("Print a time series of flagging fractions")
        .arg(
            Arg::with_name("IN-TABLE")
                .help("The path of the input data set")
                .required(true)
                .index(1),
        )
}

pub fn do_cli(matches: &ArgMatches, _nbe: &mut dyn NotificationBackend) -> Result<i32> {
    // Deal with args.

    let inpath_os = matches.value_of_os("IN-TABLE").unwrap();
    let inpath = Path::new(inpath_os).to_owned();

    // Open up the input table and do some prep work. We do this up here
    // so that we can validate some of the program configuration before
    // creating the output tables.

    fn open_table(base: &Path, extension: &str, is_input: bool) -> Result<(PathBuf, Table)> {
        let mut p = base.to_owned();

        if extension.len() > 0 {
            p.push(extension);
        }

        let mode = if is_input {
            TableOpenMode::Read
        } else {
            TableOpenMode::ReadWrite
        };

        let t = ctry!(Table::open(&p, mode);
                      "failed to open {} {}table \"{}\"",
                      if is_input { "input" } else { "output" },
                      if extension.len() > 0 { "sub-" } else { "" },
                      p.display()
        );

        Ok((p, t))
    }

    let (_, mut in_main_table) = open_table(&inpath, "", true)?;

    // Let's do it.

    struct TimeslotInfo {
        n_total: usize,
        n_flagged: usize,
    }

    let mut records = HashMap::new();
    let mut in_row_num = 0usize;
    let stderr = io::stderr();
    let mut pb = pbr::ProgressBar::on(stderr.lock(), in_main_table.n_rows());
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));

    in_main_table.for_each_row(|in_row| {
        let time: f64 = in_row.get_cell("TIME")?;
        let recast_time: u64 = unsafe { mem::transmute(time) };

        if !records.contains_key(&recast_time) {
            records.insert(
                recast_time,
                TimeslotInfo {
                    n_total: 0,
                    n_flagged: 0,
                },
            );
        }

        let state = records.get_mut(&recast_time).unwrap();
        let flag = in_row.get_cell::<Array<bool, Ix2>>("FLAG")?;
        state.n_total += flag.len();
        state.n_flagged += flag.fold(0usize, |a, f| if *f { a + 1 } else { a });

        in_row_num += 1;
        pb.inc();
        Ok(())
    })?;

    let iter = records.keys().sorted_by(|rc1, rc2| {
        // explicit type annotations to avoid accidentally transmuting &u64
        let t1: f64 = unsafe { mem::transmute::<u64, f64>(**rc1) };
        let t2: f64 = unsafe { mem::transmute::<u64, f64>(**rc2) };
        t1.partial_cmp(&t2).unwrap()
    });

    let mut t0 = f64::NAN;

    for recast_time in iter {
        let time: f64 = unsafe { mem::transmute::<u64, f64>(*recast_time) };
        let state = records.get(recast_time).unwrap();

        if t0.is_nan() {
            t0 = time;
        }

        println!(
            "{:.16e} {:.16e} {} {}",
            time,
            time - t0,
            state.n_total,
            state.n_flagged
        );
    }

    pb.finish_println(&format!(
        "Computed flag stats for {} timeslots",
        records.len()
    ));
    Ok(0)
}

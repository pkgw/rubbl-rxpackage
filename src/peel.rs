// Copyright 2019 Peter Williams <peter@newton.cx> and collaborators
// Licensed under the MIT License.

//! Peeling!
//!
//! This tool is part of a CASA-based source peeling workflow. The basic
//! outline is this:
//!
//! 1. Some first-cut calibrated data are stored in "main.ms" in the (main)
//!    DATA column.
//! 2. You image the data, yielding a CLEAN component model. There is a source
//!    that you want to peel from the data, referred to as "source A". It is
//!    probably off-axis, since if it were on-axis the standard calibration
//!    would suffice. Let's write main.ms:DATA = A_P + S + N, where A_P is the
//!    signal from source A, perturbed by direction-dependent effects; S is
//!    the signal from all other sources, and N is noise.
//! 3. You create a modified CLEAN component model called "allbutA.tt{0,1}"
//!    (assuming you are using two Taylor terms with MFS). This model is your
//!    first-cut model with source A zeroed out.
//! 4. You use `ft` to fill main.ms:MODEL with the "allbutA" model; so
//!    main.ms:MODEL = S, to the best of our abilities.
//! 5. You use `uvsub` to fill main.ms:CORRECTED with the difference between
//!    the two: main.ms:CORRECTED = A_P + N
//! 6. You use `split` to create work.ms:DATA, equal to main.ms:CORRECTED =
//!    A_P + N.
//! 7. You use fill work.ms:MODEL with an idealized model of source A. This
//!    could be done by using CASA's component-list routines and then the `ft`
//!    task again. Write work.ms:MODEL = A_I, the idealized signal from source
//!    A.
//! 8. You use `gaincal` to solve for calibration gains to correct the
//!    direction-dependent effects associated with source A.
//! 9. Use `applycal` to fill in work.ms:CORRECTED with the best approximation
//!    of work.ms:MODEL that can be obtained through the calibration fit,
//!    which we assume is multiplicative:
//!    work.ms:CORRECTED = G*work.ms:DATA = G(A_P + N) ~= A_I,
//!    where the final equality is in some least-squares-type sense. Therefore
//!    A_P ~= A_I / G ~= work.ms:MODEL * work.ms:DATA / work.ms:CORRECTED.
//! 10. Finally, this tool comes into play. We want to subtract A_P out of
//!     the main.ms data. Therefore, we have to fill in:
//!
//!     main.ms:CORRECTED = main.ms:DATA - work.ms:MODEL * work.ms:DATA / work.ms:CORRECTED
//!
//!     This tool iterates over the two datasets and performs this operation.

use clap::{App, Arg, ArgMatches, SubCommand};
use ndarray::{Ix2, Zip};
use pbr;
use rubbl_casatables::{Table, TableOpenMode};
use rubbl_core::notify::NotificationBackend;
use rubbl_core::{Array, Complex, Result};
use std;
use std::path::Path;

// Let's get this show on the road.

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("peel")
        .bin_name("rubbl rxpackage peel")
        .about("Subtract a modeled source from a dataset")
        .arg(
            Arg::with_name("MAIN-TABLE")
                .help("The path of the data set from which to subtract the source")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("WORK-TABLE")
                .help("The path of the data set containing the source model and calibration gains")
                .required(true)
                .index(2),
        )
}

pub fn do_cli(matches: &ArgMatches, nbe: &mut NotificationBackend) -> Result<i32> {
    let mainpath_os = matches.value_of_os("MAIN-TABLE").unwrap();
    let mainpath = Path::new(mainpath_os).to_owned();

    let workpath_os = matches.value_of_os("WORK-TABLE").unwrap();
    let workpath = Path::new(workpath_os).to_owned();

    // Open up tables and do some checking.

    fn open_table(base: &Path, readwrite: bool) -> Result<Table> {
        let p = base.to_owned();

        let mode = if readwrite {
            TableOpenMode::ReadWrite
        } else {
            TableOpenMode::Read
        };

        Ok(ctry!(Table::open(&p, mode);
                 "failed to open table \"{}\"", p.display()))
    }

    let mut main_table = open_table(&mainpath, true)?;
    let mut work_table = open_table(&workpath, false)?;

    let n_rows = main_table.n_rows();

    if work_table.n_rows() != n_rows {
        rn_fatal!(
            nbe,
            "main table \"{}\" has {} rows, but work \
             table \"{}\" has {} rows",
            mainpath.display(),
            n_rows,
            workpath.display(),
            work_table.n_rows()
        );
        return Ok(1);
    }

    fn check_corrected(table: &mut Table, path: &Path) -> Result<()> {
        let col_names = ctry!(table.column_names();
                              "failed to get names of columns in \"{}\"", path.display());
        let mut seen_corrected = false;

        for n in &col_names {
            if n == "CORRECTED_DATA" {
                seen_corrected = true;
                break;
            }
        }

        if !seen_corrected {
            return err_msg!(
                "CORRECTED_DATA column in table in \"{}\" \
                 is required but missing",
                path.display()
            );
        }

        Ok(())
    }

    check_corrected(&mut main_table, &mainpath)?;
    check_corrected(&mut work_table, &workpath)?;

    // Do the operation.

    let mut main_row = main_table.get_row_writer()?;
    let mut work_row = work_table.get_row_reader()?;

    let mut pb = pbr::ProgressBar::new(n_rows);
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));

    for row in 0..n_rows {
        main_table.read_row(&mut main_row, row)?;
        work_table.read_row(&mut work_row, row)?;

        let main_data: Array<Complex<f32>, Ix2> = main_row.get_cell("DATA")?;
        let main_flag: Array<bool, Ix2> = main_row.get_cell("FLAG")?;
        let work_data: Array<Complex<f32>, Ix2> = work_row.get_cell("DATA")?;
        let work_flag: Array<bool, Ix2> = work_row.get_cell("FLAG")?;
        let work_model: Array<Complex<f32>, Ix2> = work_row.get_cell("MODEL_DATA")?;
        let mut work_corr: Array<Complex<f32>, Ix2> = work_row.get_cell("CORRECTED_DATA")?;

        let mut peel_flag = main_flag | work_flag;

        // Avoid div-by-zero.
        Zip::from(&mut work_corr).and(&mut peel_flag).apply(|c, f| {
            if c.norm() == 0. {
                *f = true;
            }

            if *f {
                *c = Complex::from(1.0);
            }
        });

        let mut peel_data = main_data - work_data * work_model / work_corr;

        // Not strictly necessary, maybe, but I think this is nice.
        Zip::from(&mut peel_data).and(&mut peel_flag).apply(|c, f| {
            if !c.is_finite() {
                *f = true;
            }

            if *f {
                *c = Complex::from(0.0);
            }
        });

        main_table.put_cell("FLAG", row, &peel_flag)?;
        main_table.put_cell("CORRECTED_DATA", row, &peel_data)?;
        pb.inc();
    }

    // All done!

    pb.finish();
    Ok(0)
}

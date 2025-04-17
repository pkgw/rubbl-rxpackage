// Copyright 2019-2021 Peter Williams <peter@newton.cx> and collaborators
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
//!    the two: main.ms:CORRECTED = A_P + N. (**Note:** `uvsub` actually
//!    subtracts MODEL from CORRECTED, so to do this incrementally, you have
//!    to make sure to delete CORRECTED or reset it to DATA first.)
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
//! 10. Finally, this tool comes into play. We'll want to subtract A_P out of
//!     the main.ms data or otherwise model it. We can do this by inserting
//!     the correct DD-perturbed model and then running uvsub:
//!
//!     main.ms:MODEL = work.ms:MODEL * work.ms:DATA / work.ms:CORRECTED
//!
//!     Or we can add to main.ms:MODEL to allow incremental change. This tool
//!     iterates over the two datasets and performs this operation.

use clap::{App, Arg, ArgMatches, SubCommand};
use ndarray::{Ix2, Zip};
use pbr;
use rubbl_casatables::{CasaDataType, Table, TableOpenMode, TableRow};
use rubbl_core::{
    anyhow::{self, Result},
    ctry,
    notify::NotificationBackend,
    rn_fatal, Array, Complex,
};
use std::{
    self,
    path::{Path, PathBuf},
};

// Let's get this show on the road.

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("peel")
        .bin_name("rubbl rxpackage peel")
        .about("Model a source with source-specific calibration gain solutions")
        .arg(
            Arg::with_name("incremental")
                .long("incremental")
                .help("If specified, add the model to main:MODEL_DATA, rather than overwriting"),
        )
        .arg(
            Arg::with_name("MAIN-TABLE")
                .help("The path of the data set into which to insert the source model")
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

pub fn do_cli(matches: &ArgMatches, nbe: &mut dyn NotificationBackend) -> Result<i32> {
    let mainpath_os = matches.value_of_os("MAIN-TABLE").unwrap();
    let mainpath = Path::new(mainpath_os).to_owned();

    let workpath_os = matches.value_of_os("WORK-TABLE").unwrap();
    let workpath = Path::new(workpath_os).to_owned();

    let incremental = matches.is_present("incremental");

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

    fn check_cols(table: &mut Table, path: &Path, wanted_col_names: &[&str]) -> Result<()> {
        let observed_col_names = ctry!(
            table.column_names();
            "failed to get names of columns in \"{}\"", path.display()
        );

        // Not highly efficient, but N is small here ...

        for wanted_col in wanted_col_names {
            let mut seen_col = false;

            for n in &observed_col_names {
                if n == wanted_col {
                    seen_col = true;
                    break;
                }
            }

            if !seen_col {
                return err_msg!(
                    "{} column in table in \"{}\" \
                    is required but missing",
                    wanted_col,
                    path.display()
                );
            }
        }

        Ok(())
    }

    check_cols(&mut main_table, &mainpath, &["FLAG", "MODEL_DATA"])?;
    check_cols(
        &mut work_table,
        &workpath,
        &["FLAG", "DATA", "MODEL_DATA", "CORRECTED_DATA"],
    )?;

    // Do the operation.

    let mut main_row = main_table.get_row_writer()?;
    let mut work_row = work_table.get_row_reader()?;

    let mut pb = pbr::ProgressBar::new(n_rows);
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(500)));

    /// Helper to provide error context if a `get_cell` call fails.
    ///
    /// This could almost work as a closure, but the type parametricity would be
    /// a hassle.
    #[inline(always)]
    fn getcell_context<T: CasaDataType>(
        row: &mut TableRow,
        col_name: &str,
        rownum: u64,
        path: &Path,
    ) -> Result<T> {
        Ok(ctry!(
            row.get_cell(col_name);
            "failed to read column \"{}\" of row #{} of file \"{}\"", col_name, rownum, path.display()
        ))
    }

    /// Helper to provide error context if a `put_call` call fails.
    #[inline(always)]
    fn putcell_context<T: CasaDataType>(
        table: &mut Table,
        col_name: &str,
        row: u64,
        value: &T,
        path: &Path,
    ) -> Result<()> {
        Ok(ctry!(
            table.put_cell(col_name, row, value);
            "failed to write column \"{}\" of row #{} of file \"{}\"", col_name, row, path.display()
        ))
    }

    for row in 0..n_rows {
        ctry!(
            main_table.read_row(&mut main_row, row);
            "failed to read row #{} from \"{}\"", row, mainpath.display()
        );
        ctry!(
            work_table.read_row(&mut work_row, row);
            "failed to read row #{} from \"{}\"", row, workpath.display()
        );

        let main_flag: Array<bool, Ix2> = getcell_context(&mut main_row, "FLAG", row, &mainpath)?;
        let work_data: Array<Complex<f32>, Ix2> =
            getcell_context(&mut work_row, "DATA", row, &workpath)?;
        let work_flag: Array<bool, Ix2> = getcell_context(&mut work_row, "FLAG", row, &workpath)?;
        let work_model: Array<Complex<f32>, Ix2> =
            getcell_context(&mut work_row, "MODEL_DATA", row, &workpath)?;
        let mut work_corr: Array<Complex<f32>, Ix2> =
            getcell_context(&mut work_row, "CORRECTED_DATA", row, &workpath)?;

        let mut peel_flag = main_flag | work_flag;

        // Avoid div-by-zero.
        Zip::from(&mut work_corr)
            .and(&mut peel_flag)
            .for_each(|c, f| {
                if c.norm() == 0. {
                    *f = true;
                }

                if *f {
                    *c = Complex::from(1.0);
                }
            });

        let mut peel_model = work_data * work_model / work_corr;

        // Not strictly necessary, maybe, but I think this is nice.
        Zip::from(&mut peel_model)
            .and(&mut peel_flag)
            .for_each(|c, f| {
                if !c.is_finite() {
                    *f = true;
                }

                if *f {
                    *c = Complex::from(0.0);
                }
            });

        putcell_context(&mut main_table, "FLAG", row, &peel_flag, &mainpath)?;

        if incremental {
            let main_model: Array<Complex<f32>, Ix2> =
                getcell_context(&mut main_row, "MODEL_DATA", row, &mainpath)?;
            peel_model += &main_model;
        }

        putcell_context(&mut main_table, "MODEL_DATA", row, &peel_model, &mainpath)?;
        pb.inc();
    }

    // All done!

    pb.finish();
    Ok(0)
}

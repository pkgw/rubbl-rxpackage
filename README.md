# rubbl-rxpackage

A collection of miscellaneous astronomical data reduction utilities written in
Rust, based on the [rubbl] framework (“Rust + Hubble = rubbl = astrophysics in
Rust”). These are mainly aimed at analyzing data from the Very Large Array,
where the data sets can get very large. Certain data-intensive operations are
too slow for Python, but I find working in C++ to be extremely unpleasant.
Rust provides the speed of C++ but massively improved ergonomics.

[rubbl]: https://github.com/pkgw/rubbl

The name `rxpackage` is my shorthard for “[data] reduction package”.


## Available tools

- `rubbl rxpackage flagts` — print a time series of flagging statistics
- `rubbl rxpackage peel` — use source-specific calibration tables to implement
  “peeling” of a bright off-axis source
- `rubbl rxpackage spwglue` — combine adjacent spectral windows into one big
  one


## Installation

Prebuilt binaries are not provided. You need a Rust build toolchain installed.
If it's available, installation should be possible by just running:

```
cargo install --force --path .
```

That will make available the command-line program `rubbl-rxpackage`. If you’ve
got the core `rubbl` command installed, the tools should also be accessible by
running `rubbl rxpackage`.


## Legalities

These files are copyright Peter Williams and collaborators. They are licensed
under the MIT license.

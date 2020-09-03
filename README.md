# rubbl-rxpackage

A collection of miscellaneous astronomical data reduction utilities written in
Rust, based on the [rubbl] framework (“Rust + Hubble = rubbl = astrophysics in
Rust”). These are mainly aimed at analyzing data from the Very Large Array,
where the data sets can get very large. Certain data-intensive operations are
too slow for Python, but I find working in C++ to be extremely unpleasant.
Rust provides the speed of C++ but massively improved ergonomics.

[rubbl]: https://github.com/pkgw/rubbl

The name `rxpackage` is my shorthand for “[data] reduction package”.


## Available tools

- `rubbl rxpackage flagts` — print a time series of flagging statistics
- `rubbl rxpackage peel` — use source-specific calibration tables to implement
  “peeling” of a bright off-axis source. The intended workflow with which this
  tool assists is described briefly in the research note
  [Williams et al., 2019 RNAAS 3 110] (DOI: [10.3847/2515-5172/ab35d5]).
- `rubbl rxpackage spwglue` — combine adjacent spectral windows into one big
  one

[Williams et al., 2019 RNAAS 3 110]: https://doi.org/10.3847/2515-5172/ab35d5
[10.3847/2515-5172/ab35d5]: https://doi.org/10.3847/2515-5172/ab35d5


## Installation

Prebuilt binaries are not provided. You need a Rust build toolchain installed.
If it's available, installation should be possible by just running:

```
cargo install --force --path .
```

That will make available the command-line program `rubbl-rxpackage`. If you’ve
got the core `rubbl` command installed, the tools should also be accessible by
running `rubbl rxpackage`.


## Versioning

When you check out the main branch, you might notice that everything is built
with version `0.0.0`. This is because this project uses the [just-in-time
versioning][jitv] workflow as implemented by [Cranko]. Version numbers are
assigned inside the CI/CD pipeline. See [the GitHub release
history][gh-releases] for release history and changelogs.

[jitv]: https://pkgw.github.io/cranko/book/latest/jit-versioning/
[Cranko]: https://pkgw.github.io/cranko/book/latest/
[gh-releases]: https://github.com/pkgw/rubbl-rxpackage/releases


## Citation

If you use this software in academic work, you must cite it appropriately in
your research outputs.

The file [CITATION.cff](./CITATION.cff) in this repository aims to provide
machine-actionable citation information in the [Citation File Format]. The
short story is that *you should reference two things*: the entity that is the
specific version of the software you used, *and* the publication that
describes that software. Both of these things have DOIs, but they are
different things with different DOIs.

In particular:

- The best citation for *the software itself* is to refer to release archives
  that are stored on [Zenodo]. The pseudo-DOI [10.5281/zenodo.3403263] should
  not be used itself, but will resolve to the latest available version of the
  software, which is hopefully what you used. At the moment, you must then
  construct a BibTeX entry; here is one for version 0.1.0:
  ```
  @misc{rubbl_rxpackage_0.1.0, % NOTE: different versions will have different data!
    doi = {10.5281/zenodo.3403264},
    url = {https://doi.org/10.5281/zenodo.3403264},
    version = {v0.1.0},
    year = 2019,
    month = {sep},
    publisher = {Zenodo},
    author = {Williams, P. K. G.},
    title = {rubbl-rxpackage}
  }
  ```
  Note that the BibTeX for different releases will have different information,
  and that the optimal BibTeX entry will be different depending on your BibTeX
  style file; see the “References Section” section of [this AstroBetter post]
  for more information.
- You should also cite the publication about the software,
  [Williams et al., 2019 RNAAS 3 110], which has DOI
  [10.3847/2515-5172/ab35d5]. Here is a BibTeX entry for this publication:
  ```
  @article{Williams_2019,
	doi = {10.3847/2515-5172/ab35d5},
	url = {https://doi.org/10.3847%2F2515-5172%2Fab35d5},
	year = 2019,
	month = {jul},
	publisher = {American Astronomical Society},
	volume = {3},
	number = {7},
	pages = {110},
	author = {P. K. G. Williams and K. N. Allers and B. A. Biller and J. Vos},
	title = {A Tool and Workflow for Radio Astronomical {\textquotedblleft}Peeling{\textquotedblright} in {CASA}},
	journal = {Research Notes of the {AAS}}
  }
  ```
- There is nothing wrong with linking to this software’s GitHub repository for
  completeness, but such a link does not have long-term archival guarantees
  and is not “dereferenceable” by metadata tools.
- Adding everything up, in a gold-standard publication you might write:
  ```
  The open-source software package
  \textsf{rubbl-rxpackage}\footnote{Development currently hosted at
  \url{https://github.com/pkgw/rubbl-rxpackage}.} provides tools for
  implementing interferometric ``peeling'' \citep{Williams_2019}. We performed
  this procedure, using the workflow described by \citet{Williams_2019}, with
  version 0.1.0 of the software, which is archived in Zenodo
  \citep{rubbl_rxpackage_0.1.0}.
  ```

[Citation File Format]: https://citation-file-format.github.io/
[Zenodo]: https://zenodo.org/
[10.5281/zenodo.3403263]: https://doi.org/10.5281/zenodo.3403263
[this AstroBetter post]: https://www.astrobetter.com/blog/2019/07/01/citing-astronomy-software-inline-text-examples/

## Legalities

These files are copyright Peter Williams and collaborators. They are licensed
under the MIT license.

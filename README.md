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

There are also:

- `rubbl rxpackage show version-doi` — show the DOI of the current version of
  the software package; you can use this for citing rubbl-rxpackage
- `rubbl rxpackage show concept-doi` — show the "concept DOI" of
  rubbl-rxpackage; this is included for completeness but is probably not what
  you want. If in doubt, use the `version-doi` command.


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

To do so, you should reference *two* things: the entity that is the specific
version of the software you used, *and* the publication that describes that
software. Both of these things have DOIs, but they are different things with
different DOIs.

In particular:

- To cite *the software itself*, obtain the DOI of the release that you're
  running by executing the command `rubbl rxpackage show version-doi`. To create
  a BibTeX entry corresponding to that release, point your browser to
  `https://doi.org/{the-version-doi}`, which should take you to the [Zenodo]
  landing page for the release. Then find and follow the “BibTeX export” link.
- You should also cite the publication describing the software,
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

[Zenodo]: https://zenodo.org/


## Legalities

These files are copyright Peter Williams and collaborators. They are licensed
under the MIT license.

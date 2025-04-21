# rc: micro bump

- spwglue: fix a failure due to not-quite-correct `unsafe` Rust code (#10,
  @pkgw). Also rework the `.npy` file parser to use much more modern `nom`,
  although this should not result in any observable behavior changes.

The DOI of this release is [xx.xxxx/dev-build.rubbl-rxpackage.version][vdoi].

[vdoi]: https://doi.org/xx.xxxx/dev-build.rubbl-rxpackage.version


# rubbl-rxpackage 0.1.6 (2025-04-20)

- Update to the latest version of Rubbl and its dependencies; this
  involves a fair amount of internal reworking, but shuldn't affect
  observable behavior.
- In `peel`, try to be better about reporting errors during I/O
  (issue reported by Jimmy Lynch).

The DOI of this release is [10.5281/zenodo.15251887][vdoi].

[vdoi]: https://doi.org/10.5281/zenodo.15251887


# rubbl-rxpackage 0.1.5 (2022-12-31)

- No code changes from previous release; testing the Zenodo release
  automation. I think that in 0.1.4, there was a conflict with
  GitHub's built-in Zenodo integration, which I hope that I have
  now solved.

The DOI of this release is [10.5281/zenodo.7497313][vdoi].

[vdoi]: https://doi.org/10.5281/zenodo.7497313


# rubbl-rxpackage 0.1.4 (2022-12-31)

- spwglue: recognize SDM_CORR_BIT column in SPECTRAL_WINDOW
- Add automated DOI registration upon release with Cranko

The DOI of this release is [10.5281/zenodo.7497292][vdoi].

[vdoi]: https://doi.org/10.5281/zenodo.7497292


# rubbl-rxpackage 0.1.3 (2021-04-06)

- spwglue: recognize SDM_WINDOW_FUNCTION and SDM_NUM_BIN columns in some SPECTRAL_WINDOW tables
- Update the build to the Rust 2018 edition
- Update and streamline dependencies

# rubbl-rxpackage 0.1.2 (2020-09-03)

- Try to fix NPY file parsing by upgrading to Nom 5
- In `spwglue`, improve an error message when the glue window specification
  crossess basebands.
- Update dependencies
- Tidy up the CI

# rubbl-rxpackage 0.1.1 (2020-09-03)

- Add a Cranko-based CI/CD pipeline
- Tiny doc updates
- Not publishing to Crates.io


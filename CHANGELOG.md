# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2025-09-21
### Added
- `copy` and `copy_image` functions now support the detach mode on Linux. In detach mode, copying will spawn
a daemon that will manage the copied content instead of the parent process, useful for short scripts. See README for
details.
### Changed
- The public API now have proper docstrings visible via the built-in `help` function.
- `arboard` was bumped to 3.6.1, resulting in a **breaking change** to `paste`. Previously, if you cleared the
clipboard after copying text data into it, `paste` would have returned an empty string. Now it will raise an exception.
There are also multiple improvements and bug fixes to how image data is handled.

## [1.2.2] - 2024-10-07
### Changed
- The package can now be installed using any Python version higher than 3.7.

## [1.2.1] - 2024-08-27
### Fixed
- Initialize the inner clipboard instance lazily to prevent import-time errors in headless environments.

## [1.2.0] - 2024-04-29
### Changed
- Update `arboard` to 3.4.0.

## [1.1.1] - 2024-02-20
### Fixed
- Fix `paste_image` return type.

## [1.1.0] - 2024-02-19
### Added
- Added support for image copying/pasting with `copy_image`/`paste_image` functions.

## [1.0.1] - 2024-01-31
- Add more project metadata to be shown on PyPI and some nice badges.

## [1.0.0] - 2024-01-31
- First public release.
- Copykitten can `copy`, `paste`, and `clear` the clipboard.

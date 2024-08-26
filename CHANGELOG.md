# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.1] - 2024-08-26
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

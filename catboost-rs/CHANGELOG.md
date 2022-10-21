# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.8] - 2022-10-21
### Changed
* Bump up `catboost-sys`

## [0.1.7] - 2022-10-19
### Changed
* Fix README by adding a softlink

## [0.1.6] - 2022-10-19
### Added
* `calc_predict_proba` function that applies a sigmoid to get results as probabilities
### Changed
* Add `Sync` marker to `Model`
* Move `catboost-rs` to it's own subdirectory to make it cleaner
* Update dependencies to point to updated `catboost-sys`

## [0.1.5] - 2022-10-15
### Changed
* Updated `Cargo.toml` to point to right repository.

## [0.1.4] - 2022-10-15
### Changed
* Moved code to it's own repository, splitting it off from the original forked code

## [0.1.3] - 2022-10-15
### Changed
* Mark `Model` as `Send` as it should be thread safe according to the documentation
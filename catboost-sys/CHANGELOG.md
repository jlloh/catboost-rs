# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.1.6] - 2022-10-21
### Changed
* Remove download of libcatboost and assume user downloads it himself to keep things simple

## [0.1.5] - 2022-10-18
### Changed
* Keep a copy of the c_bindings in this repo to avoid having to clone catboost repo

## [0.1.4] - 2022-10-15
### Changed
* Moved package to it's own repository

## [0.1.2] - 2022-09-07
### Changed
* Changed build script to download libcatboost file from github release page rather than building it from source every time
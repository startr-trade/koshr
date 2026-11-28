# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-28

### Added

- The `koshr` command-line binary, answering `--version` and `--help`.
- Release archives for Linux (x86_64 and arm64), macOS (Intel and Apple silicon) and Windows, each with a SHA-256 file, a build attestation and a bill of materials; the crate on crates.io.
- A user guide (mdBook) under `docs/guide`.
- The Cargo workspace, MIT OR Apache-2.0 licences, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT and CONTRIBUTORS.
- `make ci`, matching the GitHub Actions pipeline on Linux, macOS and Windows, with dependency checks, a build that needs no C compiler, and a release workflow that publishes only after CI is green on the tagged commit.
- Brand assets and the README.
- A check that product text names no agent vendor.

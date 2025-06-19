# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/downarowiczd/rust3270/releases/tag/v0.1.0) - 2025-06-19

### Other

- Update GitHub Actions workflow for release process and add release-plz configuration
- Remove cargo-binstall and cargo-msrv installation steps from CI workflow
- Remove typo checking step from CI workflow
- Remove lychee action from CI workflow
- Remove cargo-deny step from CI workflow
- Refactor event handling and field data assignment for improved readability
- Refactor conversion implementations for Color, Highlighting, and ExtendedFieldAttribute; improve readability in WriteCommand serialization
- Add installation step for Rust Clippy in CI workflow
- Refactor imports and formatting across multiple files for improved readability and consistency
- Add installation step for nightly cargo-fmt in CI workflow
- Enhance CI/CD workflows by adding detailed release process and improving build steps
- Implement server-side components for terminal communication
- Update README to include CI badge
- Create ci.yml
- Update README to clarify rust3270 as a terminal server implementation
- Implement CP037 encoding and decoding with associated tests
- Add rustfmt configuration file with formatting settings
- initial repo setup

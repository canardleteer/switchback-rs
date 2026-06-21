# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1-0.dev.2](https://github.com/canardleteer/switchback-rs/compare/v0.0.1-0.dev.1...v0.0.1-0.dev.2) - 2026-06-21

### Added

- outbreak AsyncAPI inline schemas and link nested payload types
- add mdbook-asyncapi example and asyncapi fixture CI gates
- add Acme AsyncAPI micro corpus and load_acme_example
- populate AsyncAPI channels, bindings, and protocol attachments
- add AsyncAPI load pipeline and version dispatch
- load AsyncAPI in assemble and extend reference-manual
- add switchback-avro with vendored meta-schemas
- add kafka, amqp, and mqtt protocol wire packages
- add mdBook AsyncAPI renderer and regression goldens

### Fixed

- make publishable dev-dependencies release-plz safe

### Other

- apply rustfmt for schema outbreak changes
- apply rustfmt for publishable dev-deps MR
- refresh streetlights AsyncAPI golden fixtures
- update Cargo.lock dependencies

## [0.0.1-0.dev.1] - 2026-06-21

First release-plz publish to crates.io after bootstrap registration at
`0.0.1-0.dev.0.ffcda32`. All ten `switchback-*` workspace crates publish at
this version from committed manifests (no bootstrap dev-dep strip).

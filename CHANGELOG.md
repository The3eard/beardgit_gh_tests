# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] — 2026-03-14

### Added

- Tagging support (`--tag work`, `--tag bug`, etc.) and per-task due dates.
- Pretty `list` output with overdue-task highlighting.

### Fixed

- `tasklog done` no longer panics when given a non-existent ID; prints a
  friendly error and exits non-zero. Same fix applied to `rm`.

## [0.1.0] — 2026-02-08

### Added

- Initial release: `add`, `list`, `done`, `rm` over a JSON-backed store.
- MIT license, GitHub Actions CI, Clippy + rustfmt enforcement.

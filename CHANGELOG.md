# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-10-17

### Added

- API reference generated from source comments with tidy, including executable
  examples and their actual results for `block`, `compile`, and `call`.
- `make docs` command to build the reference PDF.
- Regression coverage for Python 3.14 template string literals (t-strings).

### Improved

- RustPython upgraded to a Python 3.14 base.

## [0.3.0]

### Fixed

- Deprecation warning in Typst 0.13

### Improved

- Python version upgraded to 3.13.

## [0.2.0]

### Fixed

- Panic when calling `open`.
- Panic when calling `datetime.now()`

### Improved

- Bundled wasm file gets smaller by removing some useless file from stdlib.

## [0.1.0]

### Added

- `block`, `compile`, and `call` API.

[Unreleased]: https://github.com/peng1999/typst-pyrunner/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/peng1999/typst-pyrunner/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/peng1999/typst-pyrunner/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/peng1999/typst-pyrunner/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/peng1999/typst-pyrunner/releases/tag/v0.1.0

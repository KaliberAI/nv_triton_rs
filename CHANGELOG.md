# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `load_unload_model` example to demonstrate model loading and unloading

### Changed

- Updated protobuf to support r24.05 of the Triton Inference Server
- Updated `build.rs` to use new methods for `tonic-build` update to 0.13
- Added `only_ready` parameter to `model_registry_index` to toggle whether to include only ready models in the index response

### Deprecated

-

### Removed

- Vendored `proto` directory in favor of a git submodule
- Extra build configuration
- `docker-compose.yaml` and related configuration

### Fixed

-

### Security

-

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- The Set types derive `Hash`, `Ord` and `PartialOrd`
- `to_repr` method to get the internal representation value.
- Implement `as_bitset` to the base enum. Allows creating a set from a single element easier in `const` context (since `.into()` is not available there).


## [v0.1.1] enum-bitset-derive - 2025-06-08

- Suppress a (false positive) Clippy warning in generated code


## [v0.1.0] enum-bitset & enum-bitset-derive - 2025-06-08

- First version published on crates.io
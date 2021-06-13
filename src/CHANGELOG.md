# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!--
## [UNRELEASED]

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security
### Docs
-->




## [UNRELEASED]

### Added
- Added `FailedProduction` struct.
- Added `FailedState` struct.
- Added `ParseResult` type.

### Changed
- [BC] Updated `RecursiveDescentParser.parse_from_tokens` to return a `Result` instead on an option.

### Fixed
- Fixed `RecursiveDescentParser.parse_from_tokens` bug which caused the parser to parse invalid tokens when multiple productions hit at least a production symbol.




## v0.2.0

### Changed
- [BC] Updated `recursive_descent_parser` to receive an iterator of `Token<TLex, TSyntax>>`.




## v0.1.0

### Added
- Added `CHANGELOG` document.
- Added `LICENSE` document.
- Added `AbstractSyntaxNode` struct.
- Added `AbstractSyntaxTree` struct.
- Added `ContextFreeGramar` struct.
- Added `ContextFreeGramarProduction` struct.
- Added `FirstFollowSymbols` struct.
- Added `RecursiveDescentParser` struct.

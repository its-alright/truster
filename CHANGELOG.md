# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- **CSV Parser** - Added CSV parsing functionality for loading data sources from CSV files
- **Global Map of Data Sources** - Implemented a global map to store and manage multiple data sources
- **DataSource Trait** - Created a trait defining the interface for all data sources with methods:
  - `get_rnd()` - Get a random data item
  - `is_empty()` - Check if the source is empty
  - `len()` - Get the number of items
  - `push()` - Add a new item to the source
- **CSV UIDs Source** - Implemented a CSV-based data source for loading UIDs from CSV files
- **Command Line Arguments** - Added CLI arguments for configuration:
  - `--profile` - Path to profile configuration file
  - `--duration` - Test duration in seconds (default: 300)
  - `--log` - Log level (default: info)
- **Config Parsing** - Added configuration parsing using `config` crate with support for:
  - Profile files (TOML format)
  - Environment variables (prefix: TRUST)
- **HTTP Method Parsing** - Added `HttpMethodType` enum with support for GET and POST methods
- **Source Type Parsing** - Added `SourceType` enum with support for:
  - Const
  - Custom
  - Dynamic
  - File
  - CSV
- **AppConfig Struct** - Created configuration structure with:
  - `name` - Application name
  - `base_url` - Base URL for endpoints
  - `endpoints` - List of endpoint configurations
  - `sources` - List of source configurations
- **AppLogger** - Added logger initialization with filtering and JSON formatting support

### Commits

| Commit | Description |
|--------|-------------|
| 62065c3 | Add CSV parser and global map of data sources |
| 22a8ade | Add DataSource trait, add CSV parsing, implement UIDs CSV source |
| 62380c6 | Add command line args |
| 71f2730 | Add config parsing |

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-28

### Added

- Core data structures: `Cell`, `Row`, `Table`
- Alignment support: `Left`, `Center`, `Right` per-cell and per-column
- Vertical alignment: `Top`, `Middle`, `Bottom` for multi-line cells
- Table styles: `Classic`, `Modern`, `Minimal`, `Compact`, `Markdown`
- Width constraints: `Fixed`, `Min`, `Max`, `Proportional`, `Wrap`
- Multi-line cell support with automatic word wrapping
- `TableBuilder` for fluent API table construction
- Column operations: `add_column`, `insert_column`, `remove_column`
- Cell spanning (colspan) via `Cell::set_span()`
- Sorting: `sort`, `sort_desc`, `sort_num`, `sort_num_desc`, `sort_by`
- Filtering: `filter_eq`, `filter_col`, `filter_has`, `filtered`
- `Padding` struct for configurable cell padding
- Zero external dependencies
- `#![forbid(unsafe_code)]` for safe Rust guarantee
- Strict clippy lints (pedantic, no panic/unwrap/expect)

[0.1.0]: https://github.com/kakilangit/crabular/releases/tag/v0.1.0

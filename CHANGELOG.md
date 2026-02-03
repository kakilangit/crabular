# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-02-03

### Added
- `FromStr` implementation for `TableStyle` to parse style from string
- Separate `crabular-cli` crate with full CLI support using clap and csv

### Changed
- Use `core::` prefix for standard library types when available

## [0.3.0] - 2026-01-30

### Added
- `Row::with_alignment(contents, alignment)` constructor for creating rows with specific alignment
- `From` trait implementations for `Row`: arrays, slices, `Vec<S>` where `S: AsRef<str>`

### Changed
- `Table::add_row()`, `set_headers()`, `row()`, `header()`, `insert_row()` now accept `Into<Row>`
- `TableBuilder::row()`, `header()`, `rows()` now accept `Into<Row>`
- Simpler API: `table.add_row(["a", "b"])` instead of `table.add_row(Row::from(&["a", "b"], Alignment::Left))`
- Use `core::` instead of `std::` for types available in both (future `no_std` compatibility)

### Removed
- **BREAKING:** `Row::from(contents, alignment)` constructor - use `Row::with_alignment(contents, alignment)` instead

## [0.2.0] - 2026-01-30

### Added

- `Display` trait implementation for zero-allocation printing
- `render_into()` method for allocation pooling with reusable buffers
- `render_cached()` method for repeated renders without recalculation
- `recalculate_widths()` method to force cache invalidation
- `Row::as_array<N>()` for fixed-size row access

### Changed

- Column widths are now cached internally using `RefCell` for better performance
- Numeric sorting now uses pre-parsing for 45-67% faster sort operations
- Render buffers are pre-allocated based on estimated output size

### Fixed

- Colspan rendering now correctly merges horizontal borders
- Junction characters properly reflect cell boundaries (┼, ┬, ┴, ─)

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
[0.2.0]: https://github.com/kakilangit/crabular/releases/tag/v0.2.0
[0.3.0]: https://github.com/kakilangit/crabular/releases/tag/v0.3.0
[0.4.0]: https://github.com/kakilangit/crabular/releases/tag/v0.4.0

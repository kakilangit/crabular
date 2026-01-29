[![CI](https://github.com/kakilangit/crabular/actions/workflows/ci.yml/badge.svg)](https://github.com/kakilangit/crabular/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/crabular.svg)](https://crates.io/crates/crabular)
[![Documentation](https://docs.rs/crabular/badge.svg)](https://docs.rs/crabular)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

![Dall-E generated crabular image](https://raw.githubusercontent.com/kakilangit/static/refs/heads/main/crabular/crabular.jpeg)

# Crabular

A high-performance ASCII table library for Rust with zero dependencies.

## Features

- **Multiple table styles** - Classic, Modern (Unicode), Minimal, Compact, Markdown
- **Flexible alignment** - Left, Center, Right per-cell and per-column
- **Vertical alignment** - Top, Middle, Bottom for multi-line cells
- **Width constraints** - Fixed, Min, Max, Proportional, Wrap
- **Multi-line cells** - Automatic word wrapping with configurable widths
- **Cell spanning** - Colspan support for merged cells
- **Sorting** - Sort by column (alphabetic or numeric, ascending or descending)
- **Filtering** - Filter rows by exact match, predicate, or substring
- **Builder API** - Fluent interface for table construction
- **Zero dependencies** - No external crates required
- **Safe Rust** - `#![forbid(unsafe_code)]`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
crabular = "0.1"
```

## Quick Start

```rust
use crabular::{Table, Row, Alignment, TableStyle};

let mut table = Table::new();
table.set_style(TableStyle::Modern);
table.set_headers(Row::from(&["Name", "Age", "City"], Alignment::Left));
table.add_row(Row::from(&["Kelana", "30", "Berlin"], Alignment::Left));
table.add_row(Row::from(&["Kata", "25", "Yogyakarta"], Alignment::Left));

println!("{}", table.render());
```

Output:
```text
┌────────┬─────┬────────────┐
│ Name   │ Age │ City       │
├────────┼─────┼────────────┤
│ Kelana │ 30  │ Berlin     │
│ Kata   │ 25  │ Yogyakarta │
└────────┴─────┴────────────┘
```

## Builder API

For a more fluent experience, use `TableBuilder`:

```rust
use crabular::{TableBuilder, TableStyle, Alignment, WidthConstraint};

TableBuilder::new()
    .style(TableStyle::Modern)
    .header(&["ID", "Name", "Score"])
    .constrain(0, WidthConstraint::Fixed(5))
    .constrain(1, WidthConstraint::Min(15))
    .align(2, Alignment::Right)
    .rows([
        vec!["1", "Kelana", "95.5"],
        vec!["2", "Kata", "87.2"],
        vec!["3", "Cherry Blossom", "92.0"],
    ])
    .print();
```

## Table Styles

```rust
use crabular::TableStyle;

// Available styles:
let _ = TableStyle::Classic;   // +---+---+ with | and -
let _ = TableStyle::Modern;    // Unicode box-drawing characters
let _ = TableStyle::Minimal;   // Header separator only
let _ = TableStyle::Compact;   // No outer borders
let _ = TableStyle::Markdown;  // GitHub-flavored markdown tables
```

### Classic
```text
+-----------------+-----+---------------+
| Name            | Age | City          |
+-----------------+-----+---------------+
| Kelana          | 30  | Berlin        |
| Kata            | 25  | Yogyakarta    |
| Cherry Blossom  | 35  | Bikini Bottom |
+-----------------+-----+---------------+
```

### Modern
```text
┌─────────────────┬─────┬───────────────┐
│ Name            │ Age │ City          │
├─────────────────┼─────┼───────────────┤
│ Kelana          │ 30  │ Berlin        │
│ Kata            │ 25  │ Yogyakarta    │
│ Cherry Blossom  │ 35  │ Bikini Bottom │
└─────────────────┴─────┴───────────────┘
```

### Minimal
```text
  Name              Age    City           
──────────────────────────────────────────
  Kelana            30     Berlin         
  Kata              25     Yogyakarta     
  Cherry Blossom    35     Bikini Bottom  
```

### Compact
```text
│ Name            │ Age  │ City          │
──────────────────┼──────┼────────────────
│ Kelana          │ 30   │ Berlin        │
│ Kata            │ 25   │ Yogyakarta    │
│ Cherry Blossom  │ 35   │ Bikini Bottom │
```

### Markdown
```text
| Name           | Age | City          |
|----------------|-----|---------------|
| Kelana         | 30  | Berlin        |
| Kata           | 25  | Yogyakarta    |
| Cherry Blossom | 35  | Bikini Bottom |
```

## Width Constraints

Control column widths with various constraints:

```rust
use crabular::{Table, WidthConstraint};

let mut table = Table::new();

// Fixed width (exactly N characters)
table.constrain(WidthConstraint::Fixed(20));

// Minimum width (at least N characters)
table.constrain(WidthConstraint::Min(10));

// Maximum width (at most N characters, truncates if needed)
table.constrain(WidthConstraint::Max(30));

// Proportional (percentage of available width)
table.constrain(WidthConstraint::Proportional(50));

// Wrap (word wrap at N characters)
table.constrain(WidthConstraint::Wrap(25));
```

## Alignment

```rust
use crabular::{Table, Row, Alignment};

let mut table = Table::new();

// Set column alignment
table.align(0, Alignment::Left);
table.align(1, Alignment::Center);
table.align(2, Alignment::Right);

// Per-cell alignment via Row::from
let row = Row::from(&["text"], Alignment::Center);
table.add_row(row);
```

## Vertical Alignment

For multi-line cells:

```rust
use crabular::{Table, VerticalAlignment};

let mut table = Table::new();

table.valign(VerticalAlignment::Top);    // Default
table.valign(VerticalAlignment::Middle);
table.valign(VerticalAlignment::Bottom);
```

## Cell Spanning (Colspan)

Create cells that span multiple columns:

```rust
use crabular::{Table, Cell, Row, Alignment};

let mut table = Table::new();
table.set_headers(Row::from(&["A", "B", "C"], Alignment::Left));

let mut row = Row::new();
let mut merged = Cell::new("Spans two columns", Alignment::Center);
merged.set_span(2);  // This cell spans 2 columns
row.push(merged);
row.push(Cell::new("Normal", Alignment::Left));
table.add_row(row);
```

## Sorting

Sort table rows by any column:

```rust
use crabular::{Table, Row, Alignment};

let mut table = Table::new();
table.add_row(Row::from(&["Kelana", "30"], Alignment::Left));
table.add_row(Row::from(&["Kata", "25"], Alignment::Left));

// Alphabetic sorting
table.sort(0);           // Ascending by column 0
table.sort_desc(0);      // Descending by column 0

// Numeric sorting
table.sort_num(1);       // Ascending numeric by column 1
table.sort_num_desc(1);  // Descending numeric by column 1

// Custom sorting - compare by first column content
table.sort_by(|a, b| {
    let a_content = a.cells().first().map_or("", |c| c.content());
    let b_content = b.cells().first().map_or("", |c| c.content());
    a_content.cmp(b_content)
});
```

## Filtering

Filter rows based on conditions:

```rust
use crabular::{Table, Row, Alignment};

let mut table = Table::new();
table.add_row(Row::from(&["Kelana", "Active", "100"], Alignment::Left));
table.add_row(Row::from(&["Kata", "Inactive", "50"], Alignment::Left));
table.add_row(Row::from(&["Cherry Blossom", "Active", "75"], Alignment::Left));

// Exact match - keeps rows where column 1 equals "Active"
table.filter_eq(1, "Active");

// Substring match - keeps rows where column 0 contains "Kelana"
// table.filter_has(0, "Kelana");

// Custom predicate on column - keeps rows where column 2 > 50
// table.filter_col(2, |val| val.parse::<i32>().unwrap_or(0) > 50);

// Full row predicate - keeps rows with more than 2 cells
let filtered = table.filtered(|row| row.len() > 2);
let _ = filtered;
```

## Column Operations

```rust
use crabular::{Table, Row, Alignment};

let mut table = Table::new();
table.set_headers(Row::from(&["A", "B"], Alignment::Left));
table.add_row(Row::from(&["1", "2"], Alignment::Left));
table.add_row(Row::from(&["3", "4"], Alignment::Left));

// Add column at the end (first value is header, rest are row values)
table.add_column(&["C", "5", "6"], Alignment::Left);

// Insert column at position (first value is header, rest are row values)
table.insert_column(1, &["X", "a", "b"], Alignment::Center);

// Remove column
table.remove_column(2);
```

## API Reference

### Table

| Method | Description |
|--------|-------------|
| `new()` | Create empty table |
| `set_headers(row)` | Set header row |
| `add_row(row)` | Add data row |
| `render()` | Render to string |
| `print()` | Print to stdout |
| `set_style(style)` | Set table style |
| `align(col, alignment)` | Set column alignment |
| `valign(alignment)` | Set vertical alignment |
| `constrain(constraint)` | Add width constraint |
| `sort(col)` | Sort ascending |
| `sort_desc(col)` | Sort descending |
| `sort_num(col)` | Sort numeric ascending |
| `sort_num_desc(col)` | Sort numeric descending |
| `filter_eq(col, value)` | Filter by exact match |
| `filter_has(col, substr)` | Filter by substring |
| `filter_col(col, pred)` | Filter by predicate |

### `TableBuilder`

| Method | Description |
|--------|-------------|
| `new()` | Create new builder |
| `style(style)` | Set table style |
| `header(cells)` | Set header row |
| `row(cells)` | Add data row |
| `rows(data)` | Add multiple rows |
| `align(col, alignment)` | Set column alignment |
| `valign(alignment)` | Set vertical alignment |
| `constrain(col, constraint)` | Set column constraint |
| `padding(padding)` | Set cell padding |
| `build()` | Build table |
| `render()` | Build and render |
| `print()` | Build and print |

## License

MIT License - see [LICENSE](LICENSE) for details.

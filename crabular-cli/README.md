# crabular-cli

A CLI tool for generating ASCII tables using the [crabular](https://github.com/kakilangit/crabular) library.

## Installation

```bash
cargo install crabular-cli
```

Or build from source:

```bash
git clone https://github.com/kakilangit/crabular
cd crabular
cargo build --release --package crabular-cli
```

## Usage

### From file or stdin

```bash
# From CSV file (default: first row is header)
crabular-cli -i data.csv

# Treat all rows as data (no header row)
crabular-cli -i data.csv --no-header

# Skip first row, treat remaining as data
crabular-cli -i data.csv --skip-header

# From stdin
cat data.csv | crabular-cli -i -

# JSON input
crabular-cli -i data.json --format json

# JSONL input
crabular-cli -i data.jsonl --format jsonl
```

### Inline data

```bash
# Inline data from stdin
printf "Name,Age,City\nJohn,30,NYC\nJane,25,LA\n" | crabular-cli -i -
```

### Options

```
-s, --style <STYLE>       Table style [default: modern]
-i, --input <INPUT>       Input file (use - for stdin) [required]
-o, --output <OUTPUT>     Output to file
-S, --separator <SEP>     Field separator [default: auto-detect from format]
    --format <FORMAT>     Data format: csv, tsv, ssv, json, jsonl [default: csv]
    --no-header           Treat all rows as data (no header row) [default: false]
    --skip-header         Skip first row, treat remaining as data [default: false]
-h, --help                Print help
-V, --version             Print version
```

### Header Options

**Default behavior:** First row is treated as header

```bash
crabular-cli -i data.csv
# Output:
# ┌─────┬──────┬───────┐
# │ ID  │ Name │ Score │
# ├─────┼──────┼───────┤
# │ 1   │ John │ 95.5  │
# └─────┴──────┴───────┘
```

**`--no-header`:** All rows are data (no header row)

```bash
crabular-cli -i data.csv --no-header
# Output:
# ┌─────┬──────┬───────┐
# │ ID  │ Name │ Score │
# │ 1   │ John │ 95.5  │
# └─────┴──────┴───────┘
```

**`--skip-header`:** Skip first row, remaining rows are data

```bash
crabular-cli -i data.csv --skip-header
# Output:
# ┌────┬──────┬──────┐
# │ 1  │ John │ 95.5 │
# └────┴──────┴──────┘
```

## Examples

### Markdown format

```bash
crabular-cli -i data.csv -s markdown
```

### Custom separator

```bash
crabular-cli -i data.txt -S '|'
```

### JSON input

```bash
crabular-cli -i data.json --format json
```

### Output to file

```bash
crabular-cli -i data.csv -o output.txt
```

## Supported Formats

- **CSV** (default) - Comma-separated values
- **TSV** - Tab-separated values
- **SSV** - Space-separated values
- **JSON** - JSON array of objects
- **JSONL** - JSON Lines (one JSON object per line)

## Table Styles

- **classic** - ASCII borders with `+` and `-`
- **modern** (default) - Unicode box-drawing characters
- **minimal** - Header separator only
- **compact** - No outer borders
- **markdown** - GitHub-flavored markdown tables

# crabular-cli

A CLI tool for generating ASCII tables using the [crabular](https://github.com/kakilangit/crabular) library.

## Installation

```bash
cargo install crabular-cli
```

Or build from source:

```bash
git clone https://github.com/kakilangit/crabular
cd crabular/crabular-cli
cargo build --release
```

## Usage

### From file or stdin

```bash
# From CSV file
crabular-cli -i data.csv

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
-s, --style <STYLE>    Table style [default: modern]
-i, --input <INPUT>    Input file (use - for stdin)
-o, --output <OUTPUT>   Output to file
-S, --separator <SEP>   Field separator [default: ,]
    --format <FORMAT>    Data format: csv, tsv, ssv, json, jsonl [default: csv]
    --has-header         First row is header [default: true]
-h, --help              Print help
-V, --version           Print version
```

## Examples

### Basic table

```bash
crabular-cli -i data.csv
```

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

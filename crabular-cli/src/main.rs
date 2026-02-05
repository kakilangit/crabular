use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use crabular::{TableBuilder, TableStyle};
use serde_json::Value;

#[derive(Debug, Parser)]
#[command(name = "crabular")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value = "modern")]
    style: StyleArg,

    #[arg(short, long)]
    input: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short = 'S', long, default_value = ",")]
    separator: String,

    #[arg(long, value_enum, default_value = "csv")]
    format: DataFormat,

    #[arg(long, default_value = "false")]
    no_header: bool,

    #[arg(long, default_value = "false")]
    skip_header: bool,

    #[arg(long, value_name = "N")]
    truncate: Option<usize>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DataFormat {
    Csv,
    Tsv,
    Ssv,
    Json,
    Jsonl,
}

impl DataFormat {
    fn default_separator(self) -> &'static str {
        match self {
            DataFormat::Csv | DataFormat::Json | DataFormat::Jsonl => ",",
            DataFormat::Tsv => "\t",
            DataFormat::Ssv => " ",
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum StyleArg {
    Classic,
    Modern,
    Minimal,
    Compact,
    Markdown,
}

impl From<StyleArg> for TableStyle {
    fn from(val: StyleArg) -> Self {
        match val {
            StyleArg::Classic => TableStyle::Classic,
            StyleArg::Modern => TableStyle::Modern,
            StyleArg::Minimal => TableStyle::Minimal,
            StyleArg::Compact => TableStyle::Compact,
            StyleArg::Markdown => TableStyle::Markdown,
        }
    }
}

#[derive(Debug)]
struct RowData {
    headers: Option<Vec<String>>,
    rows: Vec<Vec<String>>,
}

enum DataParser {
    Csv(CsvParser),
    Json(JsonParser),
    Jsonl(JsonlParser),
}

impl DataParser {
    fn parse(&mut self, reader: Box<dyn Read>) -> io::Result<RowData> {
        match self {
            DataParser::Csv(p) => p.parse(reader),
            DataParser::Json(_) => JsonParser::parse(reader),
            DataParser::Jsonl(_) => JsonlParser::parse(reader),
        }
    }
}

struct CsvParser {
    separator: String,
    no_header: bool,
    skip_header: bool,
}

impl CsvParser {
    fn new(separator: String, no_header: bool, skip_header: bool) -> Self {
        Self {
            separator,
            no_header,
            skip_header,
        }
    }

    fn parse(&mut self, mut reader: Box<dyn Read>) -> io::Result<RowData> {
        let separator_char = self.separator.chars().next().unwrap_or(',');

        let mut rdr: csv::Reader<_> = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(separator_char as u8)
            .from_reader(&mut *reader);

        let mut headers: Option<Vec<String>> = None;
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut first_row = true;

        for result in rdr.records() {
            let record: csv::StringRecord = result?;
            let row: Vec<String> = record.iter().map(ToString::to_string).collect();

            if self.skip_header && first_row {
                first_row = false;
                continue;
            }

            first_row = false;

            if self.no_header || self.skip_header {
                rows.push(row);
            } else if headers.is_none() {
                headers = Some(row);
            } else {
                rows.push(row);
            }
        }

        Ok(RowData { headers, rows })
    }
}

fn extract_row(obj: &serde_json::Map<String, Value>, keys: &mut Vec<String>) -> Vec<String> {
    if keys.is_empty() {
        *keys = obj.keys().cloned().collect();
    }

    keys.iter()
        .map(|k| {
            let v = obj.get(k);
            match v {
                Some(Value::String(s)) => s.clone(),
                Some(v) => serde_json::to_string(v).unwrap_or_default(),
                None => String::new(),
            }
        })
        .collect()
}

struct JsonParser;

impl JsonParser {
    fn new() -> Self {
        Self
    }

    fn parse(mut reader: Box<dyn Read>) -> io::Result<RowData> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let value: Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => {
                return Ok(RowData {
                    headers: None,
                    rows: vec![vec!["Invalid JSON format".to_string()]],
                });
            }
        };

        let mut keys: Vec<String> = Vec::new();
        let rows: Vec<Vec<String>> = match value {
            Value::Array(arr) => arr
                .iter()
                .filter_map(|item| {
                    if let Value::Object(obj) = item {
                        Some(extract_row(obj, &mut keys))
                    } else {
                        None
                    }
                })
                .collect(),
            Value::Object(obj) => {
                vec![extract_row(&obj, &mut keys)]
            }
            _ => vec![],
        };

        let headers = if keys.is_empty() { None } else { Some(keys) };

        Ok(RowData { headers, rows })
    }
}

struct JsonlParser;

impl JsonlParser {
    fn new() -> Self {
        Self
    }

    fn parse(mut reader: Box<dyn Read>) -> io::Result<RowData> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let mut keys: Vec<String> = Vec::new();
        let rows: Vec<Vec<String>> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let value: Value = serde_json::from_str(line).ok()?;
                if let Value::Object(obj) = &value {
                    Some(extract_row(obj, &mut keys))
                } else {
                    None
                }
            })
            .collect();

        let headers = if keys.is_empty() { None } else { Some(keys) };

        Ok(RowData { headers, rows })
    }
}

fn create_parser(
    format: DataFormat,
    separator: String,
    no_header: bool,
    skip_header: bool,
) -> DataParser {
    match format {
        DataFormat::Csv | DataFormat::Tsv | DataFormat::Ssv => {
            DataParser::Csv(CsvParser::new(separator, no_header, skip_header))
        }
        DataFormat::Json => DataParser::Json(JsonParser::new()),
        DataFormat::Jsonl => DataParser::Jsonl(JsonlParser::new()),
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let style: TableStyle = args.style.into();

    let mut builder = TableBuilder::new().style(style);
    if let Some(limit) = args.truncate {
        builder = builder.truncate(limit);
    }

    let file: Box<dyn Read> = if let Some(input_path) = &args.input {
        if input_path.as_os_str() == "-" {
            Box::new(io::stdin())
        } else {
            Box::new(fs::File::open(input_path)?)
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No input specified. Use -i FILE or pipe data via stdin",
        ));
    };

    let separator = if args.separator == "," {
        args.format.default_separator().to_string()
    } else {
        args.separator.clone()
    };

    let mut data_parser = create_parser(args.format, separator, args.no_header, args.skip_header);
    let data = data_parser.parse(file)?;

    if let Some(headers) = data.headers {
        builder = builder.header(headers.iter().map(String::as_str).collect::<Vec<_>>());
    }

    for row in &data.rows {
        builder = builder.row(row.iter().map(String::as_str).collect::<Vec<_>>());
    }

    let output = builder.render();

    if let Some(output_path) = args.output {
        fs::write(output_path, &output)?;
    } else {
        print!("{output}");
    }

    Ok(())
}

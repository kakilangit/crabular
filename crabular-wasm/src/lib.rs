//! WebAssembly bindings for crabular
//!
//! This crate provides JavaScript bindings for the crabular ASCII table library,
//! enabling browser and Node.js usage.

use core::cell::RefCell;
use crabular::{Alignment, Padding, Table, TableBuilder, TableStyle, VerticalAlignment};
use js_sys::Array;
use wasm_bindgen::prelude::*;

/// WASM-friendly table builder for JavaScript
#[wasm_bindgen]
pub struct JsTable {
    builder: RefCell<TableBuilder>,
}

/// A built table that can be manipulated
#[wasm_bindgen]
pub struct JsTableObject {
    table: RefCell<Table>,
}

impl Default for JsTable {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl JsTable {
    /// Create a new table builder
    #[must_use]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            builder: RefCell::new(TableBuilder::new()),
        }
    }

    /// Set the table style
    #[wasm_bindgen(js_name = style)]
    pub fn set_style(&self, style: &str) {
        let table_style = parse_style(style);
        let builder = self.builder.take();
        let new_builder = builder.style(table_style);
        self.builder.replace(new_builder);
    }

    /// Add a header row
    #[wasm_bindgen(js_name = header)]
    pub fn set_header(&self, headers: &Array) {
        let headers_vec = array_to_vec(headers);
        let builder = self.builder.take();
        let new_builder = builder.header(headers_vec);
        self.builder.replace(new_builder);
    }

    /// Add a data row
    #[wasm_bindgen(js_name = row)]
    pub fn add_row(&self, row: &Array) {
        let row_vec = array_to_vec(row);
        let builder = self.builder.take();
        let new_builder = builder.row(row_vec);
        self.builder.replace(new_builder);
    }

    /// Add multiple rows at once
    #[wasm_bindgen(js_name = rows)]
    pub fn add_rows(&self, rows: &Array) {
        let mut builder = self.builder.take();
        for row in rows.iter() {
            if let Ok(arr) = row.dyn_into::<Array>() {
                let row_vec = array_to_vec(&arr);
                builder = builder.row(row_vec);
            }
        }
        self.builder.replace(builder);
    }

    /// Set alignment for a specific column
    #[wasm_bindgen(js_name = align)]
    pub fn set_align(&self, column: usize, alignment: &str) {
        let align = parse_alignment(alignment);
        let builder = self.builder.take();
        let new_builder = builder.align(column, align);
        self.builder.replace(new_builder);
    }

    /// Set vertical alignment for all cells
    #[wasm_bindgen(js_name = valign)]
    pub fn set_valign(&self, alignment: &str) {
        let valign = parse_vertical_alignment(alignment);
        let builder = self.builder.take();
        let new_builder = builder.valign(valign);
        self.builder.replace(new_builder);
    }

    /// Set cell padding
    #[wasm_bindgen(js_name = padding)]
    pub fn set_padding(&self, left: usize, right: usize) {
        let builder = self.builder.take();
        let new_builder = builder.padding(Padding::new(left, right));
        self.builder.replace(new_builder);
    }

    /// Set column spacing
    #[wasm_bindgen(js_name = spacing)]
    pub fn set_spacing(&self, spacing: usize) {
        let builder = self.builder.take();
        let new_builder = builder.spacing(spacing);
        self.builder.replace(new_builder);
    }

    /// Render the table to a string
    #[wasm_bindgen]
    pub fn render(&self) -> String {
        let builder = self.builder.take();
        let result = builder.render();
        self.builder.replace(TableBuilder::new());
        result
    }

    /// Build and return the table object
    #[wasm_bindgen(js_name = build)]
    pub fn build_table(&self) -> JsTableObject {
        let builder = self.builder.take();
        JsTableObject {
            table: RefCell::new(builder.build()),
        }
    }
}

#[wasm_bindgen]
impl JsTableObject {
    /// Get the number of rows
    #[wasm_bindgen(getter)]
    pub fn len(&self) -> usize {
        self.table.borrow().len()
    }

    /// Check if the table is empty
    #[wasm_bindgen(getter, js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.table.borrow().is_empty()
    }

    /// Get the number of columns
    #[wasm_bindgen(getter)]
    pub fn cols(&self) -> usize {
        self.table.borrow().cols()
    }

    /// Add a row to the table
    #[wasm_bindgen(js_name = addRow)]
    pub fn add_row(&self, row: &Array) {
        let row_vec = array_to_vec(row);
        self.table.borrow_mut().add_row(row_vec);
    }

    /// Sort by a column (ascending)
    #[wasm_bindgen]
    pub fn sort(&self, column: usize) {
        self.table.borrow_mut().sort(column);
    }

    /// Sort by a column (descending)
    #[wasm_bindgen(js_name = sortDesc)]
    pub fn sort_desc(&self, column: usize) {
        self.table.borrow_mut().sort_desc(column);
    }

    /// Sort by a column numerically (ascending)
    #[wasm_bindgen(js_name = sortNum)]
    pub fn sort_num(&self, column: usize) {
        self.table.borrow_mut().sort_num(column);
    }

    /// Sort by a column numerically (descending)
    #[wasm_bindgen(js_name = sortNumDesc)]
    pub fn sort_num_desc(&self, column: usize) {
        self.table.borrow_mut().sort_num_desc(column);
    }

    /// Filter rows by exact column match
    #[wasm_bindgen(js_name = filterEq)]
    pub fn filter_eq(&self, column: usize, value: &str) {
        self.table.borrow_mut().filter_eq(column, value);
    }

    /// Filter rows where column contains substring
    #[wasm_bindgen(js_name = filterHas)]
    pub fn filter_has(&self, column: usize, substring: &str) {
        self.table.borrow_mut().filter_has(column, substring);
    }

    /// Render the table to a string
    #[wasm_bindgen]
    pub fn render(&self) -> String {
        self.table.borrow().render()
    }

    /// Render to a string (for JavaScript's toString)
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_js(&self) -> String {
        self.table.borrow().render()
    }
}

/// Convenience function to create and render a table in one call
#[wasm_bindgen(js_name = createTable)]
#[allow(clippy::needless_pass_by_value)]
pub fn create_table(data: &Array, style: Option<String>) -> String {
    let table_style = style.as_deref().map_or(TableStyle::Classic, parse_style);

    let mut builder = TableBuilder::new().style(table_style);

    let mut iter = data.iter();
    if let Some(first_row) = iter.next()
        && let Ok(arr) = first_row.dyn_into::<Array>()
    {
        let row_vec = array_to_vec(&arr);
        builder = builder.header(row_vec);
    }

    for row in iter {
        if let Ok(arr) = row.dyn_into::<Array>() {
            let row_vec = array_to_vec(&arr);
            builder = builder.row(row_vec);
        }
    }

    builder.render()
}

/// Render a simple table from rows
#[wasm_bindgen(js_name = renderRows)]
#[allow(clippy::needless_pass_by_value)]
pub fn render_rows(rows: &Array, style: Option<String>) -> String {
    let table_style = style.as_deref().map_or(TableStyle::Classic, parse_style);

    let mut builder = TableBuilder::new().style(table_style);

    for row in rows.iter() {
        if let Ok(arr) = row.dyn_into::<Array>() {
            let row_vec = array_to_vec(&arr);
            builder = builder.row(row_vec);
        }
    }

    builder.render()
}

fn parse_style(style: &str) -> TableStyle {
    style.parse().unwrap_or(TableStyle::Classic)
}

fn parse_alignment(align: &str) -> Alignment {
    align.parse().unwrap_or(Alignment::Left)
}

fn parse_vertical_alignment(align: &str) -> VerticalAlignment {
    align.parse().unwrap_or(VerticalAlignment::Top)
}

fn array_to_vec(arr: &Array) -> Vec<&str> {
    arr.iter()
        .filter_map(|val| val.as_string())
        .map(|s| {
            // Leak the string to get a &'static str
            // This is safe for wasm as the strings are short-lived during conversion
            Box::leak(s.into_boxed_str()) as &str
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parse_alignment;
    use crate::parse_style;
    use crate::parse_vertical_alignment;
    use crabular::{Alignment, TableStyle, VerticalAlignment};

    #[test]
    fn test_parse_style() {
        assert_eq!(parse_style("classic"), TableStyle::Classic);
        assert_eq!(parse_style("modern"), TableStyle::Modern);
        assert_eq!(parse_style("MARKDOWN"), TableStyle::Markdown);
        assert_eq!(parse_style("unknown"), TableStyle::Classic);
    }

    #[test]
    fn test_parse_alignment() {
        assert_eq!(parse_alignment("left"), Alignment::Left);
        assert_eq!(parse_alignment("center"), Alignment::Center);
        assert_eq!(parse_alignment("right"), Alignment::Right);
        assert_eq!(parse_alignment("unknown"), Alignment::Left);
    }

    #[test]
    fn test_parse_vertical_alignment() {
        assert_eq!(parse_vertical_alignment("top"), VerticalAlignment::Top);
        assert_eq!(
            parse_vertical_alignment("middle"),
            VerticalAlignment::Middle
        );
        assert_eq!(
            parse_vertical_alignment("bottom"),
            VerticalAlignment::Bottom
        );
        assert_eq!(parse_vertical_alignment("unknown"), VerticalAlignment::Top);
    }
}

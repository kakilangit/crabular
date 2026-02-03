use crate::alignment::Alignment;
use crate::constraint::WidthConstraint;
use crate::padding::Padding;
use crate::row::Row;
use crate::style::TableStyle;
use crate::table::Table;
use crate::vertical_alignment::VerticalAlignment;

/// A builder for creating tables with a fluent API.
///
/// # Example
/// ```
/// use crabular::{TableBuilder, TableStyle, Alignment, WidthConstraint};
///
/// let table = TableBuilder::new()
///     .style(TableStyle::Modern)
///     .header(["ID", "Name", "Score"])
///     .constrain(0, WidthConstraint::Min(3))
///     .constrain(1, WidthConstraint::Fixed(20))
///     .align(2, Alignment::Right)
///     .row(["1", "Kata", "95.5"])
///     .row(["2", "Kata", "87.2"])
///     .build();
/// ```
#[derive(Default)]
pub struct TableBuilder {
    table: Table,
}

impl TableBuilder {
    /// Creates a new `TableBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    /// Sets the table style.
    #[must_use]
    pub fn style(mut self, style: TableStyle) -> Self {
        self.table.set_style(style);
        self
    }

    /// Sets the table headers.
    #[must_use]
    pub fn header<R: Into<Row>>(mut self, headers: R) -> Self {
        self.table.set_headers(headers.into());
        self
    }

    /// Adds a row to the table.
    #[must_use]
    pub fn row<R: Into<Row>>(mut self, cells: R) -> Self {
        self.table.add_row(cells.into());
        self
    }

    /// Adds multiple rows to the table.
    #[must_use]
    pub fn rows<I, R>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: Into<Row>,
    {
        for row_data in rows {
            self.table.add_row(row_data.into());
        }
        self
    }

    /// Sets a width constraint for a specific column.
    #[must_use]
    pub fn constrain(mut self, column: usize, constraint: WidthConstraint) -> Self {
        // Ensure we have enough constraints
        while self.table.constraints().len() <= column {
            self.table.constrain(WidthConstraint::Auto);
        }
        self.table.set_constraint(column, constraint);
        self
    }

    /// Sets the alignment for a specific column.
    #[must_use]
    pub fn align(mut self, column: usize, alignment: Alignment) -> Self {
        self.table.align(column, alignment);
        self
    }

    /// Sets the vertical alignment for multi-line cells.
    #[must_use]
    pub fn valign(mut self, alignment: VerticalAlignment) -> Self {
        self.table.valign(alignment);
        self
    }

    /// Sets the padding for all cells.
    #[must_use]
    pub fn padding(mut self, padding: Padding) -> Self {
        self.table.set_padding(padding);
        self
    }

    /// Sets the spacing between columns.
    #[must_use]
    pub fn spacing(mut self, spacing: usize) -> Self {
        self.table.spacing(spacing);
        self
    }

    /// Builds and returns the table.
    #[must_use]
    pub fn build(self) -> Table {
        self.table
    }

    /// Builds the table and renders it to a string.
    #[must_use]
    pub fn render(self) -> String {
        self.table.render()
    }

    /// Builds the table and prints it to stdout.
    pub fn print(self) {
        self.table.print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let table = TableBuilder::new().build();
        assert!(table.is_empty());
    }

    #[test]
    fn with_style() {
        let cases = [
            TableStyle::Classic,
            TableStyle::Modern,
            TableStyle::Minimal,
            TableStyle::Compact,
            TableStyle::Markdown,
        ];
        for style in cases {
            let table = TableBuilder::new().style(style).build();
            assert_eq!(table.style(), style);
        }
    }

    #[test]
    fn with_header() {
        let table = TableBuilder::new().header(["A", "B", "C"]).build();
        assert!(table.headers().is_some());
        assert_eq!(table.headers().unwrap().len(), 3);
    }

    #[test]
    fn with_rows() {
        let table = TableBuilder::new()
            .row(["1", "2"])
            .row(["3", "4"])
            .row(["5", "6"])
            .build();
        assert_eq!(table.len(), 3);
    }

    #[test]
    fn with_rows_iter() {
        let table = TableBuilder::new()
            .rows([["Kata", "30"], ["Kata", "25"]])
            .build();
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn with_align() {
        let table = TableBuilder::new()
            .align(0, Alignment::Right)
            .align(1, Alignment::Center)
            .build();
        assert_eq!(table.get_align(0), Some(Alignment::Right));
        assert_eq!(table.get_align(1), Some(Alignment::Center));
    }

    #[test]
    fn with_valign() {
        let table = TableBuilder::new()
            .valign(VerticalAlignment::Middle)
            .build();
        assert_eq!(table.get_valign(), VerticalAlignment::Middle);
    }

    #[test]
    fn with_padding() {
        let table = TableBuilder::new().padding(Padding::new(2, 3)).build();
        assert_eq!(table.padding().left, 2);
        assert_eq!(table.padding().right, 3);
    }

    #[test]
    fn with_spacing() {
        let table = TableBuilder::new().spacing(3).build();
        assert_eq!(table.get_spacing(), 3);
    }

    #[test]
    fn with_constrain() {
        let table = TableBuilder::new()
            .constrain(0, WidthConstraint::Fixed(10))
            .constrain(1, WidthConstraint::Min(5))
            .build();
        assert_eq!(table.constraints().len(), 2);
    }

    #[test]
    fn render() {
        let output = TableBuilder::new()
            .header(["Name", "Age"])
            .row(["Kata", "30"])
            .render();
        assert!(!output.is_empty());
        assert!(output.contains("Name"));
        assert!(output.contains("Kata"));
    }

    #[test]
    fn full_example() {
        let table = TableBuilder::new()
            .style(TableStyle::Modern)
            .header(["ID", "Name", "Score"])
            .constrain(0, WidthConstraint::Fixed(5))
            .constrain(1, WidthConstraint::Min(10))
            .align(2, Alignment::Right)
            .valign(VerticalAlignment::Middle)
            .padding(Padding::uniform(1))
            .spacing(1)
            .row(["1", "Kata", "95.5"])
            .row(["2", "Kata", "87.2"])
            .build();

        assert_eq!(table.style(), TableStyle::Modern);
        assert_eq!(table.len(), 2);
        assert!(table.headers().is_some());
        assert_eq!(table.get_valign(), VerticalAlignment::Middle);
    }
}

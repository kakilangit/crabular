//! Example demonstrating various table styles and features.

use crabular::{Alignment, Cell, Row, Table, TableBuilder, TableStyle, WidthConstraint};

fn main() {
    demo_styles();
    demo_builder();
    demo_colspan();
    demo_invoice();
}

fn demo_styles() {
    // Basic table with Modern style
    println!("=== Modern Style ===");
    let mut table = Table::new();
    table.set_style(TableStyle::Modern);
    table.set_headers(Row::from(&["Name", "Age", "City"], Alignment::Left));
    table.add_row(Row::from(&["Kelana", "30", "Berlin"], Alignment::Left));
    table.add_row(Row::from(&["Kata", "25", "Yogyakarta"], Alignment::Left));
    table.add_row(Row::from(
        &["Cherry Blossom", "35", "Bikini Bottom"],
        Alignment::Left,
    ));
    table.print();

    // Classic style
    println!("\n=== Classic Style ===");
    table.set_style(TableStyle::Classic);
    table.print();

    // Minimal style
    println!("\n=== Minimal Style ===");
    table.set_style(TableStyle::Minimal);
    table.print();

    // Compact style
    println!("\n=== Compact Style ===");
    table.set_style(TableStyle::Compact);
    table.print();

    // Markdown style
    println!("\n=== Markdown Style ===");
    table.set_style(TableStyle::Markdown);
    table.print();
}

fn demo_builder() {
    // Using TableBuilder
    println!("\n=== TableBuilder with Constraints ===");
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
}

fn demo_colspan() {
    // Colspan example
    println!("\n=== Colspan Example ===");
    let mut table = Table::new();
    table.set_style(TableStyle::Modern);

    // Header row spanning all 5 columns
    let mut header = Row::new();
    let mut title_cell = Cell::new("span all 5 columns", Alignment::Left);
    title_cell.set_span(5);
    header.push(title_cell);
    table.set_headers(header);

    // Row: span 4 columns + 1 column
    let mut row1 = Row::new();
    let mut span4 = Cell::new("span 4 columns", Alignment::Left);
    span4.set_span(4);
    row1.push(span4);
    row1.push(Cell::new("just 1 column", Alignment::Left));
    table.add_row(row1);

    // Row: span 3 columns + span 2 columns
    let mut row2 = Row::new();
    let mut span3_left = Cell::new("span 3 columns", Alignment::Left);
    span3_left.set_span(3);
    row2.push(span3_left);
    let mut span2_right = Cell::new("span 2 columns", Alignment::Left);
    span2_right.set_span(2);
    row2.push(span2_right);
    table.add_row(row2);

    // Row: span 2 columns + span 3 columns
    let mut row3 = Row::new();
    let mut span2_left = Cell::new("span 2 columns", Alignment::Left);
    span2_left.set_span(2);
    row3.push(span2_left);
    let mut span3_right = Cell::new("span 3 columns", Alignment::Left);
    span3_right.set_span(3);
    row3.push(span3_right);
    table.add_row(row3);

    // Row: 1 column + span 4 columns
    let mut row4 = Row::new();
    row4.push(Cell::new("just 1 column", Alignment::Left));
    let mut span4_right = Cell::new("span 4 columns", Alignment::Left);
    span4_right.set_span(4);
    row4.push(span4_right);
    table.add_row(row4);

    // Row: all 5 individual columns
    table.add_row(Row::from(
        &[
            "just 1 column",
            "just 1 column",
            "just 1 column",
            "just 1 column",
            "just 1 column",
        ],
        Alignment::Left,
    ));

    table.print();
}

fn demo_invoice() {
    // Invoice style colspan example
    println!("\n=== Invoice Style with Colspan ===");
    let mut invoice = Table::new();
    invoice.set_style(TableStyle::Modern);

    // Invoice header spanning all columns
    let mut inv_header = Row::new();
    let mut inv_title = Cell::new("INVOICE #2024-001", Alignment::Center);
    inv_title.set_span(4);
    inv_header.push(inv_title);
    invoice.set_headers(inv_header);

    // Column headers
    invoice.add_row(Row::from(
        &["Item", "Qty", "Price", "Total"],
        Alignment::Center,
    ));

    // Line items
    invoice.add_row(Row::from(&["Widget A", "5", "$10", "$50"], Alignment::Left));
    invoice.add_row(Row::from(&["Widget B", "3", "$15", "$45"], Alignment::Left));
    invoice.add_row(Row::from(&["Service", "1", "$25", "$25"], Alignment::Left));

    // Subtotal row
    let mut subtotal = Row::new();
    let mut subtotal_label = Cell::new("Subtotal:", Alignment::Right);
    subtotal_label.set_span(3);
    subtotal.push(subtotal_label);
    subtotal.push(Cell::new("$120", Alignment::Right));
    invoice.add_row(subtotal);

    // Tax row
    let mut tax = Row::new();
    let mut tax_label = Cell::new("Tax (10%):", Alignment::Right);
    tax_label.set_span(3);
    tax.push(tax_label);
    tax.push(Cell::new("$12", Alignment::Right));
    invoice.add_row(tax);

    // Grand total row
    let mut grand_total = Row::new();
    let mut total_label = Cell::new("TOTAL:", Alignment::Right);
    total_label.set_span(3);
    grand_total.push(total_label);
    grand_total.push(Cell::new("$132", Alignment::Right));
    invoice.add_row(grand_total);

    invoice.print();
}

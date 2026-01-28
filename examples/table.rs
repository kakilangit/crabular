//! Example demonstrating various table styles and features.

use crabular::{Alignment, Row, Table, TableBuilder, TableStyle, WidthConstraint};

fn main() {
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

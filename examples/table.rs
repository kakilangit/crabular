#[allow(clippy::too_many_lines)]
fn main() {
    let data = [
        ("Kelana", "30", "Berlin"),
        ("Kata", "25", "Yogyakarta"),
        ("Squidward", "35", "Bikini Bottom"),
    ];

    println!("=== Table Styles ===");
    println!("\nClassic Style:");
    println!("{}", render_table(&data, crabular::TableStyle::Classic));

    println!("\nModern Style:");
    println!("{}", render_table(&data, crabular::TableStyle::Modern));

    println!("\nMinimal Style:");
    println!("{}", render_table(&data, crabular::TableStyle::Minimal));

    println!("\nCompact Style:");
    println!("{}", render_table(&data, crabular::TableStyle::Compact));

    println!("\nMarkdown Style:");
    println!("{}", render_table(&data, crabular::TableStyle::Markdown));

    println!("\n=== TableBuilder API ===");
    println!("\nUsing TableBuilder for fluent configuration:");
    crabular::TableBuilder::new()
        .style(crabular::TableStyle::Modern)
        .header(&["ID", "Name", "Score"])
        .constrain(0, crabular::WidthConstraint::Fixed(5))
        .constrain(1, crabular::WidthConstraint::Min(15))
        .align(2, crabular::Alignment::Right)
        .row(&["1", "Kelana", "95.5"])
        .row(&["2", "Kata", "87.2"])
        .row(&["3", "Patrick", "92.0"])
        .print();

    println!("\n=== Width Constraints ===");
    println!("\nFixed Constraints:");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Classic);
    table.set_headers(crabular::Row::from(
        &["Name", "Age", "Location"],
        crabular::Alignment::Left,
    ));
    table.constrain(crabular::WidthConstraint::Fixed(15));
    table.constrain(crabular::WidthConstraint::Fixed(5));
    table.constrain(crabular::WidthConstraint::Fixed(20));
    table.add_row(crabular::Row::from(
        &["Kelana", "30", "Berlin"],
        crabular::Alignment::Left,
    ));
    table.add_row(crabular::Row::from(
        &["Kata", "25", "Yogyakarta"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\nProportional Constraints (30%, 20%, 50%):");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Classic);
    table.set_headers(crabular::Row::from(
        &["Name", "Age", "Location"],
        crabular::Alignment::Left,
    ));
    table.constrain(crabular::WidthConstraint::Proportional(30));
    table.constrain(crabular::WidthConstraint::Proportional(20));
    table.constrain(crabular::WidthConstraint::Proportional(50));
    table.add_row(crabular::Row::from(
        &["Kelana", "30", "Berlin"],
        crabular::Alignment::Left,
    ));
    table.add_row(crabular::Row::from(
        &["Kata", "25", "Yogyakarta"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\nMin/Max Constraints:");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Classic);
    table.set_headers(crabular::Row::from(
        &["Name", "Age", "Location"],
        crabular::Alignment::Left,
    ));
    table.constrain(crabular::WidthConstraint::Min(10));
    table.constrain(crabular::WidthConstraint::Max(5));
    table.constrain(crabular::WidthConstraint::Min(15));
    table.add_row(crabular::Row::from(
        &["Kelana", "30", "Berlin"],
        crabular::Alignment::Left,
    ));
    table.add_row(crabular::Row::from(
        &["Kata", "25", "Yogyakarta"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\n=== Alignment ===");
    println!("\nDifferent alignments per column:");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Classic);
    table.set_headers(crabular::Row::from(
        &["Left", "Center", "Right"],
        crabular::Alignment::Left,
    ));
    table.align(0, crabular::Alignment::Left);
    table.align(1, crabular::Alignment::Center);
    table.align(2, crabular::Alignment::Right);
    table.add_row(crabular::Row::from(
        &["Left text", "Center text", "Right text"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\n=== Word Wrapping ===");
    println!("\nWrap constraint for long text:");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Classic);
    table.set_headers(crabular::Row::from(
        &["Description", "Status"],
        crabular::Alignment::Left,
    ));
    table.constrain(crabular::WidthConstraint::Wrap(20));
    table.constrain(crabular::WidthConstraint::Fixed(10));
    table.add_row(crabular::Row::from(
        &[
            "This is a very long description that should be wrapped at word boundaries",
            "Active",
        ],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\n=== Vertical Alignment ===");
    println!("\nTop alignment (default):");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Modern);
    table.constrain(crabular::WidthConstraint::Wrap(15));
    table.constrain(crabular::WidthConstraint::Fixed(8));
    table.valign(crabular::VerticalAlignment::Top);
    table.add_row(crabular::Row::from(
        &["This text wraps across multiple lines", "Short"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\nMiddle alignment:");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Modern);
    table.constrain(crabular::WidthConstraint::Wrap(15));
    table.constrain(crabular::WidthConstraint::Fixed(8));
    table.valign(crabular::VerticalAlignment::Middle);
    table.add_row(crabular::Row::from(
        &["This text wraps across multiple lines", "Short"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\nBottom alignment:");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Modern);
    table.constrain(crabular::WidthConstraint::Wrap(15));
    table.constrain(crabular::WidthConstraint::Fixed(8));
    table.valign(crabular::VerticalAlignment::Bottom);
    table.add_row(crabular::Row::from(
        &["This text wraps across multiple lines", "Short"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());

    println!("\n=== Cell Spanning (Colspan) ===");
    println!("\nCell spanning 2 columns:");
    let mut table = crabular::Table::new();
    table.set_style(crabular::TableStyle::Modern);
    table.set_headers(crabular::Row::from(
        &["Name", "Details", "Status"],
        crabular::Alignment::Left,
    ));
    table.add_row(crabular::Row::from(
        &["Kelana", "Developer", "Active"],
        crabular::Alignment::Left,
    ));

    // Row with colspan
    let mut row = crabular::Row::new();
    let mut merged = crabular::Cell::new("Merged across two columns", crabular::Alignment::Center);
    merged.set_span(2);
    row.push(merged);
    row.push(crabular::Cell::new("OK", crabular::Alignment::Left));
    table.add_row(row);

    table.add_row(crabular::Row::from(
        &["Kata", "Designer", "Inactive"],
        crabular::Alignment::Left,
    ));
    println!("{}", table.render());
}

fn render_table(data: &[(&str, &str, &str)], style: crabular::TableStyle) -> String {
    let mut table = crabular::Table::new();
    table.set_style(style);
    table.set_headers(crabular::Row::from(
        &["Name", "Age", "Location"],
        crabular::Alignment::Left,
    ));

    for (name, age, location) in data {
        table.add_row(crabular::Row::from(
            &[*name, *age, *location],
            crabular::Alignment::Left,
        ));
    }

    table.render()
}

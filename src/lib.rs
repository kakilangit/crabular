#![doc = include_str!("../README.md")]

pub mod alignment;
pub mod builder;
pub mod cell;
pub mod constraint;
pub mod padding;
pub mod row;
pub mod style;
pub mod table;
pub mod vertical_alignment;

pub use alignment::Alignment;
pub use builder::TableBuilder;
pub use cell::Cell;
pub use constraint::WidthConstraint;
pub use padding::Padding;
pub use row::Row;
pub use style::TableStyle;
pub use table::Table;
pub use vertical_alignment::VerticalAlignment;

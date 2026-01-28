use crate::alignment::Alignment;
use crate::cell::Cell;
use crate::constraint::WidthConstraint;
use crate::padding::Padding;
use crate::row::Row;
use crate::style::{BorderChars, TableStyle};
use crate::vertical_alignment::VerticalAlignment;

pub struct Table {
    rows: Vec<Row>,
    headers: Option<Row>,
    style: TableStyle,
    constraints: Vec<WidthConstraint>,
    padding: Padding,
    column_spacing: usize,
    column_alignments: Vec<Alignment>,
    vertical_alignment: VerticalAlignment,
}

impl Table {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            headers: None,
            style: TableStyle::Classic,
            constraints: Vec::new(),
            padding: Padding::default(),
            column_spacing: 1,
            column_alignments: Vec::new(),
            vertical_alignment: VerticalAlignment::Top,
        }
    }

    pub fn set_headers(&mut self, headers: Row) {
        self.headers = Some(headers);
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn insert_row(&mut self, index: usize, row: Row) {
        self.rows.insert(index, row);
    }

    pub fn remove_row(&mut self, index: usize) -> Option<Row> {
        if index < self.rows.len() {
            Some(self.rows.remove(index))
        } else {
            None
        }
    }

    /// Sorts the rows by the content of the specified column in ascending order.
    /// Uses lexicographic (string) comparison.
    pub fn sort(&mut self, column: usize) {
        self.rows.sort_by(|a, b| {
            let a_content = a.cells().get(column).map_or("", Cell::content);
            let b_content = b.cells().get(column).map_or("", Cell::content);
            a_content.cmp(b_content)
        });
    }

    /// Sorts the rows by the content of the specified column in descending order.
    /// Uses lexicographic (string) comparison.
    pub fn sort_desc(&mut self, column: usize) {
        self.rows.sort_by(|a, b| {
            let a_content = a.cells().get(column).map_or("", Cell::content);
            let b_content = b.cells().get(column).map_or("", Cell::content);
            b_content.cmp(a_content)
        });
    }

    /// Sorts the rows by the specified column, treating cell content as numbers.
    /// Non-numeric values are treated as 0.0.
    pub fn sort_num(&mut self, column: usize) {
        self.rows.sort_by(|a, b| {
            let a_val: f64 = a
                .cells()
                .get(column)
                .and_then(|c| c.content().parse().ok())
                .unwrap_or(0.0);
            let b_val: f64 = b
                .cells()
                .get(column)
                .and_then(|c| c.content().parse().ok())
                .unwrap_or(0.0);
            a_val
                .partial_cmp(&b_val)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Sorts the rows by the specified column in descending order, treating content as numbers.
    /// Non-numeric values are treated as 0.0.
    pub fn sort_num_desc(&mut self, column: usize) {
        self.rows.sort_by(|a, b| {
            let a_val: f64 = a
                .cells()
                .get(column)
                .and_then(|c| c.content().parse().ok())
                .unwrap_or(0.0);
            let b_val: f64 = b
                .cells()
                .get(column)
                .and_then(|c| c.content().parse().ok())
                .unwrap_or(0.0);
            b_val
                .partial_cmp(&a_val)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Sorts the rows using a custom comparison function.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Row, &Row) -> std::cmp::Ordering,
    {
        self.rows.sort_by(compare);
    }

    /// Filters rows in place, keeping only those for which the predicate returns true.
    /// Headers are not affected by filtering.
    pub fn filter<F>(&mut self, predicate: F)
    where
        F: FnMut(&Row) -> bool,
    {
        self.rows.retain(predicate);
    }

    /// Filters rows by the content of a specific column.
    /// Keeps rows where the column content equals the given value.
    pub fn filter_eq(&mut self, column: usize, value: &str) {
        self.rows.retain(|row| {
            row.cells()
                .get(column)
                .is_some_and(|cell| cell.content() == value)
        });
    }

    /// Filters rows by the content of a specific column using a predicate.
    /// Keeps rows where the predicate returns true for the column content.
    pub fn filter_col<F>(&mut self, column: usize, predicate: F)
    where
        F: Fn(&str) -> bool,
    {
        self.rows.retain(|row| {
            row.cells()
                .get(column)
                .is_some_and(|cell| predicate(cell.content()))
        });
    }

    /// Filters rows where the specified column content contains the given substring.
    pub fn filter_has(&mut self, column: usize, substring: &str) {
        self.rows.retain(|row| {
            row.cells()
                .get(column)
                .is_some_and(|cell| cell.content().contains(substring))
        });
    }

    /// Returns a new table containing only rows that match the predicate.
    /// The original table is not modified. Headers, style, and other settings are copied.
    #[must_use]
    pub fn filtered<F>(&self, mut predicate: F) -> Self
    where
        F: FnMut(&Row) -> bool,
    {
        Self {
            rows: self.rows.iter().filter(|r| predicate(r)).cloned().collect(),
            headers: self.headers.clone(),
            style: self.style,
            constraints: self.constraints.clone(),
            padding: self.padding,
            column_spacing: self.column_spacing,
            column_alignments: self.column_alignments.clone(),
            vertical_alignment: self.vertical_alignment,
        }
    }

    /// Adds a new column to the table with the given values.
    /// The first value becomes the header (if headers exist), and the rest become row values.
    /// If there are more rows than values, empty cells are added.
    /// If there are more values than rows, extra values are ignored.
    pub fn add_column(&mut self, values: &[&str], alignment: Alignment) {
        let mut value_iter = values.iter();

        // Add to headers if they exist
        if let Some(ref mut headers) = self.headers {
            let content = value_iter.next().copied().unwrap_or("");
            headers.push(Cell::new(content, alignment));
        }

        // Add to each row
        for row in &mut self.rows {
            let content = value_iter.next().copied().unwrap_or("");
            row.push(Cell::new(content, alignment));
        }

        // Extend column alignments to include the new column
        self.column_alignments.push(alignment);
    }

    /// Inserts a new column at the specified index.
    /// The first value becomes the header (if headers exist), and the rest become row values.
    pub fn insert_column(&mut self, index: usize, values: &[&str], alignment: Alignment) {
        let mut value_iter = values.iter();

        // Insert into headers if they exist
        if let Some(ref mut headers) = self.headers {
            let content = value_iter.next().copied().unwrap_or("");
            headers.insert(index, Cell::new(content, alignment));
        }

        // Insert into each row
        for row in &mut self.rows {
            let content = value_iter.next().copied().unwrap_or("");
            row.insert(index, Cell::new(content, alignment));
        }

        // Shift constraints if needed
        if index < self.constraints.len() {
            self.constraints.insert(index, WidthConstraint::Auto);
        }

        // Shift column alignments if needed
        if index < self.column_alignments.len() {
            self.column_alignments.insert(index, alignment);
        }
    }

    /// Removes a column at the specified index from all rows and headers.
    /// Returns true if the column was removed, false if the index was out of bounds.
    pub fn remove_column(&mut self, index: usize) -> bool {
        let mut removed = false;

        // Remove from headers if they exist
        if let Some(ref mut headers) = self.headers
            && headers.remove(index).is_some()
        {
            removed = true;
        }

        // Remove from each row
        for row in &mut self.rows {
            if row.remove(index).is_some() {
                removed = true;
            }
        }

        // Remove constraint if it exists
        if index < self.constraints.len() {
            self.constraints.remove(index);
        }

        // Remove column alignment if it exists
        if index < self.column_alignments.len() {
            self.column_alignments.remove(index);
        }

        removed
    }

    /// Returns the number of columns in the table.
    /// Based on the maximum cell count across headers and all rows.
    #[must_use]
    pub fn cols(&self) -> usize {
        let header_cols = self.headers.as_ref().map_or(0, Row::len);
        let row_cols = self.rows.iter().map(Row::len).max().unwrap_or(0);
        header_cols.max(row_cols)
    }

    pub fn set_style(&mut self, style: TableStyle) {
        self.style = style;
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }

    pub fn spacing(&mut self, spacing: usize) {
        self.column_spacing = spacing;
    }

    pub fn align(&mut self, column: usize, alignment: Alignment) {
        if column >= self.column_alignments.len() {
            self.column_alignments.resize(column + 1, Alignment::Left);
        }
        self.column_alignments[column] = alignment;
    }

    pub fn valign(&mut self, alignment: VerticalAlignment) {
        self.vertical_alignment = alignment;
    }

    pub fn constrain(&mut self, constraint: WidthConstraint) {
        self.constraints.push(constraint);
    }

    pub fn set_constraint(&mut self, column: usize, constraint: WidthConstraint) {
        if column >= self.constraints.len() {
            self.constraints.resize(column + 1, WidthConstraint::Auto);
        }
        self.constraints[column] = constraint;
    }

    #[must_use]
    pub fn constraints(&self) -> &[WidthConstraint] {
        &self.constraints
    }

    #[must_use]
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    #[must_use]
    pub fn headers(&self) -> Option<&Row> {
        self.headers.as_ref()
    }

    #[must_use]
    pub fn style(&self) -> TableStyle {
        self.style
    }

    #[must_use]
    pub fn padding(&self) -> Padding {
        self.padding
    }

    #[must_use]
    pub fn get_spacing(&self) -> usize {
        self.column_spacing
    }

    #[must_use]
    pub fn get_align(&self, column: usize) -> Option<Alignment> {
        self.column_alignments.get(column).copied()
    }

    #[must_use]
    pub fn get_valign(&self) -> VerticalAlignment {
        self.vertical_alignment
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() && self.headers.is_none()
    }

    #[must_use]
    pub fn header(self, headers: &[&str]) -> Self {
        let row = Row::from(headers, Alignment::Left);
        let mut table = self;
        table.set_headers(row);
        table
    }

    #[must_use]
    pub fn row(self, cells: &[&str]) -> Self {
        let row = Row::from(cells, Alignment::Left);
        let mut table = self;
        table.add_row(row);
        table
    }

    pub fn print(&self) {
        print!("{}", self.render());
    }

    fn format_cell(content: &str, width: usize, alignment: Alignment) -> String {
        let content_len = content.chars().count();

        if content_len > width {
            return if width > 3 {
                let truncated: String = content.chars().take(width - 3).collect();
                format!("{truncated}...")
            } else {
                ".".repeat(width)
            };
        }

        if content_len == width {
            return content.to_string();
        }

        let padding = width - content_len;
        match alignment {
            Alignment::Left => {
                format!("{}{}", content, " ".repeat(padding))
            }
            Alignment::Right => {
                format!("{}{}", " ".repeat(padding), content)
            }
            Alignment::Center => {
                let left = padding / 2;
                let right = padding - left;
                format!("{}{}{}", " ".repeat(left), content, " ".repeat(right))
            }
        }
    }

    pub(crate) fn wrap_text(text: &str, width: usize) -> Vec<String> {
        if text.is_empty() || width == 0 {
            return vec![String::new()];
        }

        if text.chars().count() <= width {
            return vec![text.to_string()];
        }

        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in words {
            let word_char_count = word.chars().count();
            if current_line.is_empty() {
                if word_char_count > width {
                    let chars: Vec<char> = word.chars().collect();
                    let truncated: String = chars.iter().take(width).collect();
                    lines.push(truncated);
                    let remaining: String = chars.iter().skip(width).collect();
                    if !remaining.is_empty() {
                        let remaining_lines = Self::wrap_text(&remaining, width);
                        lines.extend(remaining_lines);
                    }
                } else {
                    current_line.push_str(word);
                }
            } else {
                let current_char_count = current_line.chars().count();
                let potential_len = current_char_count + 1 + word_char_count;
                if potential_len <= width {
                    current_line.push(' ');
                    current_line.push_str(word);
                } else {
                    lines.push(current_line.clone());
                    current_line.clear();
                    if word_char_count > width {
                        let chars: Vec<char> = word.chars().collect();
                        let truncated: String = chars.iter().take(width).collect();
                        lines.push(truncated);
                        let remaining: String = chars.iter().skip(width).collect();
                        if !remaining.is_empty() {
                            let remaining_lines = Self::wrap_text(&remaining, width);
                            lines.extend(remaining_lines);
                        }
                    } else {
                        current_line.push_str(word);
                    }
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    fn calculate_column_widths(&self) -> Vec<usize> {
        let mut max_widths: Vec<usize> = Vec::new();

        if let Some(headers) = self.headers() {
            for (idx, cell) in headers.cells().iter().enumerate() {
                let width = cell.content().chars().count();
                if max_widths.len() < idx + 1 {
                    max_widths.resize(idx + 1, 0);
                }
                if width > max_widths[idx] {
                    max_widths[idx] = width;
                }
            }
        }

        for row in &self.rows {
            for (idx, cell) in row.cells().iter().enumerate() {
                let width = cell.content().chars().count();
                if max_widths.len() < idx + 1 {
                    max_widths.resize(idx + 1, 0);
                }
                if width > max_widths[idx] {
                    max_widths[idx] = width;
                }
            }
        }

        self.apply_width_constraints(&mut max_widths);
        self.apply_proportional_constraints(&mut max_widths);
        max_widths
    }

    fn apply_width_constraints(&self, widths: &mut [usize]) {
        for (i, constraint) in self.constraints.iter().enumerate() {
            if i < widths.len() {
                match constraint {
                    WidthConstraint::Fixed(w) => {
                        widths[i] = *w;
                    }
                    WidthConstraint::Min(m) => {
                        if widths[i] < *m {
                            widths[i] = *m;
                        }
                    }
                    WidthConstraint::Max(m) => {
                        if widths[i] > *m {
                            widths[i] = *m;
                        }
                    }
                    WidthConstraint::Wrap(w) => {
                        if widths[i] > *w {
                            widths[i] = *w;
                        }
                    }
                    WidthConstraint::Auto | WidthConstraint::Proportional(_) => {}
                }
            }
        }
    }

    fn apply_proportional_constraints(&self, widths: &mut [usize]) {
        let total_percentage: u8 = self
            .constraints
            .iter()
            .filter_map(|c| {
                if let WidthConstraint::Proportional(p) = c {
                    Some(*p)
                } else {
                    None
                }
            })
            .sum();

        if total_percentage == 0 || total_percentage > 100 {
            return;
        }

        let padding = self.padding.left + self.padding.right;
        let spacing = self
            .column_spacing
            .saturating_mul(widths.len().saturating_sub(1));
        let max_width: usize = 120;
        let available_width = max_width.saturating_sub(padding * widths.len() + spacing);

        let proportional_width = available_width;
        for (i, constraint) in self.constraints.iter().enumerate() {
            if i < widths.len()
                && let WidthConstraint::Proportional(percentage) = constraint
            {
                let calculated_width = (proportional_width * *percentage as usize) / 100;
                widths[i] = widths[i].max(calculated_width);
            }
        }
    }

    #[must_use]
    pub fn render(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut output = String::new();
        let column_widths = self.calculate_column_widths();
        let borders = self.style.border_chars();
        let is_minimal_or_compact = matches!(self.style, TableStyle::Minimal | TableStyle::Compact);

        let num_columns = column_widths.len();
        let _total_width = column_widths.iter().sum::<usize>()
            + (self.padding.left + self.padding.right) * num_columns
            + self.column_spacing * (num_columns.saturating_sub(1));

        if !is_minimal_or_compact {
            output.push_str(&Self::render_horizontal_border(
                &column_widths,
                self.padding,
                self.column_spacing,
                borders.top_left,
                borders.top_cross,
                borders.top_right,
                borders.horizontal,
            ));
        }

        if let Some(headers) = self.headers() {
            output.push_str(&self.render_row_with_wrapping(
                headers,
                &column_widths,
                &borders,
                &self.column_alignments,
            ));
            // Render header separator for all styles
            output.push_str(&Self::render_horizontal_border(
                &column_widths,
                self.padding,
                self.column_spacing,
                borders.left_cross,
                borders.cross,
                borders.right_cross,
                borders.horizontal,
            ));
        }

        for row in self.rows() {
            output.push_str(&self.render_row_with_wrapping(
                row,
                &column_widths,
                &borders,
                &self.column_alignments,
            ));
        }

        if !is_minimal_or_compact {
            output.push_str(&Self::render_horizontal_border(
                &column_widths,
                self.padding,
                self.column_spacing,
                borders.bottom_left,
                borders.bottom_cross,
                borders.bottom_right,
                borders.horizontal,
            ));
        }

        output
    }

    fn render_row_with_wrapping(
        &self,
        row: &Row,
        column_widths: &[usize],
        borders: &BorderChars,
        column_alignments: &[Alignment],
    ) -> String {
        let mut wrapped_cells: Vec<Vec<String>> = Vec::new();
        let mut cell_spans: Vec<usize> = Vec::new();
        let mut max_lines = 1;

        let mut col_idx = 0;
        for cell in row.cells() {
            let span = cell.span().max(1);
            cell_spans.push(span);

            // Calculate combined width for spanned cells
            let combined_width = self.calculate_span_width(col_idx, span, column_widths);
            let wrap_width = self.get_wrap_width(col_idx);

            let effective_width = wrap_width.unwrap_or(combined_width);
            let lines = if cell.content().chars().count() > effective_width && wrap_width.is_some()
            {
                Self::wrap_text(cell.content(), effective_width)
            } else {
                vec![cell.content().to_string()]
            };

            max_lines = max_lines.max(lines.len());
            wrapped_cells.push(lines);

            col_idx += span;
        }

        // Apply vertical alignment by calculating offset for each cell
        let aligned_cells: Vec<Vec<String>> = wrapped_cells
            .into_iter()
            .map(|cell_lines| {
                Self::apply_vertical_alignment(cell_lines, max_lines, self.vertical_alignment)
            })
            .collect();

        let mut output = String::new();
        for line_idx in 0..max_lines {
            output.push_str(borders.vertical);

            let mut col_idx = 0;
            for (cell_idx, cell_lines) in aligned_cells.iter().enumerate() {
                let span = cell_spans.get(cell_idx).copied().unwrap_or(1);
                let combined_width = self.calculate_span_width(col_idx, span, column_widths);

                let alignment = column_alignments.get(col_idx).copied().unwrap_or_else(|| {
                    row.cells()
                        .get(cell_idx)
                        .map_or(Alignment::Left, Cell::alignment)
                });

                let content = cell_lines.get(line_idx).map_or("", String::as_str);
                output.push_str(&" ".repeat(self.padding.left));
                output.push_str(&Self::format_cell(content, combined_width, alignment));
                output.push_str(&" ".repeat(self.padding.right));

                // Add spacing between cells
                if cell_idx < aligned_cells.len() - 1 {
                    output.push_str(&" ".repeat(self.column_spacing));
                }
                output.push_str(borders.vertical);

                col_idx += span;
            }
            output.push('\n');
        }

        output
    }

    /// Calculates the combined width for a cell that spans multiple columns.
    fn calculate_span_width(
        &self,
        start_col: usize,
        span: usize,
        column_widths: &[usize],
    ) -> usize {
        if span <= 1 {
            return column_widths.get(start_col).copied().unwrap_or(0);
        }

        let mut total_width = 0;
        for i in 0..span {
            let col = start_col + i;
            if col < column_widths.len() {
                total_width += column_widths[col];
                // Add padding and spacing for intermediate columns
                if i < span - 1 {
                    total_width += self.padding.left + self.padding.right + self.column_spacing + 1;
                }
            }
        }
        total_width
    }

    pub(crate) fn apply_vertical_alignment(
        cell_lines: Vec<String>,
        max_lines: usize,
        vertical_alignment: VerticalAlignment,
    ) -> Vec<String> {
        let cell_line_count = cell_lines.len();
        if cell_line_count >= max_lines {
            return cell_lines;
        }

        let padding_needed = max_lines - cell_line_count;
        let mut result = Vec::with_capacity(max_lines);

        match vertical_alignment {
            VerticalAlignment::Top => {
                result.extend(cell_lines);
                result.extend(std::iter::repeat_n(String::new(), padding_needed));
            }
            VerticalAlignment::Middle => {
                let top_padding = padding_needed / 2;
                let bottom_padding = padding_needed - top_padding;
                result.extend(std::iter::repeat_n(String::new(), top_padding));
                result.extend(cell_lines);
                result.extend(std::iter::repeat_n(String::new(), bottom_padding));
            }
            VerticalAlignment::Bottom => {
                result.extend(std::iter::repeat_n(String::new(), padding_needed));
                result.extend(cell_lines);
            }
        }

        result
    }

    fn get_wrap_width(&self, column: usize) -> Option<usize> {
        if let Some(WidthConstraint::Wrap(w)) = self.constraints.get(column) {
            return Some(*w);
        }
        None
    }

    fn render_horizontal_border(
        column_widths: &[usize],
        padding: Padding,
        column_spacing: usize,
        left: &str,
        cross: &str,
        right: &str,
        horizontal: &str,
    ) -> String {
        let mut line = String::new();

        line.push_str(left);
        for (index, &width) in column_widths.iter().enumerate() {
            line.push_str(&horizontal.repeat(padding.left + width + padding.right));
            if index < column_widths.len() - 1 {
                line.push_str(&horizontal.repeat(column_spacing));
                line.push_str(cross);
            }
        }
        line.push_str(right);
        line.push('\n');

        line
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let table = Table::new();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
        assert_eq!(table.cols(), 0);
        assert!(table.headers().is_none());
    }

    #[test]
    fn default_is_empty() {
        let table = Table::default();
        assert!(table.is_empty());
    }

    #[test]
    fn default_style_is_classic() {
        let table = Table::new();
        assert_eq!(table.style(), TableStyle::Classic);
    }

    #[test]
    fn default_padding() {
        let table = Table::new();
        assert_eq!(table.padding().left, 1);
        assert_eq!(table.padding().right, 1);
    }

    #[test]
    fn default_spacing() {
        let table = Table::new();
        assert_eq!(table.get_spacing(), 1);
    }

    #[test]
    fn default_valign_is_top() {
        let table = Table::new();
        assert_eq!(table.get_valign(), VerticalAlignment::Top);
    }

    #[test]
    fn set_headers() {
        let mut table = Table::new();
        table.set_headers(Row::from(&["A", "B"], Alignment::Left));
        assert!(table.headers().is_some());
        assert_eq!(table.headers().unwrap().len(), 2);
    }

    #[test]
    fn add_row() {
        let mut table = Table::new();
        table.add_row(Row::from(&["1", "2"], Alignment::Left));
        assert_eq!(table.len(), 1);
        assert!(!table.is_empty());
    }

    #[test]
    fn insert_row() {
        let mut table = Table::new();
        table.add_row(Row::from(&["a", "1"], Alignment::Left));
        table.add_row(Row::from(&["c", "3"], Alignment::Left));
        table.insert_row(1, Row::from(&["b", "2"], Alignment::Left));
        assert_eq!(table.len(), 3);
        assert_eq!(table.rows()[1].cells()[0].content(), "b");
    }

    #[test]
    fn remove_row() {
        let mut table = Table::new();
        table.add_row(Row::from(&["a", "1"], Alignment::Left));
        table.add_row(Row::from(&["b", "2"], Alignment::Left));
        let removed = table.remove_row(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().cells()[0].content(), "a");
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn remove_row_out_of_bounds() {
        let mut table = Table::new();
        table.add_row(Row::from(&["a"], Alignment::Left));
        assert!(table.remove_row(5).is_none());
    }

    #[test]
    fn cols() {
        let table = Table::new().header(&["A", "B", "C"]).row(&["1", "2", "3"]);
        assert_eq!(table.cols(), 3);
    }

    #[test]
    fn fluent_api() {
        let table = Table::new()
            .header(&["ID", "Value"])
            .row(&["1", "100"])
            .row(&["2", "200"]);
        assert_eq!(table.len(), 2);
        assert!(table.headers().is_some());
    }

    // Sorting tests
    #[test]
    fn sort_ascending() {
        let mut table = Table::new();
        table.add_row(Row::from(&["Charlie"], Alignment::Left));
        table.add_row(Row::from(&["Kelana"], Alignment::Left));
        table.add_row(Row::from(&["Kata"], Alignment::Left));
        table.sort(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "Charlie");
        assert_eq!(table.rows()[1].cells()[0].content(), "Kata");
        assert_eq!(table.rows()[2].cells()[0].content(), "Kelana");
    }

    #[test]
    fn sort_descending() {
        let mut table = Table::new();
        table.add_row(Row::from(&["Kelana"], Alignment::Left));
        table.add_row(Row::from(&["Charlie"], Alignment::Left));
        table.sort_desc(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "Kelana");
        assert_eq!(table.rows()[1].cells()[0].content(), "Charlie");
    }

    #[test]
    fn sort_num_ascending() {
        let mut table = Table::new();
        table.add_row(Row::from(&["100"], Alignment::Left));
        table.add_row(Row::from(&["25"], Alignment::Left));
        table.add_row(Row::from(&["50"], Alignment::Left));
        table.sort_num(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "25");
        assert_eq!(table.rows()[1].cells()[0].content(), "50");
        assert_eq!(table.rows()[2].cells()[0].content(), "100");
    }

    #[test]
    fn sort_num_descending() {
        let mut table = Table::new();
        table.add_row(Row::from(&["25"], Alignment::Left));
        table.add_row(Row::from(&["100"], Alignment::Left));
        table.sort_num_desc(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "100");
        assert_eq!(table.rows()[1].cells()[0].content(), "25");
    }

    #[test]
    fn sort_preserves_headers() {
        let mut table = Table::new();
        table.set_headers(Row::from(&["Name"], Alignment::Left));
        table.add_row(Row::from(&["Charlie"], Alignment::Left));
        table.add_row(Row::from(&["Kelana"], Alignment::Left));
        table.sort(0);
        assert_eq!(table.headers().unwrap().cells()[0].content(), "Name");
    }

    // Filter tests
    #[test]
    fn filter() {
        let mut table = Table::new();
        table.add_row(Row::from(&["Kelana", "25"], Alignment::Left));
        table.add_row(Row::from(&["Kata", "30"], Alignment::Left));
        table.add_row(Row::from(&["Charlie", "25"], Alignment::Left));
        table.filter(|row| row.cells()[1].content() == "25");
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filter_eq() {
        let mut table = Table::new();
        table.add_row(Row::from(&["Active"], Alignment::Left));
        table.add_row(Row::from(&["Inactive"], Alignment::Left));
        table.add_row(Row::from(&["Active"], Alignment::Left));
        table.filter_eq(0, "Active");
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filter_col() {
        let mut table = Table::new();
        table.add_row(Row::from(&["100"], Alignment::Left));
        table.add_row(Row::from(&["50"], Alignment::Left));
        table.add_row(Row::from(&["75"], Alignment::Left));
        table.filter_col(0, |val| val.parse::<i32>().is_ok_and(|n| n > 60));
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filter_has() {
        let mut table = Table::new();
        table.add_row(Row::from(&["Kelana Smith"], Alignment::Left));
        table.add_row(Row::from(&["Kata Jones"], Alignment::Left));
        table.add_row(Row::from(&["Charlie Smith"], Alignment::Left));
        table.filter_has(0, "Smith");
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filtered_returns_new_table() {
        let mut table = Table::new();
        table.set_style(TableStyle::Modern);
        table.add_row(Row::from(&["25"], Alignment::Left));
        table.add_row(Row::from(&["30"], Alignment::Left));
        table.add_row(Row::from(&["25"], Alignment::Left));

        let filtered = table.filtered(|row| row.cells()[0].content() == "25");
        assert_eq!(table.len(), 3); // Original unchanged
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered.style(), TableStyle::Modern);
    }

    // Column operations tests
    #[test]
    fn add_column() {
        let mut table = Table::new();
        table.set_headers(Row::from(&["A", "B"], Alignment::Left));
        table.add_row(Row::from(&["1", "2"], Alignment::Left));
        table.add_column(&["C", "3"], Alignment::Right);
        assert_eq!(table.cols(), 3);
        assert_eq!(table.headers().unwrap().cells()[2].content(), "C");
    }

    #[test]
    fn insert_column() {
        let mut table = Table::new();
        table.set_headers(Row::from(&["A", "C"], Alignment::Left));
        table.add_row(Row::from(&["1", "3"], Alignment::Left));
        table.insert_column(1, &["B", "2"], Alignment::Center);
        assert_eq!(table.headers().unwrap().cells()[1].content(), "B");
    }

    #[test]
    fn remove_column() {
        let mut table = Table::new();
        table.set_headers(Row::from(&["A", "B", "C"], Alignment::Left));
        table.add_row(Row::from(&["1", "2", "3"], Alignment::Left));
        assert!(table.remove_column(1));
        assert_eq!(table.cols(), 2);
        assert_eq!(table.headers().unwrap().cells()[1].content(), "C");
    }

    // Render tests
    #[test]
    fn render_empty_table() {
        let table = Table::new();
        assert_eq!(table.render(), "");
    }

    #[test]
    fn render_single_row() {
        let table = Table::new().row(&["a", "b"]);
        let output = table.render();
        assert!(!output.is_empty());
        assert!(output.contains('a'));
        assert!(output.contains('b'));
    }

    #[test]
    fn render_with_headers() {
        let table = Table::new().header(&["X", "Y"]).row(&["1", "2"]);
        let output = table.render();
        assert!(output.contains('X'));
        assert!(output.contains('Y'));
        assert!(output.contains('1'));
    }

    // Text wrapping tests
    #[test]
    fn wrap_text_short() {
        let lines = Table::wrap_text("hello", 10);
        assert_eq!(lines, vec!["hello"]);
    }

    #[test]
    fn wrap_text_multiple_words() {
        let lines = Table::wrap_text("hello world foo", 10);
        assert!(lines.len() >= 2);
    }

    #[test]
    fn wrap_text_long_word() {
        let lines = Table::wrap_text("supercalifragilisticexpialidocious", 10);
        assert!(lines.len() > 1);
    }

    #[test]
    fn wrap_text_unicode() {
        // Test with multi-byte UTF-8 characters (Japanese)
        let lines = Table::wrap_text("こんにちは世界", 5);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "こんにちは");
        assert_eq!(lines[1], "世界");
    }

    #[test]
    fn wrap_text_unicode_long_word() {
        // Test wrapping a long word with multi-byte characters
        let lines = Table::wrap_text("日本語テスト文字列", 4);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "日本語テ");
        assert_eq!(lines[1], "スト文字");
        assert_eq!(lines[2], "列");
    }

    #[test]
    fn wrap_text_emoji() {
        // Test with emoji (4-byte UTF-8 characters)
        let lines = Table::wrap_text("🎉🎊🎁🎄🎅", 3);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "🎉🎊🎁");
        assert_eq!(lines[1], "🎄🎅");
    }

    // Vertical alignment tests
    #[test]
    fn apply_vertical_alignment_top() {
        let lines = vec!["a".to_string()];
        let result = Table::apply_vertical_alignment(lines, 3, VerticalAlignment::Top);
        assert_eq!(result, vec!["a", "", ""]);
    }

    #[test]
    fn apply_vertical_alignment_middle() {
        let lines = vec!["a".to_string()];
        let result = Table::apply_vertical_alignment(lines, 3, VerticalAlignment::Middle);
        assert_eq!(result, vec!["", "a", ""]);
    }

    #[test]
    fn apply_vertical_alignment_bottom() {
        let lines = vec!["a".to_string()];
        let result = Table::apply_vertical_alignment(lines, 3, VerticalAlignment::Bottom);
        assert_eq!(result, vec!["", "", "a"]);
    }
}

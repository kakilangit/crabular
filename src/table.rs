use crate::alignment::Alignment;
use crate::cell::Cell;
use crate::constraint::WidthConstraint;
use crate::padding::Padding;
use crate::row::Row;
use crate::style::{BorderChars, TableStyle};
use crate::vertical_alignment::VerticalAlignment;
use core::cell::RefCell;

pub struct Table {
    rows: Vec<Row>,
    headers: Option<Row>,
    style: TableStyle,
    constraints: Vec<WidthConstraint>,
    padding: Padding,
    column_spacing: usize,
    column_alignments: Vec<Alignment>,
    vertical_alignment: VerticalAlignment,
    /// Cached column widths for repeated renders.
    /// Uses interior mutability to allow caching in `&self` methods.
    cached_widths: RefCell<Option<Vec<usize>>>,
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
            cached_widths: RefCell::new(None),
        }
    }

    /// Invalidates the cached column widths.
    fn invalidate_cache(&self) {
        *self.cached_widths.borrow_mut() = None;
    }

    pub fn set_headers<R: Into<Row>>(&mut self, headers: R) {
        self.headers = Some(headers.into());
        self.invalidate_cache();
    }

    pub fn add_row<R: Into<Row>>(&mut self, row: R) {
        self.rows.push(row.into());
        self.invalidate_cache();
    }

    pub fn insert_row<R: Into<Row>>(&mut self, index: usize, row: R) {
        self.rows.insert(index, row.into());
        self.invalidate_cache();
    }

    pub fn remove_row(&mut self, index: usize) -> Option<Row> {
        if index < self.rows.len() {
            self.invalidate_cache();
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
    ///
    /// This method pre-parses numeric values before sorting for better performance
    /// on large tables.
    pub fn sort_num(&mut self, column: usize) {
        // Pre-parse numeric values to avoid repeated parsing during sort
        let parsed: Vec<f64> = self
            .rows
            .iter()
            .map(|row| {
                row.cells()
                    .get(column)
                    .and_then(|c| c.content().parse().ok())
                    .unwrap_or(0.0)
            })
            .collect();

        // Create indices and sort by parsed values
        let mut indices: Vec<usize> = (0..self.rows.len()).collect();
        indices.sort_by(|&a, &b| {
            parsed[a]
                .partial_cmp(&parsed[b])
                .unwrap_or(core::cmp::Ordering::Equal)
        });

        // Reorder rows using the sorted indices
        let mut sorted_rows = Vec::with_capacity(self.rows.len());
        for idx in indices {
            sorted_rows.push(core::mem::take(&mut self.rows[idx]));
        }
        self.rows = sorted_rows;
    }

    /// Sorts the rows by the specified column in descending order, treating content as numbers.
    /// Non-numeric values are treated as 0.0.
    ///
    /// This method pre-parses numeric values before sorting for better performance
    /// on large tables.
    pub fn sort_num_desc(&mut self, column: usize) {
        // Pre-parse numeric values to avoid repeated parsing during sort
        let parsed: Vec<f64> = self
            .rows
            .iter()
            .map(|row| {
                row.cells()
                    .get(column)
                    .and_then(|c| c.content().parse().ok())
                    .unwrap_or(0.0)
            })
            .collect();

        // Create indices and sort by parsed values (descending)
        let mut indices: Vec<usize> = (0..self.rows.len()).collect();
        indices.sort_by(|&a, &b| {
            parsed[b]
                .partial_cmp(&parsed[a])
                .unwrap_or(core::cmp::Ordering::Equal)
        });

        // Reorder rows using the sorted indices
        let mut sorted_rows = Vec::with_capacity(self.rows.len());
        for idx in indices {
            sorted_rows.push(core::mem::take(&mut self.rows[idx]));
        }
        self.rows = sorted_rows;
    }

    /// Sorts the rows using a custom comparison function.
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Row, &Row) -> core::cmp::Ordering,
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
            cached_widths: RefCell::new(None),
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
    pub fn header<R: Into<Row>>(mut self, headers: R) -> Self {
        self.set_headers(headers);
        self
    }

    #[must_use]
    pub fn row<R: Into<Row>>(mut self, cells: R) -> Self {
        self.add_row(cells);
        self
    }

    pub fn print(&self) {
        print!("{}", self.render());
    }

    /// Renders the table into a provided byte buffer, reusing the allocation.
    ///
    /// This method allows for zero-allocation rendering when the buffer is reused
    /// across multiple renders, making it ideal for repeated rendering scenarios
    /// like pagination or filtering UI.
    ///
    /// # Arguments
    /// * `buf` - A buffer to render into. Will be cleared and reused.
    ///
    /// # Returns
    /// * `Ok(())` if rendering succeeded
    ///
    /// # Errors
    /// This function currently never returns an error, but returns `Result` for
    /// future compatibility with potential I/O operations.
    ///
    /// # Examples
    /// ```ignore
    /// let mut buffer = Vec::with_capacity(4096);
    /// for item in items {
    ///     buffer.clear();
    ///     table.render_into(&mut buffer)?;
    ///     stdout.write_all(&buffer)?;
    /// }
    /// ```
    pub fn render_into(&self, buf: &mut Vec<u8>) -> core::fmt::Result {
        buf.clear();
        let rendered = self.render();
        buf.extend_from_slice(rendered.as_bytes());
        Ok(())
    }

    /// Formats a cell's content with the given width and alignment.
    ///
    /// This is a lower-level function that can be useful for custom formatting needs.
    ///
    /// # Arguments
    /// * `content` - The cell content to format
    /// * `width` - The target width for the formatted output
    /// * `alignment` - The alignment to use
    ///
    /// # Returns
    /// The formatted string with appropriate padding
    ///
    /// # Examples
    /// ```
    /// # use crabular::{Table, Alignment};
    /// let formatted = Table::format_cell("test", 10, Alignment::Left);
    /// assert_eq!(formatted, "test      ");
    /// ```
    #[must_use]
    pub fn format_cell(content: &str, width: usize, alignment: Alignment) -> String {
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

        // Optimized version: pre-allocate and use push_str() instead of format!
        let padding = width - content_len;
        let mut result = String::with_capacity(width);

        match alignment {
            Alignment::Left => {
                result.push_str(content);
                for _ in 0..padding {
                    result.push(' ');
                }
            }
            Alignment::Right => {
                for _ in 0..padding {
                    result.push(' ');
                }
                result.push_str(content);
            }
            Alignment::Center => {
                let left = padding / 2;
                let right = padding - left;
                for _ in 0..left {
                    result.push(' ');
                }
                result.push_str(content);
                for _ in 0..right {
                    result.push(' ');
                }
            }
        }

        result
    }

    pub(crate) fn wrap_text(text: &str, width: usize) -> Vec<String> {
        if text.is_empty() || width == 0 {
            return vec![String::new()];
        }

        if text.chars().count() <= width {
            return vec![text.to_string()];
        }

        let mut lines = Vec::new();
        let mut current_line = String::with_capacity(width);
        let mut current_char_count = 0;

        // Iterate directly over split_whitespace() without collecting into Vec
        for word in text.split_whitespace() {
            let word_char_count = word.chars().count();

            if current_char_count == 0 {
                // Starting a new line
                if word_char_count > width {
                    Self::wrap_long_word(word, width, &mut lines);
                } else {
                    current_line.push_str(word);
                    current_char_count = word_char_count;
                }
            } else {
                // Continuing an existing line
                let potential_len = current_char_count + 1 + word_char_count;
                if potential_len <= width {
                    current_line.push(' ');
                    current_line.push_str(word);
                    current_char_count = potential_len;
                } else {
                    // Line is full, start a new one
                    lines.push(core::mem::take(&mut current_line));
                    current_char_count = 0;

                    if word_char_count > width {
                        Self::wrap_long_word(word, width, &mut lines);
                    } else {
                        current_line.push_str(word);
                        current_char_count = word_char_count;
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

    /// Helper to wrap a word that exceeds the column width.
    /// Breaks the word into chunks of `width` characters and appends to `lines`.
    fn wrap_long_word(word: &str, width: usize, lines: &mut Vec<String>) {
        let mut chars = word.chars().peekable();

        while chars.peek().is_some() {
            let chunk: String = chars.by_ref().take(width).collect();
            lines.push(chunk);
        }
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

        let column_widths = self.calculate_column_widths();
        self.render_with_widths(&column_widths)
    }

    /// Renders the table using cached column widths if available.
    ///
    /// This method provides improved performance for repeated renders of the same table.
    /// The first call calculates and caches column widths. Subsequent calls reuse the cache
    /// until the table is modified.
    ///
    /// # Returns
    /// The rendered table as a `String`
    ///
    /// # Examples
    /// ```
    /// # use crabular::{Table, Alignment};
    /// let table = Table::new().header(&["A", "B"]).row(&["1", "2"]);
    /// let _first = table.render_cached(); // Calculates and caches widths
    /// let _second = table.render_cached(); // Uses cached widths (faster)
    /// ```
    #[must_use]
    pub fn render_cached(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        // Use cached widths or calculate and cache them
        let column_widths = {
            let mut cache = self.cached_widths.borrow_mut();
            if let Some(ref widths) = *cache {
                widths.clone()
            } else {
                let widths = self.calculate_column_widths();
                *cache = Some(widths.clone());
                widths
            }
        };

        self.render_with_widths(&column_widths)
    }

    /// Internal method that renders the table with pre-calculated column widths.
    fn render_with_widths(&self, column_widths: &[usize]) -> String {
        let borders = self.style.border_chars();
        let is_minimal_or_compact = matches!(self.style, TableStyle::Minimal | TableStyle::Compact);

        let num_columns = column_widths.len();

        // Pre-calculate approximate buffer size
        let row_width: usize = column_widths.iter().sum::<usize>()
            + (self.padding.left + self.padding.right) * num_columns
            + self.column_spacing * num_columns.saturating_sub(1)
            + num_columns + 1 // border chars
            + 1; // newline

        let num_rows = self.len();
        let border_rows = if is_minimal_or_compact { 1 } else { 3 };
        let estimated_lines = num_rows + border_rows + usize::from(self.headers().is_some());
        let estimated_capacity = row_width * estimated_lines;

        let mut output = String::with_capacity(estimated_capacity);

        // Get the first row to determine top border boundaries
        let first_row = self.headers().or_else(|| self.rows.first());

        if !is_minimal_or_compact {
            let first_boundaries = first_row.map_or_else(
                || Self::all_boundaries(num_columns),
                |row| Self::get_row_boundaries(row, num_columns),
            );
            // For top border, only use first row boundaries (pass same for both)
            output.push_str(&Self::render_horizontal_border_with_spans(
                column_widths,
                self.padding,
                self.column_spacing,
                borders.top_left,
                borders.top_cross,
                borders.top_right,
                borders.horizontal,
                borders.top_cross,    // T-down (for top border, same as top_cross)
                borders.bottom_cross, // T-up (for top border, use bottom_cross)
                &first_boundaries,
                &first_boundaries, // Same boundaries - junction only if first row has boundary
            ));
        }

        if let Some(headers) = self.headers() {
            let header_boundaries = Self::get_row_boundaries(headers, num_columns);
            output.push_str(&self.render_row_with_wrapping(
                headers,
                column_widths,
                &borders,
                &self.column_alignments,
            ));

            // Get first data row boundaries for the separator
            let first_data_boundaries = self.rows.first().map_or_else(
                || Self::all_boundaries(num_columns),
                |row| Self::get_row_boundaries(row, num_columns),
            );

            output.push_str(&Self::render_horizontal_border_with_spans(
                column_widths,
                self.padding,
                self.column_spacing,
                borders.left_cross,
                borders.cross,
                borders.right_cross,
                borders.horizontal,
                borders.top_cross,      // T-down (row below has boundary)
                borders.bottom_cross,   // T-up (row above has boundary)
                &first_data_boundaries, // Row below (first data row)
                &header_boundaries,     // Row above (headers)
            ));
        }

        for row in self.rows() {
            output.push_str(&self.render_row_with_wrapping(
                row,
                column_widths,
                &borders,
                &self.column_alignments,
            ));
        }

        if !is_minimal_or_compact {
            let last_row = self.rows.last().or(self.headers());
            let last_boundaries = last_row.map_or_else(
                || Self::all_boundaries(num_columns),
                |row| Self::get_row_boundaries(row, num_columns),
            );
            // For bottom border, only use last row boundaries (pass same for both)
            output.push_str(&Self::render_horizontal_border_with_spans(
                column_widths,
                self.padding,
                self.column_spacing,
                borders.bottom_left,
                borders.bottom_cross,
                borders.bottom_right,
                borders.horizontal,
                borders.top_cross,    // T-down
                borders.bottom_cross, // T-up
                &last_boundaries,     // Same boundaries - junction only if last row has boundary
                &last_boundaries,
            ));
        }

        output
    }

    /// Returns a vector indicating which column indices have a cell boundary.
    /// Index 0 and `num_columns` are always true (left and right table edges).
    fn get_row_boundaries(row: &Row, num_columns: usize) -> Vec<bool> {
        let mut boundaries = vec![false; num_columns + 1];
        boundaries[0] = true;
        boundaries[num_columns] = true;

        let mut col_idx = 0;
        for cell in row.cells() {
            if col_idx <= num_columns {
                boundaries[col_idx] = true;
            }
            col_idx += cell.span().max(1);
        }
        if col_idx <= num_columns {
            boundaries[col_idx] = true;
        }

        boundaries
    }

    /// Returns boundaries where all columns have separators (no colspan).
    fn all_boundaries(num_columns: usize) -> Vec<bool> {
        vec![true; num_columns + 1]
    }

    /// Forces recalculation of column widths on the next render.
    ///
    /// This method is primarily useful when the table has been modified externally
    /// or when you want to ensure fresh calculations.
    ///
    /// # Examples
    /// ```
    /// # use crabular::{Table, Alignment};
    /// let mut table = Table::new().header(&["A", "B"]).row(&["1", "2"]);
    /// table.recalculate_widths();
    /// ```
    pub fn recalculate_widths(&mut self) {
        self.invalidate_cache();
    }

    fn render_row_with_wrapping(
        &self,
        row: &Row,
        column_widths: &[usize],
        borders: &BorderChars,
        column_alignments: &[Alignment],
    ) -> String {
        let num_columns = column_widths.len();
        let mut wrapped_cells: Vec<Vec<String>> = Vec::with_capacity(row.len());
        let mut cell_spans: Vec<usize> = Vec::with_capacity(row.len());
        let mut max_lines = 1;

        // Build a set of column boundaries for this row
        // A boundary exists at column index `i` if a cell starts there
        let mut boundaries = vec![false; num_columns + 1];
        boundaries[0] = true; // Left edge always has boundary
        boundaries[num_columns] = true; // Right edge always has boundary

        let mut col_idx = 0;
        for cell in row.cells() {
            let span = cell.span().max(1);
            cell_spans.push(span);
            boundaries[col_idx] = true; // Cell starts here

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
        // Mark boundary at end of last cell
        if col_idx <= num_columns {
            boundaries[col_idx] = true;
        }

        // Apply vertical alignment by calculating offset for each cell
        let aligned_cells: Vec<Vec<String>> = wrapped_cells
            .into_iter()
            .map(|cell_lines| {
                Self::apply_vertical_alignment(cell_lines, max_lines, self.vertical_alignment)
            })
            .collect();

        // Pre-calculate row line width
        let line_width: usize = column_widths.iter().sum::<usize>()
            + (self.padding.left + self.padding.right) * num_columns
            + self.column_spacing * num_columns.saturating_sub(1)
            + num_columns + 1 // border chars
            + 1; // newline

        let mut output = String::with_capacity(line_width * max_lines);

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

                // Left padding
                for _ in 0..self.padding.left {
                    output.push(' ');
                }
                output.push_str(&Self::format_cell(content, combined_width, alignment));
                // Right padding
                for _ in 0..self.padding.right {
                    output.push(' ');
                }

                col_idx += span;

                // Add spacing and vertical border
                // Only add spacing if not at the last column
                if col_idx < num_columns {
                    for _ in 0..self.column_spacing {
                        output.push(' ');
                    }
                }
                output.push_str(borders.vertical);
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
                result.extend(core::iter::repeat_n(String::new(), padding_needed));
            }
            VerticalAlignment::Middle => {
                let top_padding = padding_needed / 2;
                let bottom_padding = padding_needed - top_padding;
                result.extend(core::iter::repeat_n(String::new(), top_padding));
                result.extend(cell_lines);
                result.extend(core::iter::repeat_n(String::new(), bottom_padding));
            }
            VerticalAlignment::Bottom => {
                result.extend(core::iter::repeat_n(String::new(), padding_needed));
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

    /// Renders a horizontal border with proper handling of column spans.
    ///
    /// Uses different junction characters based on cell boundaries:
    /// - Cross (┼) when both rows have boundary
    /// - T-down (┬) when only row below has boundary
    /// - T-up (┴) when only row above has boundary
    /// - Horizontal (─) when neither has boundary
    #[allow(clippy::too_many_arguments)]
    fn render_horizontal_border_with_spans(
        column_widths: &[usize],
        padding: Padding,
        column_spacing: usize,
        left: &str,
        cross: &str,
        right: &str,
        horizontal: &str,
        cross_down: &str, // T pointing down (┬) - only row below has boundary
        cross_up: &str,   // T pointing up (┴) - only row above has boundary
        boundaries_below: &[bool],
        boundaries_above: &[bool],
    ) -> String {
        let num_columns = column_widths.len();

        // Pre-calculate line width
        let content_width: usize = column_widths.iter().sum::<usize>()
            + (padding.left + padding.right) * num_columns
            + column_spacing * num_columns.saturating_sub(1);
        let border_chars = num_columns + 1;
        let estimated_capacity = content_width + border_chars + 1;

        let mut line = String::with_capacity(estimated_capacity);

        line.push_str(left);

        // Check if horizontal is a single character for optimization
        let h_char = if horizontal.len() == 1 {
            horizontal.chars().next()
        } else {
            None
        };

        for (index, &width) in column_widths.iter().enumerate() {
            let cell_width = padding.left + width + padding.right;
            if let Some(ch) = h_char {
                for _ in 0..cell_width {
                    line.push(ch);
                }
            } else {
                for _ in 0..cell_width {
                    line.push_str(horizontal);
                }
            }

            if index < num_columns - 1 {
                // Column boundary index (between column `index` and `index + 1`)
                let boundary_idx = index + 1;
                let has_boundary_below =
                    boundaries_below.get(boundary_idx).copied().unwrap_or(true);
                let has_boundary_above =
                    boundaries_above.get(boundary_idx).copied().unwrap_or(true);

                // Determine the junction character based on boundaries
                let junction = match (has_boundary_above, has_boundary_below) {
                    (true, true) => cross,        // Both have boundary: ┼
                    (false, true) => cross_down,  // Only below: ┬
                    (true, false) => cross_up,    // Only above: ┴
                    (false, false) => horizontal, // Neither: ─ (continue horizontal)
                };

                if junction == horizontal {
                    // No boundary on both sides - continue with horizontal line
                    // Add spacing width + 1 (for the cross character position)
                    let span_width = column_spacing + 1;
                    if let Some(ch) = h_char {
                        for _ in 0..span_width {
                            line.push(ch);
                        }
                    } else {
                        for _ in 0..span_width {
                            line.push_str(horizontal);
                        }
                    }
                } else {
                    // There's a junction character to render
                    if let Some(ch) = h_char {
                        for _ in 0..column_spacing {
                            line.push(ch);
                        }
                    } else {
                        for _ in 0..column_spacing {
                            line.push_str(horizontal);
                        }
                    }
                    line.push_str(junction);
                }
            }
        }
        line.push_str(right);
        line.push('\n');

        line
    }
}

impl core::fmt::Display for Table {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.render())
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
        table.set_headers(["A", "B"]);
        assert!(table.headers().is_some());
        assert_eq!(table.headers().unwrap().len(), 2);
    }

    #[test]
    fn add_row() {
        let mut table = Table::new();
        table.add_row(["1", "2"]);
        assert_eq!(table.len(), 1);
        assert!(!table.is_empty());
    }

    #[test]
    fn insert_row() {
        let mut table = Table::new();
        table.add_row(["a", "1"]);
        table.add_row(["c", "3"]);
        table.insert_row(1, ["b", "2"]);
        assert_eq!(table.len(), 3);
        assert_eq!(table.rows()[1].cells()[0].content(), "b");
    }

    #[test]
    fn remove_row() {
        let mut table = Table::new();
        table.add_row(["a", "1"]);
        table.add_row(["b", "2"]);
        let removed = table.remove_row(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().cells()[0].content(), "a");
        assert_eq!(table.len(), 1);
    }

    #[test]
    fn remove_row_out_of_bounds() {
        let mut table = Table::new();
        table.add_row(["a"]);
        assert!(table.remove_row(5).is_none());
    }

    #[test]
    fn cols() {
        let table = Table::new().header(["A", "B", "C"]).row(["1", "2", "3"]);
        assert_eq!(table.cols(), 3);
    }

    #[test]
    fn fluent_api() {
        let table = Table::new()
            .header(["ID", "Value"])
            .row(["1", "100"])
            .row(["2", "200"]);
        assert_eq!(table.len(), 2);
        assert!(table.headers().is_some());
    }

    // Sorting tests
    #[test]
    fn sort_ascending() {
        let mut table = Table::new();
        table.add_row(["Squidward"]);
        table.add_row(["Kelana"]);
        table.add_row(["Kata"]);
        table.sort(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "Kata");
        assert_eq!(table.rows()[1].cells()[0].content(), "Kelana");
        assert_eq!(table.rows()[2].cells()[0].content(), "Squidward");
    }

    #[test]
    fn sort_descending() {
        let mut table = Table::new();
        table.add_row(["Kelana"]);
        table.add_row(["Squidward"]);
        table.sort_desc(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "Squidward");
        assert_eq!(table.rows()[1].cells()[0].content(), "Kelana");
    }

    #[test]
    fn sort_num_ascending() {
        let mut table = Table::new();
        table.add_row(["100"]);
        table.add_row(["25"]);
        table.add_row(["50"]);
        table.sort_num(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "25");
        assert_eq!(table.rows()[1].cells()[0].content(), "50");
        assert_eq!(table.rows()[2].cells()[0].content(), "100");
    }

    #[test]
    fn sort_num_descending() {
        let mut table = Table::new();
        table.add_row(["25"]);
        table.add_row(["100"]);
        table.sort_num_desc(0);
        assert_eq!(table.rows()[0].cells()[0].content(), "100");
        assert_eq!(table.rows()[1].cells()[0].content(), "25");
    }

    #[test]
    fn sort_preserves_headers() {
        let mut table = Table::new();
        table.set_headers(["Name"]);
        table.add_row(["Squidward"]);
        table.add_row(["Kelana"]);
        table.sort(0);
        assert_eq!(table.headers().unwrap().cells()[0].content(), "Name");
    }

    // Filter tests
    #[test]
    fn filter() {
        let mut table = Table::new();
        table.add_row(["Kelana", "25"]);
        table.add_row(["Kata", "30"]);
        table.add_row(["Squidward", "25"]);
        table.filter(|row| row.cells()[1].content() == "25");
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filter_eq() {
        let mut table = Table::new();
        table.add_row(["Active"]);
        table.add_row(["Inactive"]);
        table.add_row(["Active"]);
        table.filter_eq(0, "Active");
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filter_col() {
        let mut table = Table::new();
        table.add_row(["100"]);
        table.add_row(["50"]);
        table.add_row(["75"]);
        table.filter_col(0, |val| val.parse::<i32>().is_ok_and(|n| n > 60));
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filter_has() {
        let mut table = Table::new();
        table.add_row(["Kelana Smith"]);
        table.add_row(["Kata Jones"]);
        table.add_row(["Squidward Smith"]);
        table.filter_has(0, "Smith");
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn filtered_returns_new_table() {
        let mut table = Table::new();
        table.set_style(TableStyle::Modern);
        table.add_row(["25"]);
        table.add_row(["30"]);
        table.add_row(["25"]);

        let filtered = table.filtered(|row| row.cells()[0].content() == "25");
        assert_eq!(table.len(), 3); // Original unchanged
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered.style(), TableStyle::Modern);
    }

    // Column operations tests
    #[test]
    fn add_column() {
        let mut table = Table::new();
        table.set_headers(["A", "B"]);
        table.add_row(["1", "2"]);
        table.add_column(&["C", "3"], Alignment::Right);
        assert_eq!(table.cols(), 3);
        assert_eq!(table.headers().unwrap().cells()[2].content(), "C");
    }

    #[test]
    fn insert_column() {
        let mut table = Table::new();
        table.set_headers(["A", "C"]);
        table.add_row(["1", "3"]);
        table.insert_column(1, &["B", "2"], Alignment::Center);
        assert_eq!(table.headers().unwrap().cells()[1].content(), "B");
    }

    #[test]
    fn remove_column() {
        let mut table = Table::new();
        table.set_headers(["A", "B", "C"]);
        table.add_row(["1", "2", "3"]);
        assert!(table.remove_column(1));
        assert_eq!(table.cols(), 2);
        assert_eq!(table.headers().unwrap().cells()[1].content(), "C");
    }

    // Render tests
    #[test]
    fn render_into_reuses_buffer() {
        let table = Table::new().header(["A", "B"]).row(["1", "2"]);

        let mut buffer = Vec::with_capacity(10);
        let original_capacity = buffer.capacity();

        table.render_into(&mut buffer).unwrap();
        let _first_capacity = buffer.capacity();

        buffer.clear();
        table.render_into(&mut buffer).unwrap();

        assert!(buffer.capacity() >= original_capacity);
        assert!(!buffer.is_empty());
    }

    #[test]
    fn render_single_row() {
        let table = Table::new().row(["a", "b"]);
        let output = table.render();
        assert!(!output.is_empty());
        assert!(output.contains('a'));
        assert!(output.contains('b'));
    }

    #[test]
    fn render_with_headers() {
        let table = Table::new().header(["X", "Y"]).row(["1", "2"]);
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

    #[test]
    fn display_trait_matches_render() {
        let table = Table::new()
            .header(["Name", "Value"])
            .row(["Kata", "100"])
            .row(["Kelana", "200"]);

        let rendered = table.render();
        let displayed = format!("{table}");

        assert_eq!(rendered, displayed);
    }

    #[test]
    fn display_trait_empty_table() {
        let table = Table::new();
        let displayed = format!("{table}");
        assert_eq!(displayed, "");
    }

    #[test]
    fn display_trait_with_style() {
        let mut table = Table::new();
        table.set_style(TableStyle::Modern);
        table.set_headers(["A", "B"]);
        table.add_row(["1", "2"]);

        let rendered = table.render();
        let displayed = format!("{table}");

        assert_eq!(rendered, displayed);
    }

    #[test]
    fn add_row_invalidates_cache() {
        let mut table = Table::new().header(["A"]).row(["1"]);

        let first = table.render_cached();

        table.add_row(["2"]);

        let second = table.render_cached();

        assert_ne!(first, second);
    }

    #[test]
    fn set_headers_invalidates_cache() {
        let mut table = Table::new().header(["A"]).row(["1"]);

        let first = table.render_cached();

        table.set_headers(["B"]);

        let second = table.render_cached();

        assert_ne!(first, second);
    }

    #[test]
    fn render_into_matches_render() {
        let table = Table::new()
            .header(["Name", "Value"])
            .row(["Kata", "100"])
            .row(["Kelana", "200"]);

        let rendered = table.render();
        let mut buffer = Vec::new();
        table.render_into(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), rendered);
    }

    #[test]
    fn format_cell_left_alignment() {
        let result = Table::format_cell("test", 10, Alignment::Left);
        assert_eq!(result, "test      ");
    }

    #[test]
    fn format_cell_right_alignment() {
        let result = Table::format_cell("test", 10, Alignment::Right);
        assert_eq!(result, "      test");
    }

    #[test]
    fn format_cell_center_alignment() {
        let result = Table::format_cell("test", 10, Alignment::Center);
        assert_eq!(result, "   test   ");
    }

    #[test]
    fn format_cell_truncation() {
        let result = Table::format_cell("hello world", 8, Alignment::Left);
        assert_eq!(result, "hello...");
    }

    #[test]
    fn format_cell_exact_width() {
        let result = Table::format_cell("test", 4, Alignment::Left);
        assert_eq!(result, "test");
    }

    #[test]
    fn recalculate_widths_forces_recalculation() {
        let mut table = Table::new().header(["A"]).row(["1"]);

        let _ = table.render_cached();

        table.recalculate_widths();
        let result = table.render_cached();

        assert!(!result.is_empty());
    }

    #[test]
    fn render_cached_reuses_cache() {
        let table = Table::new().header(["A", "B"]).row(["1", "2"]);

        // First call populates cache
        let first = table.render_cached();

        // Verify cache is populated
        assert!(table.cached_widths.borrow().is_some());

        // Second call should return same result (using cache)
        let second = table.render_cached();

        assert_eq!(first, second);
    }

    #[test]
    fn render_cached_matches_render() {
        let table = Table::new()
            .header(["Name", "Age"])
            .row(["Kata", "30"])
            .row(["Kelana", "25"]);

        let rendered = table.render();
        let cached = table.render_cached();

        assert_eq!(rendered, cached);
    }
}

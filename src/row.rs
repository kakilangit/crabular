use crate::Alignment;
use crate::cell::Cell;

#[derive(Clone)]
pub struct Row {
    cells: Vec<Cell>,
}

impl Row {
    #[must_use]
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    #[must_use]
    pub fn from(contents: &[&str], default_alignment: Alignment) -> Self {
        let cells = contents
            .iter()
            .map(|&content| Cell::new(content, default_alignment))
            .collect();
        Self { cells }
    }

    pub fn push(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    /// Inserts a cell at the specified index.
    ///
    /// # Panics
    /// Panics if `index > self.len()`.
    pub fn insert(&mut self, index: usize, cell: Cell) {
        self.cells.insert(index, cell);
    }

    /// Removes and returns the cell at the specified index.
    /// Returns `None` if index is out of bounds.
    pub fn remove(&mut self, index: usize) -> Option<Cell> {
        if index < self.cells.len() {
            Some(self.cells.remove(index))
        } else {
            None
        }
    }

    /// Returns a mutable reference to the cell at the specified index.
    pub fn cell_mut(&mut self, index: usize) -> Option<&mut Cell> {
        self.cells.get_mut(index)
    }

    #[must_use]
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Attempts to convert the row's cells into a fixed-size array.
    ///
    /// This method provides zero-overhead access to rows with a known number of columns,
    /// eliminating bounds checking in hot paths.
    ///
    /// # Arguments
    /// * `N` - The expected number of columns in the row
    ///
    /// # Returns
    /// * `Some(&[Cell; N])` if the row has exactly N cells
    /// * `None` if the row has a different number of cells
    ///
    /// # Examples
    /// ```
    /// use crabular::{Alignment, Row};
    ///
    /// let row = Row::from(&["a", "b", "c"], Alignment::Left);
    /// if let Some(array) = row.as_array::<3>() {
    ///     // Stack-allocated, no bounds checking needed
    ///     assert_eq!(array[0].content(), "a");
    /// }
    /// ```
    #[must_use]
    pub fn as_array<const N: usize>(&self) -> Option<&[Cell; N]> {
        self.cells.as_array()
    }
}

impl core::fmt::Display for Row {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            if i > 0 {
                write!(f, " | ")?;
            }
            write!(f, "{}", cell.content())?;
        }
        Ok(())
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let row = Row::new();
        assert!(row.is_empty());
        assert_eq!(row.len(), 0);
    }

    #[test]
    fn default_is_empty() {
        let row = Row::default();
        assert!(row.is_empty());
    }

    #[test]
    fn from_strings() {
        let cases = [
            (vec!["a"], 1),
            (vec!["a", "b"], 2),
            (vec!["a", "b", "c"], 3),
        ];
        for (contents, expected_len) in cases {
            let row = Row::from(&contents, Alignment::Left);
            assert_eq!(row.len(), expected_len);
            for (i, &content) in contents.iter().enumerate() {
                assert_eq!(row.cells()[i].content(), content);
            }
        }
    }

    #[test]
    fn push() {
        let mut row = Row::new();
        row.push(Cell::new("a", Alignment::Left));
        row.push(Cell::new("b", Alignment::Right));
        assert_eq!(row.len(), 2);
        assert_eq!(row.cells()[0].content(), "a");
        assert_eq!(row.cells()[1].content(), "b");
    }

    #[test]
    fn insert() {
        let mut row = Row::from(&["a", "c"], Alignment::Left);
        row.insert(1, Cell::new("b", Alignment::Left));
        assert_eq!(row.len(), 3);
        assert_eq!(row.cells()[0].content(), "a");
        assert_eq!(row.cells()[1].content(), "b");
        assert_eq!(row.cells()[2].content(), "c");
    }

    #[test]
    fn remove() {
        let mut row = Row::from(&["a", "b", "c"], Alignment::Left);
        let removed = row.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().content(), "b");
        assert_eq!(row.len(), 2);
    }

    #[test]
    fn remove_out_of_bounds() {
        let mut row = Row::from(&["a"], Alignment::Left);
        assert!(row.remove(5).is_none());
        assert_eq!(row.len(), 1);
    }

    #[test]
    fn cell_mut() {
        let mut row = Row::from(&["a", "b"], Alignment::Left);
        if let Some(cell) = row.cell_mut(0) {
            cell.set_alignment(Alignment::Right);
        }
        assert_eq!(row.cells()[0].alignment(), Alignment::Right);
    }

    #[test]
    fn clone_trait() {
        let row = Row::from(&["a", "b"], Alignment::Left);
        let cloned = row.clone();
        assert_eq!(row.len(), cloned.len());
        for (orig, copy) in row.cells().iter().zip(cloned.cells().iter()) {
            assert_eq!(orig.content(), copy.content());
        }
    }

    #[test]
    fn display_trait_empty_row() {
        let row = Row::new();
        let displayed = format!("{row}");
        assert_eq!(displayed, "");
    }

    #[test]
    fn display_trait_single_cell() {
        let row = Row::from(&["hello"], Alignment::Left);
        let displayed = format!("{row}");
        assert_eq!(displayed, "hello");
    }

    #[test]
    fn display_trait_multiple_cells() {
        let row = Row::from(&["a", "b", "c"], Alignment::Left);
        let displayed = format!("{row}");
        assert_eq!(displayed, "a | b | c");
    }

    #[test]
    fn as_array_matching_size() {
        let row = Row::from(&["a", "b", "c"], Alignment::Left);
        let array = row.as_array::<3>();
        assert!(array.is_some());
        assert_eq!(array.unwrap()[0].content(), "a");
        assert_eq!(array.unwrap()[1].content(), "b");
        assert_eq!(array.unwrap()[2].content(), "c");
    }

    #[test]
    fn as_array_wrong_size() {
        let row = Row::from(&["a", "b", "c"], Alignment::Left);
        assert!(row.as_array::<2>().is_none());
        assert!(row.as_array::<4>().is_none());
    }

    #[test]
    fn as_array_empty_row() {
        let row = Row::new();
        assert!(row.as_array::<0>().is_some());
        assert!(row.as_array::<1>().is_none());
    }
}

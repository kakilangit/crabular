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
    pub fn with_alignment<I, S>(contents: I, alignment: Alignment) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let cells = contents
            .into_iter()
            .map(|s| Cell::new(s.as_ref(), alignment))
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
    /// let row = Row::with_alignment(&["a", "b", "c"], Alignment::Left);
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

impl<S: AsRef<str>> From<&[S]> for Row {
    fn from(contents: &[S]) -> Self {
        Self::with_alignment(contents, Alignment::default())
    }
}

impl<S: AsRef<str>> From<Vec<S>> for Row {
    fn from(contents: Vec<S>) -> Self {
        Self::with_alignment(contents, Alignment::default())
    }
}

impl<S: AsRef<str>, const N: usize> From<[S; N]> for Row {
    fn from(contents: [S; N]) -> Self {
        Self::with_alignment(contents, Alignment::default())
    }
}

impl<S: AsRef<str>, const N: usize> From<&[S; N]> for Row {
    fn from(contents: &[S; N]) -> Self {
        Self::with_alignment(contents, Alignment::default())
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
        let row1: Row = ["a"].into();
        assert_eq!(row1.len(), 1);
        assert_eq!(row1.cells()[0].content(), "a");

        let row2: Row = ["a", "b"].into();
        assert_eq!(row2.len(), 2);
        assert_eq!(row2.cells()[0].content(), "a");
        assert_eq!(row2.cells()[1].content(), "b");

        let row3: Row = ["a", "b", "c"].into();
        assert_eq!(row3.len(), 3);
        assert_eq!(row3.cells()[0].content(), "a");
        assert_eq!(row3.cells()[1].content(), "b");
        assert_eq!(row3.cells()[2].content(), "c");
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
        let mut row: Row = ["a", "c"].into();
        row.insert(1, Cell::new("b", Alignment::Left));
        assert_eq!(row.len(), 3);
        assert_eq!(row.cells()[0].content(), "a");
        assert_eq!(row.cells()[1].content(), "b");
        assert_eq!(row.cells()[2].content(), "c");
    }

    #[test]
    fn remove() {
        let mut row: Row = ["a", "b", "c"].into();
        let removed = row.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().content(), "b");
        assert_eq!(row.len(), 2);
    }

    #[test]
    fn remove_out_of_bounds() {
        let mut row: Row = ["a"].into();
        assert!(row.remove(5).is_none());
        assert_eq!(row.len(), 1);
    }

    #[test]
    fn cell_mut() {
        let mut row: Row = ["a", "b"].into();
        if let Some(cell) = row.cell_mut(0) {
            cell.set_alignment(Alignment::Right);
        }
        assert_eq!(row.cells()[0].alignment(), Alignment::Right);
    }

    #[test]
    fn clone_trait() {
        let row: Row = ["a", "b"].into();
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
        let row: Row = ["hello"].into();
        let displayed = format!("{row}");
        assert_eq!(displayed, "hello");
    }

    #[test]
    fn display_trait_multiple_cells() {
        let row: Row = ["a", "b", "c"].into();
        let displayed = format!("{row}");
        assert_eq!(displayed, "a | b | c");
    }

    #[test]
    fn as_array_matching_size() {
        let row: Row = ["a", "b", "c"].into();
        let array = row.as_array::<3>();
        assert!(array.is_some());
        assert_eq!(array.unwrap()[0].content(), "a");
        assert_eq!(array.unwrap()[1].content(), "b");
        assert_eq!(array.unwrap()[2].content(), "c");
    }

    #[test]
    fn as_array_wrong_size() {
        let row: Row = ["a", "b", "c"].into();
        assert!(row.as_array::<2>().is_none());
        assert!(row.as_array::<4>().is_none());
    }

    #[test]
    fn as_array_empty_row() {
        let row = Row::new();
        assert!(row.as_array::<0>().is_some());
        assert!(row.as_array::<1>().is_none());
    }

    #[test]
    fn from_array() {
        let row: Row = ["a", "b", "c"].into();
        assert_eq!(row.len(), 3);
        assert_eq!(row.cells()[0].content(), "a");
        assert_eq!(row.cells()[1].content(), "b");
        assert_eq!(row.cells()[2].content(), "c");
    }

    #[test]
    fn from_array_ref() {
        let row: Row = (&["a", "b"]).into();
        assert_eq!(row.len(), 2);
        assert_eq!(row.cells()[0].content(), "a");
    }

    #[test]
    fn from_vec() {
        let row: Row = vec!["x", "y"].into();
        assert_eq!(row.len(), 2);
        assert_eq!(row.cells()[0].content(), "x");
        assert_eq!(row.cells()[1].content(), "y");
    }

    #[test]
    fn from_vec_string() {
        let row: Row = vec!["hello".to_string(), "world".to_string()].into();
        assert_eq!(row.len(), 2);
        assert_eq!(row.cells()[0].content(), "hello");
        assert_eq!(row.cells()[1].content(), "world");
    }

    #[test]
    fn from_slice() {
        let data = ["a", "b", "c"];
        let row: Row = data.as_slice().into();
        assert_eq!(row.len(), 3);
    }
}

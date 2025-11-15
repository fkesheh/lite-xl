//! Terminal grid implementation.
//!
//! This module provides the [`Grid`] type which represents a 2D grid of terminal
//! cells with a fixed number of rows and columns.

use super::cell::Cell;
use serde::{Deserialize, Serialize};

/// A 2D grid of terminal cells.
///
/// The grid has a fixed size (rows × columns) and stores cells in row-major order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid {
    /// Number of rows in the grid.
    rows: usize,
    /// Number of columns in the grid.
    cols: usize,
    /// The cells stored in row-major order.
    cells: Vec<Cell>,
}

impl Grid {
    /// Creates a new grid with the specified dimensions.
    ///
    /// All cells are initialized to empty (space with default colors).
    ///
    /// # Panics
    /// Panics if rows or cols is 0.
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0, "Grid must have at least 1 row");
        assert!(cols > 0, "Grid must have at least 1 column");

        let cells = vec![Cell::default(); rows * cols];
        Self { rows, cols, cells }
    }

    /// Returns the number of rows in the grid.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the grid.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the dimensions of the grid as (rows, cols).
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Calculates the linear index for the given row and column.
    ///
    /// Returns None if the position is out of bounds.
    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    /// Gets a reference to the cell at the specified position.
    ///
    /// Returns None if the position is out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.index(row, col).map(|i| &self.cells[i])
    }

    /// Gets a mutable reference to the cell at the specified position.
    ///
    /// Returns None if the position is out of bounds.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.index(row, col).map(|i| &mut self.cells[i])
    }

    /// Sets the cell at the specified position.
    ///
    /// Returns true if the cell was set, false if the position is out of bounds.
    pub fn set(&mut self, row: usize, col: usize, cell: Cell) -> bool {
        if let Some(i) = self.index(row, col) {
            self.cells[i] = cell;
            true
        } else {
            false
        }
    }

    /// Sets the character at the specified position, preserving other attributes.
    ///
    /// Returns true if the character was set, false if the position is out of bounds.
    pub fn set_char(&mut self, row: usize, col: usize, c: char) -> bool {
        if let Some(cell) = self.get_mut(row, col) {
            cell.set_char(c);
            true
        } else {
            false
        }
    }

    /// Clears the cell at the specified position (sets it to a space).
    pub fn clear_cell(&mut self, row: usize, col: usize) {
        if let Some(cell) = self.get_mut(row, col) {
            cell.clear();
        }
    }

    /// Clears all cells in the grid (sets them all to spaces).
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    /// Clears a specific row.
    pub fn clear_row(&mut self, row: usize) {
        if row < self.rows {
            let start = row * self.cols;
            let end = start + self.cols;
            for cell in &mut self.cells[start..end] {
                cell.clear();
            }
        }
    }

    /// Clears rows from `start` to `end` (inclusive).
    pub fn clear_rows(&mut self, start: usize, end: usize) {
        let start = start.min(self.rows);
        let end = end.min(self.rows);
        for row in start..=end {
            self.clear_row(row);
        }
    }

    /// Gets an iterator over all cells in the specified row.
    pub fn row(&self, row: usize) -> Option<&[Cell]> {
        if row < self.rows {
            let start = row * self.cols;
            let end = start + self.cols;
            Some(&self.cells[start..end])
        } else {
            None
        }
    }

    /// Gets a mutable iterator over all cells in the specified row.
    pub fn row_mut(&mut self, row: usize) -> Option<&mut [Cell]> {
        if row < self.rows {
            let start = row * self.cols;
            let end = start + self.cols;
            Some(&mut self.cells[start..end])
        } else {
            None
        }
    }

    /// Scrolls the grid up by one line.
    ///
    /// The top line is removed, all other lines move up, and a new blank line
    /// is added at the bottom. Returns the removed top line.
    pub fn scroll_up(&mut self) -> Vec<Cell> {
        let cols = self.cols;
        let removed = self.cells.drain(0..cols).collect();
        self.cells.extend(vec![Cell::default(); cols]);
        removed
    }

    /// Scrolls the grid down by one line.
    ///
    /// The bottom line is removed, all other lines move down, and a new blank
    /// line is added at the top.
    pub fn scroll_down(&mut self) {
        let cols = self.cols;
        self.cells.truncate(self.cells.len() - cols);
        self.cells.splice(0..0, vec![Cell::default(); cols]);
    }

    /// Scrolls a region up by one line.
    ///
    /// Lines from `start` to `end` (inclusive) are scrolled up. The top line
    /// of the region is removed and a blank line is inserted at the bottom.
    pub fn scroll_region_up(&mut self, start: usize, end: usize) -> Vec<Cell> {
        if start >= self.rows || end >= self.rows || start > end {
            return Vec::new();
        }

        let start_idx = start * self.cols;
        let end_idx = (end + 1) * self.cols;
        let removed_end = start_idx + self.cols;

        let removed: Vec<Cell> = self.cells.drain(start_idx..removed_end).collect();
        self.cells
            .splice(end_idx - self.cols..end_idx - self.cols, vec![Cell::default(); self.cols]);
        removed
    }

    /// Scrolls a region down by one line.
    ///
    /// Lines from `start` to `end` (inclusive) are scrolled down. The bottom
    /// line of the region is removed and a blank line is inserted at the top.
    pub fn scroll_region_down(&mut self, start: usize, end: usize) {
        if start >= self.rows || end >= self.rows || start > end {
            return;
        }

        let start_idx = start * self.cols;
        let end_idx = (end + 1) * self.cols;

        self.cells.drain((end_idx - self.cols)..end_idx);
        self.cells.splice(start_idx..start_idx, vec![Cell::default(); self.cols]);
    }

    /// Resizes the grid to the new dimensions.
    ///
    /// If the grid grows, new cells are initialized to empty.
    /// If the grid shrinks, cells are removed from the bottom and right.
    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        if new_rows == self.rows && new_cols == self.cols {
            return;
        }

        let mut new_cells = vec![Cell::default(); new_rows * new_cols];

        // Copy existing cells
        let copy_rows = self.rows.min(new_rows);
        let copy_cols = self.cols.min(new_cols);

        for row in 0..copy_rows {
            let old_start = row * self.cols;
            let old_end = old_start + copy_cols;
            let new_start = row * new_cols;
            let new_end = new_start + copy_cols;
            new_cells[new_start..new_end].copy_from_slice(&self.cells[old_start..old_end]);
        }

        self.rows = new_rows;
        self.cols = new_cols;
        self.cells = new_cells;
    }

    /// Fills the grid with the given cell.
    pub fn fill(&mut self, cell: Cell) {
        self.cells.fill(cell);
    }

    /// Fills a region of the grid with the given cell.
    pub fn fill_region(&mut self, start_row: usize, start_col: usize, end_row: usize, end_col: usize, cell: Cell) {
        for row in start_row..=end_row.min(self.rows - 1) {
            for col in start_col..=end_col.min(self.cols - 1) {
                if let Some(c) = self.get_mut(row, col) {
                    *c = cell.clone();
                }
            }
        }
    }

    /// Returns an iterator over all cells in the grid.
    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    /// Returns a mutable iterator over all cells in the grid.
    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut()
    }

    /// Converts the grid to a string representation.
    ///
    /// Each row is separated by a newline character.
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.rows * (self.cols + 1));
        for row in 0..self.rows {
            if let Some(cells) = self.row(row) {
                for cell in cells {
                    result.push(cell.c);
                }
                if row < self.rows - 1 {
                    result.push('\n');
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(24, 80);
        assert_eq!(grid.rows(), 24);
        assert_eq!(grid.cols(), 80);
        assert_eq!(grid.size(), (24, 80));
    }

    #[test]
    #[should_panic]
    fn test_grid_new_zero_rows() {
        Grid::new(0, 80);
    }

    #[test]
    #[should_panic]
    fn test_grid_new_zero_cols() {
        Grid::new(24, 0);
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(3, 3);
        let cell = Cell::new('X');

        assert!(grid.set(1, 1, cell.clone()));
        assert_eq!(grid.get(1, 1), Some(&cell));
        assert_eq!(grid.get(10, 10), None);
    }

    #[test]
    fn test_grid_clear() {
        let mut grid = Grid::new(2, 2);
        grid.set(0, 0, Cell::new('A'));
        grid.set(1, 1, Cell::new('B'));

        grid.clear();

        assert_eq!(grid.get(0, 0).unwrap().c, ' ');
        assert_eq!(grid.get(1, 1).unwrap().c, ' ');
    }

    #[test]
    fn test_grid_clear_row() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, Cell::new('A'));
        grid.set(1, 0, Cell::new('B'));
        grid.set(1, 1, Cell::new('C'));

        grid.clear_row(1);

        assert_eq!(grid.get(0, 0).unwrap().c, 'A');
        assert_eq!(grid.get(1, 0).unwrap().c, ' ');
        assert_eq!(grid.get(1, 1).unwrap().c, ' ');
    }

    #[test]
    fn test_grid_scroll_up() {
        let mut grid = Grid::new(3, 2);
        grid.set(0, 0, Cell::new('A'));
        grid.set(1, 0, Cell::new('B'));
        grid.set(2, 0, Cell::new('C'));

        let removed = grid.scroll_up();
        assert_eq!(removed[0].c, 'A');
        assert_eq!(grid.get(0, 0).unwrap().c, 'B');
        assert_eq!(grid.get(1, 0).unwrap().c, 'C');
        assert_eq!(grid.get(2, 0).unwrap().c, ' ');
    }

    #[test]
    fn test_grid_resize_grow() {
        let mut grid = Grid::new(2, 2);
        grid.set(0, 0, Cell::new('A'));
        grid.set(1, 1, Cell::new('B'));

        grid.resize(3, 3);

        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 3);
        assert_eq!(grid.get(0, 0).unwrap().c, 'A');
        assert_eq!(grid.get(1, 1).unwrap().c, 'B');
        assert_eq!(grid.get(2, 2).unwrap().c, ' ');
    }

    #[test]
    fn test_grid_resize_shrink() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, Cell::new('A'));
        grid.set(1, 1, Cell::new('B'));
        grid.set(2, 2, Cell::new('C'));

        grid.resize(2, 2);

        assert_eq!(grid.rows(), 2);
        assert_eq!(grid.cols(), 2);
        assert_eq!(grid.get(0, 0).unwrap().c, 'A');
        assert_eq!(grid.get(1, 1).unwrap().c, 'B');
        assert_eq!(grid.get(2, 2), None); // Out of bounds
    }

    #[test]
    fn test_grid_to_string() {
        let mut grid = Grid::new(2, 3);
        grid.set(0, 0, Cell::new('A'));
        grid.set(0, 1, Cell::new('B'));
        grid.set(0, 2, Cell::new('C'));
        grid.set(1, 0, Cell::new('D'));
        grid.set(1, 1, Cell::new('E'));
        grid.set(1, 2, Cell::new('F'));

        assert_eq!(grid.to_string(), "ABC\nDEF");
    }
}

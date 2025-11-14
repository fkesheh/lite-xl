//! Multi-cursor selection system.
//!
//! This module provides selection and cursor management with full support for
//! multiple simultaneous selections (multi-cursor editing).

use crate::buffer::{Position, Range};
use serde::{Deserialize, Serialize};

/// A selection with anchor and cursor positions.
///
/// A selection represents a region of text with two endpoints:
/// - `anchor`: The fixed point where the selection started
/// - `cursor`: The moving point (head) that extends the selection
///
/// When anchor == cursor, the selection is just a cursor (zero-width).
/// The selection's range is always normalized (start <= end).
///
/// # Examples
///
/// ```
/// use lite_xl::document::Selection;
/// use lite_xl::buffer::Position;
///
/// // Create a cursor
/// let cursor = Selection::cursor(Position::new(5, 10));
/// assert!(cursor.is_cursor());
///
/// // Create a selection
/// let sel = Selection::new(Position::new(0, 0), Position::new(0, 10));
/// assert!(!sel.is_cursor());
/// assert_eq!(sel.range().start, Position::new(0, 0));
/// assert_eq!(sel.range().end, Position::new(0, 10));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    /// The anchor point (where selection started)
    anchor: Position,
    /// The cursor position (active end, where it currently is)
    cursor: Position,
}

impl Selection {
    /// Create a new selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selection;
    /// use lite_xl::buffer::Position;
    ///
    /// let sel = Selection::new(Position::new(0, 0), Position::new(0, 5));
    /// ```
    pub fn new(anchor: Position, cursor: Position) -> Self {
        Self { anchor, cursor }
    }

    /// Create a zero-width selection (cursor).
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selection;
    /// use lite_xl::buffer::Position;
    ///
    /// let cursor = Selection::cursor(Position::new(5, 10));
    /// assert_eq!(cursor.anchor(), Position::new(5, 10));
    /// assert_eq!(cursor.head(), Position::new(5, 10));
    /// ```
    pub fn cursor(pos: Position) -> Self {
        Self::new(pos, pos)
    }

    /// Get the anchor position.
    #[inline]
    pub fn anchor(&self) -> Position {
        self.anchor
    }

    /// Get the head position (same as cursor).
    ///
    /// This is the active, moving end of the selection.
    #[inline]
    pub fn head(&self) -> Position {
        self.cursor
    }

    /// Get the tail position (same as anchor).
    ///
    /// This is the fixed end of the selection.
    #[inline]
    pub fn tail(&self) -> Position {
        self.anchor
    }

    /// Get the selection as a range.
    ///
    /// The range is normalized (start <= end).
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selection;
    /// use lite_xl::buffer::Position;
    ///
    /// let sel = Selection::new(Position::new(0, 10), Position::new(0, 0));
    /// let range = sel.range();
    /// assert_eq!(range.start, Position::new(0, 0));
    /// assert_eq!(range.end, Position::new(0, 10));
    /// ```
    pub fn range(&self) -> Range {
        Range::new(self.anchor, self.cursor)
    }

    /// Check if this is a cursor (zero-width selection).
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selection;
    /// use lite_xl::buffer::Position;
    ///
    /// let cursor = Selection::cursor(Position::new(5, 10));
    /// assert!(cursor.is_cursor());
    ///
    /// let sel = Selection::new(Position::new(0, 0), Position::new(0, 5));
    /// assert!(!sel.is_cursor());
    /// ```
    #[inline]
    pub fn is_cursor(&self) -> bool {
        self.anchor == self.cursor
    }

    /// Check if the selection is reversed (cursor < anchor).
    #[inline]
    pub fn is_reversed(&self) -> bool {
        self.cursor < self.anchor
    }

    /// Flip the anchor and cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selection;
    /// use lite_xl::buffer::Position;
    ///
    /// let sel = Selection::new(Position::new(0, 0), Position::new(0, 5));
    /// let flipped = sel.flip();
    /// assert_eq!(flipped.anchor(), Position::new(0, 5));
    /// assert_eq!(flipped.head(), Position::new(0, 0));
    /// ```
    pub fn flip(self) -> Self {
        Self::new(self.cursor, self.anchor)
    }

    /// Move the cursor to a new position, keeping the anchor fixed.
    ///
    /// This extends or shrinks the selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selection;
    /// use lite_xl::buffer::Position;
    ///
    /// let mut sel = Selection::cursor(Position::new(0, 0));
    /// sel.extend_to(Position::new(0, 10));
    /// assert!(!sel.is_cursor());
    /// assert_eq!(sel.range().end, Position::new(0, 10));
    /// ```
    pub fn extend_to(&mut self, pos: Position) {
        self.cursor = pos;
    }

    /// Move both anchor and cursor to a position (collapse to cursor).
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selection;
    /// use lite_xl::buffer::Position;
    ///
    /// let mut sel = Selection::new(Position::new(0, 0), Position::new(0, 10));
    /// sel.move_to(Position::new(5, 5));
    /// assert!(sel.is_cursor());
    /// assert_eq!(sel.head(), Position::new(5, 5));
    /// ```
    pub fn move_to(&mut self, pos: Position) {
        self.anchor = pos;
        self.cursor = pos;
    }

    /// Collapse the selection to the cursor position.
    pub fn collapse_to_cursor(&mut self) {
        self.anchor = self.cursor;
    }

    /// Collapse the selection to the anchor position.
    pub fn collapse_to_anchor(&mut self) {
        self.cursor = self.anchor;
    }

    /// Check if this selection overlaps with another.
    pub fn overlaps(&self, other: &Selection) -> bool {
        self.range().overlaps(other.range())
    }

    /// Merge this selection with another.
    ///
    /// The anchor will be from the leftmost position, and the cursor
    /// from the rightmost position.
    pub fn merge(&self, other: &Selection) -> Selection {
        let merged_range = self.range().union(other.range());
        Selection::new(merged_range.start, merged_range.end)
    }
}

/// A collection of selections (multi-cursor support).
///
/// Maintains multiple selections with one designated as primary.
/// Selections are automatically sorted and can be merged to eliminate overlaps.
///
/// # Examples
///
/// ```
/// use lite_xl::document::Selections;
/// use lite_xl::buffer::Position;
///
/// let mut selections = Selections::single(Position::new(0, 0));
/// assert_eq!(selections.len(), 1);
///
/// selections.add_cursor(Position::new(1, 0));
/// assert_eq!(selections.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Selections {
    /// All selections, sorted by position
    selections: Vec<Selection>,
    /// Index of the primary selection
    primary_idx: usize,
}

impl Selections {
    /// Create a selections collection with a single cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selections;
    /// use lite_xl::buffer::Position;
    ///
    /// let selections = Selections::single(Position::new(0, 0));
    /// assert_eq!(selections.len(), 1);
    /// ```
    pub fn single(pos: Position) -> Self {
        Self {
            selections: vec![Selection::cursor(pos)],
            primary_idx: 0,
        }
    }

    /// Create selections from a single selection.
    pub fn from_selection(selection: Selection) -> Self {
        Self {
            selections: vec![selection],
            primary_idx: 0,
        }
    }

    /// Create selections from a vector of selections.
    ///
    /// The first selection is designated as primary.
    pub fn from_vec(mut selections: Vec<Selection>) -> Self {
        if selections.is_empty() {
            selections.push(Selection::cursor(Position::zero()));
        }
        Self {
            selections,
            primary_idx: 0,
        }
    }

    /// Get the number of selections.
    #[inline]
    pub fn len(&self) -> usize {
        self.selections.len()
    }

    /// Check if there are no selections.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.selections.is_empty()
    }

    /// Get the primary selection.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selections;
    /// use lite_xl::buffer::Position;
    ///
    /// let selections = Selections::single(Position::new(5, 10));
    /// assert_eq!(selections.primary().head(), Position::new(5, 10));
    /// ```
    #[inline]
    pub fn primary(&self) -> &Selection {
        &self.selections[self.primary_idx]
    }

    /// Get a mutable reference to the primary selection.
    #[inline]
    pub fn primary_mut(&mut self) -> &mut Selection {
        &mut self.selections[self.primary_idx]
    }

    /// Get the index of the primary selection.
    #[inline]
    pub fn primary_index(&self) -> usize {
        self.primary_idx
    }

    /// Set the primary selection by index.
    ///
    /// If the index is out of bounds, this is a no-op.
    pub fn set_primary(&mut self, idx: usize) {
        if idx < self.selections.len() {
            self.primary_idx = idx;
        }
    }

    /// Get a selection by index.
    pub fn get(&self, idx: usize) -> Option<&Selection> {
        self.selections.get(idx)
    }

    /// Get a mutable selection by index.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Selection> {
        self.selections.get_mut(idx)
    }

    /// Add a new selection.
    ///
    /// The selections are not automatically sorted or merged.
    /// Call `normalize()` to sort and merge overlapping selections.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::{Selections, Selection};
    /// use lite_xl::buffer::Position;
    ///
    /// let mut selections = Selections::single(Position::new(0, 0));
    /// selections.add(Selection::cursor(Position::new(1, 0)));
    /// assert_eq!(selections.len(), 2);
    /// ```
    pub fn add(&mut self, selection: Selection) {
        self.selections.push(selection);
    }

    /// Add a new cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selections;
    /// use lite_xl::buffer::Position;
    ///
    /// let mut selections = Selections::single(Position::new(0, 0));
    /// selections.add_cursor(Position::new(1, 0));
    /// assert_eq!(selections.len(), 2);
    /// ```
    pub fn add_cursor(&mut self, pos: Position) {
        self.add(Selection::cursor(pos));
    }

    /// Remove a selection by index.
    ///
    /// If this is the last selection, it will not be removed.
    pub fn remove(&mut self, idx: usize) {
        if self.selections.len() > 1 && idx < self.selections.len() {
            self.selections.remove(idx);
            // Adjust primary index if necessary
            if self.primary_idx >= self.selections.len() {
                self.primary_idx = self.selections.len() - 1;
            } else if idx < self.primary_idx {
                self.primary_idx -= 1;
            }
        }
    }

    /// Clear all selections and set a single cursor.
    pub fn clear_to_cursor(&mut self, pos: Position) {
        self.selections.clear();
        self.selections.push(Selection::cursor(pos));
        self.primary_idx = 0;
    }

    /// Get an iterator over all selections.
    pub fn iter(&self) -> impl Iterator<Item = &Selection> {
        self.selections.iter()
    }

    /// Get a mutable iterator over all selections.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Selection> {
        self.selections.iter_mut()
    }

    /// Transform all selections using a function.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::Selections;
    /// use lite_xl::buffer::Position;
    ///
    /// let mut selections = Selections::single(Position::new(0, 0));
    /// selections.transform(|sel| {
    ///     let mut new_sel = sel.clone();
    ///     new_sel.extend_to(Position::new(0, 5));
    ///     new_sel
    /// });
    /// ```
    pub fn transform<F>(&mut self, mut f: F)
    where
        F: FnMut(&Selection) -> Selection,
    {
        for selection in &mut self.selections {
            *selection = f(selection);
        }
    }

    /// Sort selections by their start position.
    pub fn sort(&mut self) {
        let primary = self.selections[self.primary_idx].clone();
        
        self.selections.sort_by(|a, b| {
            let a_range = a.range();
            let b_range = b.range();
            a_range.start.cmp(&b_range.start)
        });

        // Update primary index
        self.primary_idx = self.selections.iter()
            .position(|s| s == &primary)
            .unwrap_or(0);
    }

    /// Merge overlapping selections.
    ///
    /// This removes overlapping selections by merging them into single selections.
    /// The selections should be sorted before calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use lite_xl::document::{Selections, Selection};
    /// use lite_xl::buffer::Position;
    ///
    /// let mut selections = Selections::from_vec(vec![
    ///     Selection::new(Position::new(0, 0), Position::new(0, 5)),
    ///     Selection::new(Position::new(0, 3), Position::new(0, 8)),
    /// ]);
    /// selections.normalize();
    /// assert_eq!(selections.len(), 1);
    /// ```
    pub fn merge_overlapping(&mut self) {
        if self.selections.len() <= 1 {
            return;
        }

        let primary = self.selections[self.primary_idx].clone();
        let mut merged = Vec::with_capacity(self.selections.len());
        let mut current = self.selections[0].clone();

        for selection in &self.selections[1..] {
            if current.overlaps(selection) || current.range().end == selection.range().start {
                current = current.merge(selection);
            } else {
                merged.push(current);
                current = selection.clone();
            }
        }
        merged.push(current);

        self.selections = merged;

        // Update primary index
        self.primary_idx = self.selections.iter()
            .position(|s| {
                let s_range = s.range();
                let p_range = primary.range();
                s_range.start <= p_range.start && p_range.end <= s_range.end
            })
            .unwrap_or(0);
    }

    /// Normalize selections (sort and merge).
    ///
    /// This is a convenience method that calls `sort()` then `merge_overlapping()`.
    pub fn normalize(&mut self) {
        self.sort();
        self.merge_overlapping();
    }

    /// Check if any selection is not a cursor.
    pub fn has_selection(&self) -> bool {
        self.selections.iter().any(|s| !s.is_cursor())
    }

    /// Collapse all selections to their cursor positions.
    pub fn collapse_all(&mut self) {
        for selection in &mut self.selections {
            selection.collapse_to_cursor();
        }
    }
}

impl Default for Selections {
    fn default() -> Self {
        Self::single(Position::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_creation() {
        let cursor = Selection::cursor(Position::new(5, 10));
        assert!(cursor.is_cursor());
        assert_eq!(cursor.anchor(), Position::new(5, 10));
        assert_eq!(cursor.head(), Position::new(5, 10));

        let sel = Selection::new(Position::new(0, 0), Position::new(0, 10));
        assert!(!sel.is_cursor());
    }

    #[test]
    fn test_selection_range() {
        let sel = Selection::new(Position::new(0, 10), Position::new(0, 0));
        let range = sel.range();
        assert_eq!(range.start, Position::new(0, 0));
        assert_eq!(range.end, Position::new(0, 10));
    }

    #[test]
    fn test_selection_extend() {
        let mut sel = Selection::cursor(Position::new(0, 0));
        sel.extend_to(Position::new(0, 10));
        assert!(!sel.is_cursor());
        assert_eq!(sel.anchor(), Position::new(0, 0));
        assert_eq!(sel.head(), Position::new(0, 10));
    }

    #[test]
    fn test_selection_move() {
        let mut sel = Selection::new(Position::new(0, 0), Position::new(0, 10));
        sel.move_to(Position::new(5, 5));
        assert!(sel.is_cursor());
        assert_eq!(sel.head(), Position::new(5, 5));
    }

    #[test]
    fn test_selections_single() {
        let selections = Selections::single(Position::new(0, 0));
        assert_eq!(selections.len(), 1);
        assert_eq!(selections.primary().head(), Position::new(0, 0));
    }

    #[test]
    fn test_selections_add() {
        let mut selections = Selections::single(Position::new(0, 0));
        selections.add_cursor(Position::new(1, 0));
        selections.add_cursor(Position::new(2, 0));
        assert_eq!(selections.len(), 3);
    }

    #[test]
    fn test_selections_sort() {
        let mut selections = Selections::from_vec(vec![
            Selection::cursor(Position::new(2, 0)),
            Selection::cursor(Position::new(0, 0)),
            Selection::cursor(Position::new(1, 0)),
        ]);
        selections.sort();
        assert_eq!(selections.get(0).unwrap().head(), Position::new(0, 0));
        assert_eq!(selections.get(1).unwrap().head(), Position::new(1, 0));
        assert_eq!(selections.get(2).unwrap().head(), Position::new(2, 0));
    }

    #[test]
    fn test_selections_merge() {
        let mut selections = Selections::from_vec(vec![
            Selection::new(Position::new(0, 0), Position::new(0, 5)),
            Selection::new(Position::new(0, 3), Position::new(0, 8)),
        ]);
        selections.normalize();
        assert_eq!(selections.len(), 1);
        assert_eq!(selections.primary().range().start, Position::new(0, 0));
        assert_eq!(selections.primary().range().end, Position::new(0, 8));
    }

    #[test]
    fn test_selections_no_merge_non_overlapping() {
        let mut selections = Selections::from_vec(vec![
            Selection::new(Position::new(0, 0), Position::new(0, 5)),
            Selection::new(Position::new(1, 0), Position::new(1, 5)),
        ]);
        selections.normalize();
        assert_eq!(selections.len(), 2);
    }

    #[test]
    fn test_selections_transform() {
        let mut selections = Selections::single(Position::new(0, 0));
        selections.add_cursor(Position::new(1, 0));
        
        selections.transform(|sel| {
            let mut new_sel = sel.clone();
            new_sel.extend_to(sel.head().offset_column(5));
            new_sel
        });

        assert!(!selections.get(0).unwrap().is_cursor());
        assert!(!selections.get(1).unwrap().is_cursor());
    }
}

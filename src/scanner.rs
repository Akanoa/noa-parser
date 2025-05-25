//! A scanner for a sequence of elements.

use crate::errors::ParseResult;
use crate::visitor::Visitor;
use std::io::Cursor;
use std::ops::Deref;

/// Wrapper around a `Cursor`.
#[derive(Debug, PartialEq)]
pub struct Scanner<'a, T> {
    /// The internal cursor.
    cursor: Cursor<&'a [T]>,
}

impl<'a, T> Scanner<'a, T> {
    pub fn new(data: &'a [T]) -> Scanner<'a, T> {
        Scanner {
            cursor: Cursor::new(data),
        }
    }
}

impl<'a, T> Scanner<'a, T> {
    /// Move the internal cursor forward by `n` positions.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of positions to move the cursor forward.
    ///
    /// # Panics
    ///
    /// Panics if the internal cursor is moved past the end of the data.
    pub fn bump_by(&mut self, n: usize) {
        self.cursor.set_position(self.cursor.position() + n as u64);
    }

    /// Move the internal cursor to the specified position.
    ///
    /// # Arguments
    ///
    /// * `n` - The position to move the cursor to.
    ///
    /// # Panics
    ///
    /// Panics if the internal cursor is moved past the end of the data.
    pub fn jump_to(&mut self, n: usize) {
        self.cursor.set_position(n as u64);
    }

    /// Move the internal cursor backward by `n` positions.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of positions to move the cursor backward.
    ///
    /// # Panics
    ///
    /// Panics if the internal cursor is moved to a position before the start of the data.
    pub fn rewind(&mut self, n: usize) {
        self.cursor.set_position(self.cursor.position() - n as u64);
    }

    /// Return the current position of the internal cursor.
    ///
    /// # Returns
    ///
    /// The current position of the internal cursor.
    pub fn current_position(&self) -> usize {
        self.cursor.position() as usize
    }

    /// Return a slice of the data that remains to be scanned.
    ///
    /// # Returns
    ///
    /// A slice of the data that remains to be scanned.
    pub fn remaining(&self) -> &[T] {
        &self.cursor.get_ref()[self.current_position()..]
    }

    /// Return the original data given to the scanner.
    ///
    /// # Returns
    ///
    /// The original data given to the scanner.
    pub fn data(&self) -> &'a [T] {
        self.cursor.get_ref()
    }

    /// Consume the scanner and return a slice of the remaining data.
    ///
    /// # Returns
    ///
    /// A slice of the remaining data.
    pub fn into_data(self) -> &'a [T] {
        &self.cursor.get_ref()[self.current_position()..]
    }

    /// Return true if there are no more elements to scan, false otherwise.
    ///
    /// # Returns
    ///
    /// true if there are no more elements to scan, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.remaining().is_empty()
    }
}

impl<'a, T> Deref for Scanner<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.remaining()
    }
}

impl<'a, T> Scanner<'a, T> {
    /// Run a visitor on the scanner.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The type of the visitor to run.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The scanner to run the visitor on.
    ///
    /// # Returns
    ///
    /// The result of running the visitor on the scanner.
    pub fn visit<V: Visitor<'a, T>>(&mut self) -> ParseResult<V> {
        V::accept(self)
    }
}

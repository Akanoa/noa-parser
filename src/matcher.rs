//! Provides the `Match` trait.

/// Describes a matchable object.
pub trait Match<T> {
    /// Returns true if the data matches the pattern.
    ///
    /// # Arguments
    /// data - the data to match
    ///
    /// # Returns
    /// (true, index) if the data matches the pattern,
    /// (false, index) otherwise
    fn matcher(&self, data: &[T]) -> (bool, usize);
}

/// Size of the matchable object.
pub trait MatchSize {
    /// Returns the size of the matchable object.
    fn size(&self) -> usize;
}

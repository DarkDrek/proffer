//!
//! Traits for documentation lines
//!

use crate::internal::Docs;

/// Provides methods to add documentation line. to elements.
pub trait DocExt {
    /// Add a single documentation line.
    fn add_doc(&mut self, trait_bound: impl ToString) -> &mut Self;

    /// Add multiple documentation lines at once.
    fn add_docs(&mut self, doc: impl IntoIterator<Item = impl ToString>) -> &mut Self;
}

impl<T: Docs> DocExt for T {
    /// Add a single documentation line.
    fn add_doc(&mut self, trait_bound: impl ToString) -> &mut Self {
        self.docs().push(trait_bound.to_string());
        self
    }

    /// Add multiple documentation lines at once.
    fn add_docs(&mut self, doc: impl IntoIterator<Item = impl ToString>) -> &mut Self {
        self.docs().extend(doc.into_iter().map(|d| d.to_string()));
        self
    }
}

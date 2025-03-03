use slab::Slab;

use super::SubmissionHandler;

/// A boxed [`SubmissionHandler`].
pub(crate) type BoxedHandler<'h, S> = Box<dyn SubmissionHandler<S> + 'h>;

/// Index of a stored [`BoxedHandler`].
pub(crate) type Index = usize;

/// A [`BoxedHandler`] container.
pub(crate) struct Handlers<'h, S> {
    slab: Slab<BoxedHandler<'h, S>>,
}

impl<'h, S> Handlers<'h, S> {
    /// Create a new [`Handlers`].
    pub(crate) const fn new() -> Self {
        Self { slab: Slab::new() }
    }

    /// Get the [`Index`] which will be used for the next insertion.
    pub(crate) fn vacant_index(&self) -> Index {
        self.slab.vacant_key()
    }

    /// Add a [`BoxedHandler`].
    ///
    /// Returns the [`Index`] of the added [`BoxedHandler`].
    pub(crate) fn insert(&mut self, handler: BoxedHandler<'h, S>) -> Index {
        self.slab.insert(handler)
    }

    /// Take a [`BoxedHandler`] using its [`Index`].
    ///
    /// The [`BoxedHandler`] is removed when it is taken.
    pub(crate) fn take(&mut self, index: Index) -> Option<BoxedHandler<'h, S>> {
        self.slab.try_remove(index)
    }
}

use crate::io::Submission;
use crate::{event_loop::Queue, io::Completion};

pub(crate) mod list;

/// A handler for submission.
pub trait SubmissionHandler<S> {
    /// Called when pushing to the queue.
    ///
    /// Return the next submission.
    fn on_push(&mut self) -> Submission;

    /// Called when the submission is completed.
    fn on_completion<'h>(
        self: Box<Self>,
        event: Completion,
        queue: &mut Queue<'_, 'h, S>,
        state: &mut S,
    ) -> Result<(), String>
    where
        Self: 'h;
}

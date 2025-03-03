use crate::{
    handler::{SubmissionHandler, list::Handlers},
    io::{
        Completion,
        sys::{CompletionQueue, IoBuilder, SubmissionQueue},
    },
};

struct EventLoop<'h, S> {
    sq: SubmissionQueue,
    cq: CompletionQueue,
    hd: Handlers<'h, S>,
}

impl<'h, S> EventLoop<'h, S> {
    /// Create a new event loop.
    pub fn new() -> Result<Self, String> {
        let (submissions, completions) = IoBuilder::new().build().map_err(|e| e.to_string())?;

        Ok(Self {
            sq: submissions,
            cq: completions,
            hd: Handlers::new(),
        })
    }

    /// Get the submission queue.
    pub fn queue(&mut self) -> Queue<'_, 'h, S> {
        Queue(self)
    }
}

impl<'h, S: 'h> EventLoop<'h, S> {
    /// Wait for events and dispatch them.
    pub fn dispatch(&mut self, state: &mut S) -> Result<(), String> {
        // Submit all queued submissions and wait for completions
        self.sq
            .submit_and_wait()
            .map_err(|_| "Failed to wait for completions")?;

        // Dispatch events
        while let Some(event) = self.cq.next() {
            // Get the associated handler
            let handler = self
                .hd
                .take(event.user_data())
                .ok_or("Handler not found!")?;

            // Handle the event
            handler.on_completion(Completion(event), &mut Queue(self), state)?;
        }

        Ok(())
    }
}

/// A queue to push new submissions.
pub struct Queue<'a, 'h, S>(&'a mut EventLoop<'h, S>);

impl<'h, S> Queue<'_, 'h, S> {
    /// Push a new submission.
    pub fn push<H>(&mut self, mut handler: Box<H>) -> Result<(), String>
    where
        H: SubmissionHandler<S> + 'h,
    {
        let mut submission = handler.on_push().0;

        // Set [`sys::Submission`]'s user data to the index
        submission.set_user_data(self.0.hd.vacant_index());

        // Queue the submission to the IO system
        self.0
            .sq
            .push(submission)
            .map_err(|_| "Failed to queue submission to IO system")?;

        // Store the handler
        self.0.hd.insert(handler);

        Ok(())
    }
}

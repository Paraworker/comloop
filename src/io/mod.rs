use std::io;

pub(crate) mod sys;

/// An IO submission request.
pub struct Submission(pub(crate) sys::Submission);

impl Submission {
    pub fn read() -> Self {
        todo!()
    }

    pub fn write() -> Self {
        todo!()
    }

    pub fn poll_add() -> Self {
        todo!()
    }

    pub fn poll_remove() -> Self {
        todo!()
    }
}

/// An IO completion event.
pub struct Completion(pub(crate) sys::Completion);

impl Completion {
    /// Get the result of the completion.
    pub fn result(&self) -> Result<(), io::Error> {
        todo!()
    }
}

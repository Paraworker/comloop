use std::io;

pub(crate) struct IoBuilder {}

impl IoBuilder {
    pub(crate) fn new() -> Self {
        todo!()
    }

    pub(crate) fn build(self) -> io::Result<(SubmissionQueue, CompletionQueue)> {
        todo!()
    }
}

pub(crate) struct Submission {}

impl Submission {
    pub(crate) fn new() -> Self {
        todo!()
    }

    pub(crate) fn set_user_data(&mut self, data: usize) {
        todo!();
    }
}

pub(crate) struct SubmissionQueue {}

impl SubmissionQueue {
    pub(crate) fn push(&mut self, submission: Submission) -> io::Result<()> {
        todo!();
    }

    pub(crate) fn submit_and_wait(&mut self) -> io::Result<usize> {
        todo!();
    }
}

pub(crate) struct Completion {}

impl Completion {
    pub(crate) fn new() -> Self {
        todo!()
    }

    pub(crate) fn user_data(&self) -> usize {
        todo!();
    }
}

pub(crate) struct CompletionQueue {}

impl CompletionQueue {
    pub(crate) fn next(&mut self) -> Option<Completion> {
        todo!()
    }
}

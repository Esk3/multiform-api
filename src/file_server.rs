use std::{future::Future, pin::Pin};

use std::fs::File;

#[derive(Debug, Clone)]
pub struct FileServer {
    path: String,
}

impl FileServer {
    #[must_use]
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

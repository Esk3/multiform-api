use std::{future::Future, pin::Pin};

use std::fs::File;

use super::service::Service;

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

impl Service<String> for FileServer {
    type Response = tiny_http::Response<File>;

    type Error = ();

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, path: String) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            let file = std::fs::File::open(this.path + &path).unwrap();
            // TODO: set response content-type header
            Ok(tiny_http::Response::from_file(file))
        })
    }
}

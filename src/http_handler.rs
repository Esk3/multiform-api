use crate::{into_response::IntoResponse, service::Service};
use std::{future::Future, pin::Pin};

#[derive(Debug, Clone)]
pub struct HttpHandler<S: Clone> {
    pub inner: S,
}
impl<S> Service<tiny_http::Request> for HttpHandler<S>
where
    S: for<'a> Service<tiny_http::Request, Response = (tiny_http::Request, Box<dyn IntoResponse>)>
        + Clone
        + 'static,
{
    type Response = (tiny_http::Request, Box<dyn IntoResponse>);

    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, request: tiny_http::Request) -> Self::Future {
        let mut this = self.clone();
        Box::pin(async move {
            let Ok(response) = this.inner.call(request).await else {
                panic!();
            };
            Ok(response)
        })
    }
}

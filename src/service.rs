use std::{future::Future, pin::Pin};

pub trait Service<Request> {
    type Response;
    type Error;

    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, request: Request) -> Self::Future;
}

pub type Fut<T, E> = Pin<Box<dyn Future<Output = Result<T, E>>>>;

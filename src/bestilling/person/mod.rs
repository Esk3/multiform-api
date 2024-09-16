use crate::{error::ServerError, into_response::IntoResponse, service::{Fut, Service}};

use super::router_args::Args;

pub mod model;

pub fn handler() -> impl Service<Args, Response = Box<dyn IntoResponse>, Error = ServerError> + Clone {
    Handler
}

#[derive(Clone)]
struct Handler;
impl Service<Args> for Handler {
    type Response = Box<dyn IntoResponse>;

    type Error = ServerError;

    type Future = Fut<Self::Response, Self::Error>;

    fn call(&mut self, request: Args) -> Self::Future {
        todo!()
    }
}

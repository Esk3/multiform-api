use std::sync::{Arc, Mutex};

use crate::{
    index,
    into_response::IntoResponse,
    service::{Fut, Service},
    State,
};
use tiny_http::Request;

#[derive(Debug, Clone)]
pub struct Router {
    pub state: Arc<Mutex<State>>,
}

impl Router {
    pub fn new() -> Self {
        todo!()
    }
    fn route(&self, url: &str, method: &tiny_http::Method) -> Route {
        match (url, method) {
            ("/", tiny_http::Method::Get) => Route::GetIndex,
            _ => Route::NotFound,
        }
    }
    async fn run_route(&self, route: Route) -> Result<Box<dyn IntoResponse>, ()> {
        match route {
            Route::GetIndex => Ok(index()),
            Route::GetOther => todo!(),
            Route::NotFound => todo!(),
        }
    }
}

impl Service<tiny_http::Request> for Router {
    type Response = (Request, Box<dyn IntoResponse>);

    type Error = ();

    type Future = Fut<Self::Response, Self::Error>;

    fn call(&mut self, request: tiny_http::Request) -> Self::Future {
        let this = self.clone();
        let route = this.route(request.url(), request.method());
        Box::pin(async move {
            let response = this.run_route(route).await.unwrap();
            Ok((request, response))
        })
    }
}

enum Route {
    GetIndex,
    GetOther,
    NotFound,
}

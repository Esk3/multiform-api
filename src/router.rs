use std::sync::{Arc, Mutex};

use crate::{
    index,
    into_response::IntoResponse,
    read,
    service::{Fut, Service},
    write, State,
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
    fn route(&self, url: &str, method: &tiny_http::Method, body: String) -> Route {
        match (url, method) {
            ("/", tiny_http::Method::Get) => Route::GetIndex,
            (url, tiny_http::Method::Get) if url.starts_with("/read/") => {
                let key = url.strip_prefix("/read/").unwrap().to_string();
                Route::GetRead { key }
            }
            (url, tiny_http::Method::Post) if url.starts_with("/write/") => {
                let key = url.strip_prefix("/write/").unwrap().to_string();
                Route::PostWrite { key, value: body }
            }
            _ => Route::NotFound,
        }
    }
    async fn run_route(&self, route: Route) -> Result<Box<dyn IntoResponse>, ()> {
        match route {
            Route::GetIndex => Ok(index()),
            Route::GetRead { key } => Ok(read(self.state.clone(), key)),
            Route::PostWrite { key, value } => Ok(write(self.state.clone(), key, value)),
            Route::NotFound => todo!(),
        }
    }
}

impl Service<tiny_http::Request> for Router {
    type Response = (Request, Box<dyn IntoResponse>);

    type Error = ();

    type Future = Fut<Self::Response, Self::Error>;

    fn call(&mut self, mut request: tiny_http::Request) -> Self::Future {
        let mut buf = String::new();
        request.as_reader().read_to_string(&mut buf).unwrap();
        let this = self.clone();
        let route = this.route(request.url(), request.method(), buf);
        Box::pin(async move {
            let response = this.run_route(route).await.unwrap();
            Ok((request, response))
        })
    }
}

enum Route {
    GetIndex,
    GetRead { key: String },
    PostWrite { key: String, value: String },
    NotFound,
}

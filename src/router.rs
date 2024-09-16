use crate::{
    bestilling::router_args::RouterArgs,
    error::ServerError,
    index,
    into_response::IntoResponse,
    read,
    service::{Fut, Service},
    write, State,
};
use tiny_http::Request;

#[derive(Debug, Clone)]
pub struct Router<B: Clone> {
    pub state: State,
    bestings_handler: B,
}

impl<B> Router<B>
where
    B: Service<RouterArgs, Response = Box<dyn IntoResponse>, Error = ServerError> + Clone,
{
    pub fn new(state: State, bestings_handler: B) -> Self {
        Self {
            state,
            bestings_handler,
        }
    }
    fn route(&self, request: &tiny_http::Request, body: String) -> Route {
        let (url, method) = (request.url(), request.method());
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
            (url, _) if url.starts_with("/bestilling") => {
                let args = crate::bestilling::router_args::RouterArgs::clone_from_http_request(
                    request,
                    body,
                    self.state.pool.clone(),
                );
                Route::Bestilling(args)
            }
            _ => Route::NotFound,
        }
    }
    async fn run_route(&mut self, route: Route) -> Result<Box<dyn IntoResponse>, ()> {
        match route {
            Route::GetIndex => Ok(index()),
            Route::GetRead { key } => Ok(read(self.state.clone(), key)),
            Route::PostWrite { key, value } => Ok(write(self.state.clone(), key, value)),
            // Route::Bestilling(args) => bestilling::Router.call(args).await,
            // Route::Bestilling(args) => bestilling::handler().call(args).await,
            Route::Bestilling(args) => self.bestings_handler.call(args).await,
            Route::NotFound => todo!(),
        }
    }
}

impl<B> Service<tiny_http::Request> for Router<B>
where
    B: Service<RouterArgs, Response = Box<dyn IntoResponse>, Error = ()> + Clone + 'static,
{
    type Response = (Request, Box<dyn IntoResponse>);

    type Error = ();

    type Future = Fut<Self::Response, Self::Error>;

    fn call(&mut self, mut request: tiny_http::Request) -> Self::Future {
        let mut buf = String::new();
        request.as_reader().read_to_string(&mut buf).unwrap();
        let mut this = self.clone();
        let route = this.route(&request, buf);
        Box::pin(async move {
            match this.run_route(route).await {
                Ok(response) => Ok((request, response)),
                Err(e) => Err(e),
            }
            // let response = this.run_route(route).await;
            // Ok((request, response))
        })
    }
}

enum Route {
    GetIndex,
    GetRead { key: String },
    PostWrite { key: String, value: String },
    Bestilling(crate::bestilling::router_args::RouterArgs),
    NotFound,
}

use crate::{
    into_response::IntoResponse,
    response::new_header,
    service::{self, Fut, Service},
};

use super::router_args::RouterArgs;

#[derive(Clone)]
pub struct BestillingsId<T: Clone> {
    pub inner: T,
}

impl<T> service::Service<RouterArgs> for BestillingsId<T>
where
    T: Service<RouterArgs, Response = Box<dyn IntoResponse>, Error = ()> + Clone + 'static,
{
    type Response = Box<dyn IntoResponse>;

    type Error = ();

    type Future = Fut<Self::Response, Self::Error>;

    fn call(&mut self, request: RouterArgs) -> Self::Future {
        let mut this = self.clone();
        if request.bestillings_id.is_some() {
            Box::pin(async move { this.inner.call(request).await })
        } else {
            Box::pin(async move {
                Ok::<Box<dyn IntoResponse>, ()>(Box::new(tiny_http::Response::new(
                    tiny_http::StatusCode(301),
                    vec![new_header("Location", "/my_redirect")],
                    std::io::Cursor::new(Vec::new()),
                    Some(0),
                    None,
                )))
            })
        }
    }
}

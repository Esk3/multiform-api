use router_args::{Args, RouterArgs};

use crate::{into_response::IntoResponse, service::Service};

mod bestilling_id;
pub mod billett;
mod model;
pub mod person;
pub mod router_args;

pub fn handler() -> impl Service<RouterArgs, Response = Box<dyn IntoResponse>, Error = ()> + Clone {
    bestilling_id::BestillingsId::new(Router {
        billett_handler: billett::handler(),
        person_handler: person::handler(),
    })
}

#[derive(Clone)]
pub struct Router<B, P> {
    billett_handler: B,
    person_handler: P,
}

impl<B, P> crate::service::Service<Args> for Router<B, P>
where
    B: Service<Args, Response = Box<dyn IntoResponse>, Error = ()> + Clone + 'static,
    P: Service<Args, Response = Box<dyn IntoResponse>, Error = ()> + Clone + 'static,
{
    type Response = Box<dyn IntoResponse>;

    type Error = ();

    type Future = crate::service::Fut<Self::Response, Self::Error>;

    fn call(&mut self, request: Args) -> Self::Future {
        let mut this = self.clone();
        Box::pin(async move {
            match (request.url.as_str(), &request.method) {
                ("/bestilling/billett", _) => this.billett_handler.call(request).await,
                ("/person", _) => this.person_handler.call(request).await,
                (_, _) => Err(()),
            }
        })
    }
}

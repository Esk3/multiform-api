use router_args::{Args, RouterArgs};

use crate::{into_response::IntoResponse, service::Service};

mod bestilling_id;
mod billett;
mod model;
mod person;
pub mod router_args;

pub fn handler() -> impl Service<RouterArgs, Response = Box<dyn IntoResponse>, Error = ()> {
    bestilling_id::BestillingsId::new(Router) 
}

#[derive(Clone)]
pub struct Router;

impl crate::service::Service<Args> for Router {
    type Response = Box<dyn IntoResponse>;

    type Error = ();

    type Future = crate::service::Fut<Self::Response, Self::Error>;

    fn call(
        &mut self,
        Args {
            url,
            method,
            bestillings_id,
            body,
            pool,
        }: Args,
    ) -> Self::Future {
        Box::pin(async move {
            dbg!(bestillings_id, body);
            // match (url.as_str(), method) {
            //     ("/bestilling", _) => todo!(),
            //     (_, _) => todo!(),
            // };
            let rows = sqlx::query("select * from billett")
                .fetch_all(&*pool)
                .await
                .unwrap();
            dbg!(rows);
            Ok::<Box<dyn IntoResponse>, ()>(Box::new(()))
        })
    }
}

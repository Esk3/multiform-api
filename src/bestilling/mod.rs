use std::sync::Arc;

use crate::into_response::IntoResponse;

mod billett;
mod model;
mod person;

pub struct RouterArgs {
    url: String,
    method: tiny_http::Method,
    bestillings_id: Option<i32>,
    body: String,
    pool: Arc<sqlx::PgPool>,
}

impl RouterArgs {
    pub fn clone_from_http_request(
        request: &tiny_http::Request,
        body: String,
        pool: Arc<sqlx::PgPool>,
    ) -> Self {
        let bestillings_id = request
            .headers()
            .into_iter()
            .find(|header| header.field.equiv("bestillings_id"))
            .map(|header| header.value.clone())
            .map(|value| value.as_str().parse().ok())
            .flatten();
        Self {
            url: request.url().to_string(),
            method: request.method().clone(),
            bestillings_id,
            body,
            pool,
        }
    }
}

pub struct Router;

impl crate::service::Service<RouterArgs> for Router {
    type Response = Box<dyn IntoResponse>;

    type Error = ();

    type Future = crate::service::Fut<Self::Response, Self::Error>;

    fn call(
        &mut self,
        RouterArgs {
            url,
            method,
            bestillings_id,
            body,
            pool,
        }: RouterArgs,
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

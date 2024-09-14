use std::sync::Arc;

pub struct RouterArgs {
    pub url: String,
    pub method: tiny_http::Method,
    pub bestillings_id: Option<i32>,
    pub body: String,
    pub pool: Arc<sqlx::PgPool>,
}

impl RouterArgs {
    pub fn clone_from_http_request(
        request: &tiny_http::Request,
        body: String,
        pool: Arc<sqlx::PgPool>,
    ) -> Self {
        let bestillings_id = request
            .headers()
            .iter()
            .find(|header| header.field.equiv("bestillings_id"))
            .map(|header| header.value.clone())
            .and_then(|value| value.as_str().parse().ok());
        Self {
            url: request.url().to_string(),
            method: request.method().clone(),
            bestillings_id,
            body,
            pool,
        }
    }
}

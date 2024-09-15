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
        let bestillings_id = request.headers()
            .iter()
            .find(|header| header.field.equiv("Cookie"))
            .and_then(|header| {
                header
                    .value
                    .to_string()
                    .split("; ")
                    .find_map(|s| {
                        let (key, value) = s.split_once('=')?;
                        if key != "bestillings_id" {
                            return  None;
                        }
                        Some(value.to_string())
                    })
            })
            .and_then(|value| value.parse().ok());
        Self {
            url: request.url().to_string(),
            method: request.method().clone(),
            bestillings_id,
            body,
            pool,
        }
    }
}
pub struct Args {
    pub url: String,
    pub method: tiny_http::Method,
    pub bestillings_id: i32,
    pub body: String,
    pub pool: Arc<sqlx::PgPool>,
}

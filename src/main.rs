use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::service::Service;
use http_handler::HttpHandler;
use into_response::IntoResponse;
use router::Router;
use sqlx::PgPool;

mod bestilling;
mod file_server;
mod http_handler;
mod into_response;
mod response;
mod router;
mod service;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();

    let state = State {
        store: Arc::new(Mutex::new(HashMap::new())),
        pool: Arc::new(pool),
    };

    let mut handler = HttpHandler {
        inner: Router::new(state, bestilling::handler()),
    };

    let server = tiny_http::Server::http("127.0.0.1:3000").unwrap();
    println!("server listing on: {}", server.server_addr());

    for request in server.incoming_requests() {
        let (req, res) = handler.call(request).await.unwrap();
        req.respond(res.into_response()).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub store: Arc<Mutex<HashMap<String, String>>>,
    pub pool: Arc<PgPool>,
}

fn index() -> Box<dyn IntoResponse> {
    Box::new("hey from index")
}

fn read(state: State, key: String) -> Box<dyn IntoResponse> {
    if let Some(value) = state.store.lock().unwrap().get(&key) {
        Box::new(value.to_string())
    } else {
        Box::new("key not found")
    }
}

fn write(state: State, key: String, value: String) -> Box<dyn IntoResponse> {
    if let Some(old_value) = state.store.lock().unwrap().insert(key, value) {
        Box::new(old_value)
    } else {
        Box::new(())
    }
}

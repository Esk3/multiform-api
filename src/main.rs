use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::service::Service;
use http_handler::HttpHandler;
use into_response::IntoResponse;
use router::Router;

mod file_server;
mod http_handler;
mod into_response;
mod response;
mod router;
mod service;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(State {
        store: HashMap::new()
    }));
    let mut handler = HttpHandler {
        inner: Router { state },
    };
    let server = tiny_http::Server::http("127.0.0.1:3000").unwrap();
    println!("server listing on: {}", server.server_addr());
    for request in server.incoming_requests() {
        let (req, res) = handler.call(request).await.unwrap();
        req.respond(res.into_response()).unwrap();
    }
}

#[derive(Debug)]
pub struct State {
    pub store: HashMap<String, String>
}

fn index() -> Box<dyn IntoResponse> {
    Box::new("hey from index")
}

fn read(state: Arc<Mutex<State>>, key: String) -> Box<dyn IntoResponse> {
    if let Some(value) = state.lock().unwrap().store.get(&key) {
        Box::new(value.to_string())
    } else {
        Box::new("key not found")
    }
}

fn write(state: Arc<Mutex<State>>, key: String, value: String) -> Box<dyn IntoResponse> {
    if let Some(old_value) = state.lock().unwrap().store.insert(key, value) {
        Box::new(old_value)
    } else {
        Box::new(())
    }
}

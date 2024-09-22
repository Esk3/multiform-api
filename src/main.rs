use poem::{listener::TcpListener, Route};
use poem_openapi::Tags;
use std::sync::Arc;

mod api;
mod www;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let pool = Arc::new(pool);

    let api_service = poem_openapi::OpenApiService::new(
        (
            api::billett::BilletApi::new(pool.clone()),
            api::person::PersonApi::new(pool.clone()),
            api::lufthavn::LufthavnApi::new(pool.clone()),
            api::fly::FlyApi::new(pool.clone()),
        ),
        "Fly Api",
        "1.0",
    )
    .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    println!("server listing on: localhost:3000");
    let router = Route::new().nest("/api", api_service).nest("/docs", ui).nest("/", www::web_router());
    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(router)
        .await
        .unwrap();
}

#[derive(Debug, Tags)]
pub enum ApiTags {
    Lufthavn,
    Billett,
    Person,
    Bestilling,
    Fly,
}

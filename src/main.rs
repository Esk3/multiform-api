use poem::{listener::TcpListener, Route};
use poem_openapi::Tags;
use std::sync::Arc;

mod bestilling;
mod lufthavn;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let pool = Arc::new(pool);

    println!("server listing on: localhost:3000");
    let api_service = poem_openapi::OpenApiService::new(
        (
            bestilling::billett::BilletApi { pool: pool.clone() },
            bestilling::person::PersonApi { pool: pool.clone() },
            lufthavn::LufthavnApi::new(pool.clone()),
        ),
        "Fly Api",
        "1.0",
    )
    .server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(Route::new().nest("/", api_service).nest("/docs", ui))
        .await
        .unwrap();
}

#[derive(Debug, Tags)]
pub enum ApiTags {
    Lufthavn,
    Billett,
    Person,
    Bestilling
}

use poem::{listener::TcpListener, Route};
use poem_openapi::Tags;
use std::sync::Arc;

mod billett;
mod fly;
mod lufthavn;
mod person;
mod reise;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let pool = Arc::new(pool);

    let api_service = poem_openapi::OpenApiService::new(
        (
            billett::BilletApi::new(pool.clone()),
            person::PersonApi::new(pool.clone()),
            lufthavn::LufthavnApi::new(pool.clone()),
            fly::FlyApi::new(pool.clone()),
        ),
        "Fly Api",
        "1.0",
    )
    .server("http://localhost:3000");
    let ui = api_service.swagger_ui();

    println!("server listing on: localhost:3000");
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
    Bestilling,
    Fly,
}

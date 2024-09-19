use poem_openapi::Object;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Object)]
pub struct Fly {
    #[sqlx(rename = "fly_id")]
    id: i32,
    luxus_seter: i32,
    flex_seter: i32,
    billig_seter: i32,
}

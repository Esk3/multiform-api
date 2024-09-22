use poem_openapi::Object;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Object)]
pub struct Fly {
    #[sqlx(rename = "fly_id")]
    pub id: i32,
    pub luxus_seter: i32,
    pub flex_seter: i32,
    pub billig_seter: i32,
}

#[derive(Debug, Object)]
pub struct FlyForm {
    pub luxus_seter: i32,
    pub flex_seter: i32,
    pub billig_seter: i32,
}

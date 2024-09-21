use poem_openapi::{Enum, Object};

#[derive(Debug, sqlx::Type, Enum)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
#[oai(rename_all = "snake_case")]
pub enum Status {
    Voksen,
    Barn,
    Honnør,
}

#[derive(Debug, sqlx::Type, Enum)]
#[sqlx(type_name = "billett_type", rename_all = "snake_case")]
#[oai(rename_all = "snake_case")]
pub enum BillettType {
    Billig,
    Flex,
    Luxus,
}

#[derive(Debug, sqlx::FromRow, Object)]
pub struct Billett {
    pub(super) billett_id: i32,
    pub(super) reise_id: Option<i32>,
    pub(super) person_id: Option<i32>,
    pub(super) bekreftet: bool,
    pub(super) status: Status,
    pub(super) billett_type: BillettType,
    pub(super) timestamp: String,
}

#[derive(Debug, Object)]
pub struct BillettForm {
    pub(super) reise_id: Option<i32>,
    pub(super) person_id: Option<i32>,
    #[oai(default)]
    pub(super) bekreftet: bool,
    pub(super) status: Status,
    pub(super) billett_type: BillettType,
}

#[derive(Debug, sqlx::FromRow, Object)]
pub struct BekreftetBillett {
    pub(super) billett_id: i32,
    pub(super) reise_id: i32,
    pub(super) person_id: i32,
    pub(super) status: String,
    pub(super) billett_type: String,
    pub(super) timestamp: String,
}

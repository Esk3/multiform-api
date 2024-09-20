use poem_openapi::Object;

#[derive(Debug, sqlx::FromRow, Object)]
pub struct Billett {
    pub(super) billett_id: i32,
    pub(super) reise_id: Option<i32>,
    pub(super) person_id: Option<i32>,
    pub(super) bekreftet: bool,
    pub(super) status: String,
    pub(super) billett_type: String,
    pub(super) timestamp: String,
}

#[derive(Debug, Object)]
pub struct BillettForm {
    pub(super) reise_id: Option<i32>,
    pub(super) person_id: Option<i32>,
    pub(super) bekreftet: bool,
    pub(super) status: String,
    pub(super) billett_type: String,
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

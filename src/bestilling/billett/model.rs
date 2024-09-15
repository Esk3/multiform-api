#[derive(Debug, sqlx::FromRow)]
pub struct Billett {
    pub(super) bestillings_id: i32,
    /// https://en.wikipedia.org/wiki/International_Air_Transport_Association
    pub(super) fra_iata_code: String,
    /// https://en.wikipedia.org/wiki/International_Air_Transport_Association
    pub(super) til_iata_code: String,
    pub(super) timestamp: String,
    pub(super) status: String,
    pub(super) billett_type: String,
}
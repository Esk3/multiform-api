use super::{billett::model::Billett, person::model::Person};

#[derive(Debug, sqlx::FromRow)]
pub struct Bestilling {
    pub id: i32,
    #[sqlx(flatten)]
    pub person: Person,
    #[sqlx(faltten)]
    pub billet: Billett,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BestillingId {
    pub id: i32,
}

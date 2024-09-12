use super::{billett::model::Billett, person::model::Person};

#[derive(Debug, sqlx::FromRow)]
pub struct Bestilling {
    id: i32,
    #[sqlx(flatten)]
    person: Person,
    #[sqlx(faltten)]
    billet: Billett,
}

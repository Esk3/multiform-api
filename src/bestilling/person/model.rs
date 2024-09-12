#[derive(Debug, sqlx::FromRow)]
pub struct Person {
    bestillings_id: i32,
    fornavn: String,
    etternavn: String,
    adresse: String,
    postnummer: u32,
    epost: String,
    telefonnummer: String,
}

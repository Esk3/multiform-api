#[derive(Debug, sqlx::FromRow)]
pub struct Person {
    id: i32,
    fornavn: String,
    etternavn: String,
    adresse: String,
    postnummer: i32,
    epost: String,
    telefonnummer: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BestillingPerson {
    bestilling_id: i32,
    person_id: i32,
}

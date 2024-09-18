use poem_openapi::Object;

#[derive(Debug, sqlx::FromRow, Object)]
pub struct Person {
    id: i32,
    fornavn: String,
    etternavn: String,
    adresse: String,
    postnummer: i32,
    epost: String,
    telefonnummer: i32,
}

#[derive(Debug, sqlx::FromRow, Object)]
pub struct PersonForm {
    pub fornavn: String,
    pub etternavn: String,
    pub adresse: String,
    pub postnummer: i32,
    pub epost: String,
    pub telefonnummer: i32,
}

#[derive(Debug, sqlx::FromRow, Object)]
pub struct BestillingPerson {
    pub bestilling_id: i32,
    pub person_id: i32,
}

#[derive(Debug, sqlx::FromRow, Object)]
pub struct PersonId {
    pub id: i32
}

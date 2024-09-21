use super::model;

pub struct PersonQuery<'a, 'b> {
    tx: &'a mut sqlx::Transaction<'b, sqlx::Postgres>
}

impl<'a, 'b> PersonQuery<'a,'b> {
    pub fn new(tx: &'a mut sqlx::Transaction<'b, sqlx::Postgres>) -> Self {
        Self { tx }
    }
    pub async fn get_fra_id(&mut self, id: i32) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where id = $1",
        )
        .bind(id)
        .fetch_one(&mut **self.tx)
        .await
    }
    pub async fn get_fra_fornavn(&mut self, fornavn: String) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where fornavn = $1",
        )
        .bind(fornavn)
        .fetch_one(&mut **self.tx)
        .await
    }
    pub async fn get_fra_etternavn(&mut self, etternavn: String) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where etternavn = $1",
        )
        .bind(etternavn)
        .fetch_one(&mut **self.tx)
        .await
    }
    pub async fn get_fra_fullnavn(&mut self, navn: String) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where fornavn || etternavn = $1",
        )
        .bind(navn)
        .fetch_one(&mut **self.tx)
        .await
    }
    pub async fn insert_person(
        &mut self,
        person: &model::PersonForm,
    ) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "insert into personer (fornavn, etternavn, adresse, postnummer, epost, telefonnummer)
            values ($1, $2, $3, $4, $5, $6)
            returning *",
        )
        .bind(&person.fornavn)
        .bind(&person.etternavn)
        .bind(&person.adresse)
        .bind(person.postnummer)
        .bind(&person.epost)
        .bind(person.telefonnummer)
        .fetch_one(&mut **self.tx)
        .await
    }
}

use std::sync::Arc;

use super::model;

pub struct PersonQuery {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl PersonQuery {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
    pub async fn get_fra_id(&self, id: i32) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where id = $1",
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn get_fra_fornavn(&self, fornavn: String) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where fornavn = $1",
        )
        .bind(fornavn)
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn get_fra_etternavn(&self, etternavn: String) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where etternavn = $1",
        )
        .bind(etternavn)
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn get_fra_fullnavn(&self, navn: String) -> Result<model::Person, sqlx::Error> {
        sqlx::query_as(
            "select *
            from person
            where fornavn || etternavn = $1",
        )
        .bind(navn)
        .fetch_one(&*self.pool)
        .await
    }
    pub async fn insert_person(
        &self,
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
        .fetch_one(&*self.pool)
        .await
    }
}

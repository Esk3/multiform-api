use std::sync::Arc;

use super::model::{self, BekreftetBillett, Billett};

pub struct BillettQuery {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}
impl BillettQuery {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn get_billett_by_id(&self, id: i32) -> Result<Option<Billett>, sqlx::Error> {
        sqlx::query_as(
            "select billett_id, reise_id, person_id, bekreftet,
                status, billett_type, timestamp::text
                from billetter
                where billett_id = $1",
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn get_bekreftet_billett_by_id(
        &self,
        id: i32,
    ) -> Result<Option<BekreftetBillett>, sqlx::Error> {
        sqlx::query_as(
            "select billett_id, reise_id, person_id,
            status::text, billett_type::text, timestamp::text
            from bekreftet_billetter
            where billett_id = $1",
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn insert_billet(
        &self,
        model::BillettForm {
            reise_id,
            person_id,
            bekreftet,
            status,
            billett_type,
        }: &model::BillettForm,
    ) -> Result<model::Billett, sqlx::Error> {
        sqlx::query_as(
            "insert into billetter (reise_id, person_id, bekreftet, status, billett_type)
            values ($1, $2, $3, $4, $5)
            returning billett_id, reise_id, person_id, bekreftet, status, billett_type, timestamp::text",
        )
        .bind(reise_id)
        .bind(person_id)
        .bind(bekreftet)
        .bind(status)
        .bind(billett_type)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn set_reise(&self, billett_id: i32, reise_id: i32) -> Result<Billett, sqlx::Error> {
        sqlx::query_as(
            "update billetter set reise_id = $1
            where billett_id = $2
            returning billett_id, reise_id, person_id, bekreftet, status, billett_type, timestamp::text",
        )
        .bind(reise_id)
        .bind(billett_id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn set_person(
        &self,
        billett_id: i32,
        person_id: i32,
    ) -> Result<Billett, sqlx::Error> {
        todo!()
    }
}

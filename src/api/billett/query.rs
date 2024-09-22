use super::model::{self, BekreftetBillett, Billett};

pub struct BillettQuery<'a, 'b> {
    tx: &'a mut sqlx::Transaction<'b, sqlx::Postgres>,
}
impl<'a, 'b> BillettQuery<'a, 'b> {
    pub fn new(tx: &'a mut sqlx::Transaction<'b, sqlx::Postgres>) -> Self {
        Self { tx }
    }

    pub async fn get_billett_by_id(&mut self, id: i32) -> Result<Option<Billett>, sqlx::Error> {
        sqlx::query_as(
            "select billett_id, reise_id, person_id, bekreftet,
                status, billett_type, timestamp::text
                from billetter
                where billett_id = $1",
        )
        .bind(id)
        .fetch_optional(&mut **self.tx)
        .await
    }

    pub async fn get_bekreftet_billett_by_id(
        &mut self,
        id: i32,
    ) -> Result<Option<BekreftetBillett>, sqlx::Error> {
        sqlx::query_as(
            "select billett_id, reise_id, person_id,
            status::text, billett_type::text, timestamp::text
            from bekreftet_billetter
            where billett_id = $1",
        )
        .bind(id)
        .fetch_optional(&mut **self.tx)
        .await
    }

    pub async fn insert_billet(
        &mut self,
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
        .fetch_one(&mut **self.tx)
        .await
    }

    pub async fn set_reise(
        &mut self,
        billett_id: i32,
        reise_id: i32,
    ) -> Result<Billett, sqlx::Error> {
        sqlx::query_as(
            "update billetter set reise_id = $1
            where billett_id = $2
            returning billett_id, reise_id, person_id, bekreftet, status, billett_type, timestamp::text",
        )
        .bind(reise_id)
        .bind(billett_id)
        .fetch_one(&mut **self.tx)
        .await
    }

    pub async fn set_person(
        &mut self,
        billett_id: i32,
        person_id: i32,
    ) -> Result<Billett, sqlx::Error> {
        todo!()
    }
}

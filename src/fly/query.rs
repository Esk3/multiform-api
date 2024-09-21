use super::model;

pub struct FlyQuery<'a, 'b> {
    tx: &'a mut sqlx::Transaction<'b, sqlx::Postgres>,
}

impl<'a, 'b> FlyQuery<'a, 'b> {
    pub fn new(tx: &'a mut sqlx::Transaction<'b, sqlx::Postgres>) -> Self {
        Self { tx }
    }
    pub async fn get_fly(&mut self) -> Result<Vec<model::Fly>, sqlx::Error> {
        sqlx::query_as(
            "select *
            from fly",
        )
        .fetch_all(&mut **self.tx)
        .await
    }
    pub async fn get_fly_by_id(&mut self, id: i32) -> Result<Option<model::Fly>, sqlx::Error> {
        sqlx::query_as(
            "select *
            from fly
            where fly_id = $1",
        )
        .bind(id)
        .fetch_optional(&mut **self.tx)
        .await
    }
    pub async fn create_fly(&mut self, form: model::FlyForm) -> Result<model::Fly, sqlx::Error> {
        sqlx::query_as(
            "insert into fly (luxus_seter, flex_seter, billig_seter)
            values ($1, $2, $3)",
        )
        .bind(form.luxus_seter)
        .bind(form.flex_seter)
        .bind(form.billig_seter)
        .fetch_one(&mut **self.tx)
        .await
    }
}

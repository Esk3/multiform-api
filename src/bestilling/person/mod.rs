use crate::ApiTags;

use model::PersonId;
use poem_openapi::{
    param::Cookie,
    payload::{Json, PlainText},
    ApiRequest, ApiResponse, Object, OpenApi,
};
use std::sync::Arc;

pub mod model;
pub mod query;

#[derive(ApiResponse)]
enum InsertPersonResult {
    #[oai(status = 200)]
    Ok(Json<model::PersonId>),
    #[oai(status = 500)]
    Err,
}

#[derive(ApiResponse)]
enum InsertBestillingPersonResult {
    #[oai(status = 200)]
    Ok(Json<model::BestillingPerson>),
    #[oai(status = 500)]
    Err,
}

pub struct PersonApi {
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[OpenApi(prefix_path = "/v1/person", tag = "ApiTags::Person")]
impl PersonApi {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText("hello persons".to_string())
    }
    #[oai(path = "/", method = "post")]
    async fn ny_person(&self, Json(person): Json<model::PersonForm>) -> InsertPersonResult {
        match query::PersonQuery::new(self.pool.clone())
            .insert_person(&person)
            .await
        {
            Ok(id) => InsertPersonResult::Ok(Json(id)),
            Err(e) => {
                dbg!(e);
                InsertPersonResult::Err
            }
        }
    }
    #[oai(path = "/bestilling_person", method = "post")]
    async fn json_bestilling_person(
        &self,
        Json(bestilling_person): Json<model::BestillingPerson>,
    ) -> InsertBestillingPersonResult {
        match query::PersonQuery::new(self.pool.clone())
            .insert_bestilling_person(bestilling_person.bestilling_id, bestilling_person.person_id)
            .await
        {
            Ok(bestilling_person) => InsertBestillingPersonResult::Ok(Json(bestilling_person)),
            Err(e) => {
                dbg!(e);
                InsertBestillingPersonResult::Err
            }
        }
    }
    #[oai(path = "/cookie_bestilling_person", method = "post")]
    async fn bestilling_person(
        &self,
        Cookie(bestilling_id): Cookie<i32>,
        Json(person_id): Json<PersonId>,
    ) -> InsertBestillingPersonResult {
        dbg!(bestilling_id, person_id.id);
        match query::PersonQuery::new(self.pool.clone())
            .insert_bestilling_person(bestilling_id, person_id.id)
            .await
        {
            Ok(bestilling_person) => InsertBestillingPersonResult::Ok(Json(bestilling_person)),
            Err(e) => {
                dbg!(e);
                InsertBestillingPersonResult::Err
            }
        }
    }
}

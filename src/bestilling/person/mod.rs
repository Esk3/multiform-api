use crate::{ApiTags, BestillingsId};

use model::{BestillingPerson, PersonId};
use poem_openapi::{
    param::{Cookie, Header, Query},
    payload::{Form, Json, PlainText},
    ApiRequest, ApiResponse, OpenApi,
};
use std::sync::Arc;

pub mod model;
pub mod query;

#[derive(ApiRequest)]
enum CreatePersonAndBestillingRequest {
    Json(Json<model::PersonForm>),
    Form(Form<model::PersonForm>),
}

#[derive(ApiResponse)]
enum CreatePersonAndBestillingResponse {
    #[oai(status = 201)]
    Ok(Json<BestillingPerson>),
    #[oai(status = 500)]
    Err,
}

#[derive(ApiResponse)]
enum CreateBestillingWithPersonResponse {
    #[oai(status = 201)]
    Ok(Json<BestillingPerson>),
    #[oai(status = 404)]
    PersonNotFound,
    #[oai(status = 500)]
    Err,
}

#[derive(ApiResponse)]
enum CreatePersonResult {
    #[oai(status = 201)]
    Ok(Json<model::PersonId>),
    #[oai(status = 500)]
    Err,
}

#[derive(ApiResponse)]
enum CreateBestillingPersonResult {
    #[oai(status = 201)]
    Ok(
        Json<model::BestillingPerson>,
        #[oai(header = "Set-Cookie")] String,
    ),
    #[oai(status = 500)]
    Err,
}

pub struct PersonApi {
    pub pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

#[OpenApi(prefix_path = "/v1/person", tag = "ApiTags::Person")]
impl PersonApi {
    #[oai(path = "/", method = "post")]
    async fn ny_person(&self, Json(person): Json<model::PersonForm>) -> CreatePersonResult {
        match query::PersonQuery::new(self.pool.clone())
            .insert_person(&person)
            .await
        {
            Ok(id) => CreatePersonResult::Ok(Json(id)),
            Err(e) => {
                dbg!(e);
                CreatePersonResult::Err
            }
        }
    }
    #[oai(path = "/json_bestilling_person", method = "post")]
    async fn json_bestilling_person(
        &self,
        Json(bestilling_person): Json<model::BestillingPerson>,
    ) -> CreateBestillingPersonResult {
        let bestilling_id = bestilling_person.bestilling_id;
        match query::PersonQuery::new(self.pool.clone())
            .insert_bestilling_person(bestilling_person.bestilling_id, bestilling_person.person_id)
            .await
        {
            Ok(bestilling_person) => CreateBestillingPersonResult::Ok(
                Json(bestilling_person),
                format!("bestilling_id={bestilling_id}"),
            ),
            Err(e) => {
                dbg!(e);
                CreateBestillingPersonResult::Err
            }
        }
    }
    #[oai(path = "/bestilling_person", method = "post")]
    async fn bestilling_person(
        &self,
        Cookie(bestilling_id): Cookie<Option<i32>>,
        Json(person_id): Json<PersonId>,
    ) -> CreateBestillingPersonResult {
        dbg!(bestilling_id, person_id.id);
        let Ok(bestilling_id) = BestillingsId::new(bestilling_id)
            .get_or_create(self.pool.clone())
            .await
        else {
            return CreateBestillingPersonResult::Err;
        };
        match query::PersonQuery::new(self.pool.clone())
            .insert_bestilling_person(bestilling_id, person_id.id)
            .await
        {
            Ok(bestilling_person) => CreateBestillingPersonResult::Ok(
                Json(bestilling_person),
                format!("bestilling_id={bestilling_id}"),
            ),
            Err(e) => {
                dbg!(e);
                CreateBestillingPersonResult::Err
            }
        }
    }
    #[oai(path = "/create/new/person", method = "post")]
    async fn create_person_and_bestilling(
        &self,
        req: CreatePersonAndBestillingRequest,
        Cookie(bestilling_id): Cookie<Option<i32>>,
    ) -> CreatePersonAndBestillingResponse {
        let query = query::PersonQuery::new(self.pool.clone());
        let person = match req {
            CreatePersonAndBestillingRequest::Json(Json(person))
            | CreatePersonAndBestillingRequest::Form(Form(person)) => person,
        };
        let Ok(person_id) = query.insert_person(&person).await else {
            return CreatePersonAndBestillingResponse::Err;
        };
        let Ok(bestilling_id) = BestillingsId::new(bestilling_id)
            .get_or_create(self.pool.clone())
            .await
        else {
            return CreatePersonAndBestillingResponse::Err;
        };

        let Ok(bestilling_person) = query
            .insert_bestilling_person(bestilling_id, person_id.id)
            .await
        else {
            return CreatePersonAndBestillingResponse::Err;
        };
        CreatePersonAndBestillingResponse::Ok(Json(bestilling_person))
    }
    #[oai(path = "/create/with/person", method = "post")]
    async fn create_bestilling_with_person(
        &self,
        Cookie(bestilling_id): Cookie<Option<i32>>,
        Form(person_id): Form<i32>,
    ) -> CreateBestillingWithPersonResponse {
        let query = query::PersonQuery::new(self.pool.clone());
        let Ok(person) = query.get_fra_id(person_id).await else {
            return CreateBestillingWithPersonResponse::PersonNotFound;
        };
        let Ok(bestilling_id) = BestillingsId::new(bestilling_id)
            .get_or_create(self.pool.clone())
            .await
        else {
            return CreateBestillingWithPersonResponse::Err;
        };
        let Ok(bestilling_person) = query
            .insert_bestilling_person(bestilling_id, person.id)
            .await
        else {
            return CreateBestillingWithPersonResponse::Err;
        };
        CreateBestillingWithPersonResponse::Ok(Json(bestilling_person))
    }
}

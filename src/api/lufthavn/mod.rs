use crate::ApiTags;

use model::SearchQuery;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    ApiResponse, OpenApi,
};
use std::sync::Arc;

mod model;
mod query;

#[allow(clippy::large_enum_variant)]
#[derive(ApiResponse)]
enum LufthavnFraIataCodeResponse {
    #[oai(status = 200)]
    Ok(Json<model::Lufthavn>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    Err,
}

#[derive(ApiResponse)]
enum SearchResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<model::Lufthavn>>),
    #[oai(status = 500)]
    Err,
}

pub struct LufthavnApi {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl LufthavnApi {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
}

#[OpenApi(prefix_path = "/v1/lufthavn", tag = "ApiTags::Lufthavn")]
impl LufthavnApi {
    #[oai(path = "/:iata_code", method = "get")]
    async fn get_by_iata_code(
        &self,
        /// eg "BGO" https://en.wikipedia.org/wiki/International_Air_Transport_Association
        #[oai(validator(min_length = 3, max_length = 4))]
        Path(iata_code): Path<String>,
    ) -> LufthavnFraIataCodeResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match query::Query::new(&mut tx).get_by_iata_code(iata_code).await {
            Ok(Some(lufthavn)) => LufthavnFraIataCodeResponse::Ok(Json(lufthavn)),
            Ok(None) => LufthavnFraIataCodeResponse::NotFound,
            Err(e) => {
                dbg!(e);
                LufthavnFraIataCodeResponse::Err
            }
        }
    }
    #[oai(path = "/search", method = "get")]
    #[allow(clippy::too_many_arguments)]
    async fn search(
        &self,
        /// https://en.wikipedia.org/wiki/International_Air_Transport_Association
        #[oai(validator(min_length = 3, max_length = 4))]
        Query(iata_code): Query<Option<String>>,
        Query(airport_type): Query<Option<model::AirportType>>,
        Query(name): Query<Option<String>>,
        Query(name_exact): Query<Option<bool>>,
        Query(elevation_ft): Query<Option<f32>>,
        Query(elevation_ft_greater_then): Query<Option<f32>>,
        Query(elevation_ft_less_then): Query<Option<f32>>,
        Query(continent): Query<Option<model::Continent>>,
        /// eg "NO" | "DK" | "UK" | ...
        /// https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes
        #[oai(validator(min_length = 2, max_length = 2))]
        Query(iso_country): Query<Option<String>>,
        Query(iso_region): Query<Option<String>>,
        /// By navn
        Query(municipality): Query<Option<String>>,
        Query(municipality_exact): Query<Option<bool>>,
        Query(gps_code): Query<Option<String>>,
        Query(local_code): Query<Option<String>>,
        Query(coordinates): Query<Option<String>>,
        Query(limit): Query<Option<i32>>,
    ) -> SearchResponse {
        let mut tx = self.pool.begin().await.unwrap();
        match query::Query::new(&mut tx)
            .search(SearchQuery {
                iata_code,
                airport_type,
                name,
                name_exact: name_exact.unwrap_or(false),
                elevation_ft,
                elevation_ft_greater_then,
                elevation_ft_less_then,
                continent,
                iso_country,
                iso_region,
                municipality,
                municipality_exact: municipality_exact.unwrap_or(false),
                gps_code,
                local_code,
                coordinates,
                limit,
            })
            .await
        {
            Ok(rows) => SearchResponse::Ok(Json(rows)),
            Err(e) => {
                dbg!(e);
                SearchResponse::Err
            }
        }
    }
}

use std::sync::Arc;

use super::model::{Lufthavn, SearchQuery};

pub struct Query {
    pool: Arc<sqlx::Pool<sqlx::Postgres>>,
}

impl Query {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Postgres>>) -> Self {
        Self { pool }
    }
    pub async fn get_by_iata_code(
        &self,
        iata_code: String,
    ) -> Result<Option<Lufthavn>, sqlx::Error> {
        sqlx::query_as(
            "select *
            from lufthavner
            where iata_code = upper($1)",
        )
        .bind(iata_code)
        .fetch_optional(&*self.pool)
        .await
    }
    pub async fn search(&self, search_query: SearchQuery) -> Result<Vec<Lufthavn>, sqlx::Error> {
        sqlx::query_as(
            "
            select *
            from lufthavner
            where ( iata_code = upper($1) or $1 is null )
            and ( airport_type = $2 or $2 is null )
            and ( case when $3 then name = $4 else lower(name) like '%' || lower($4) || '%' end or $4 is null )
            and ( elevation_ft = $5 or $5 is null )
            and ( elevation_ft < $6 or $6 is null )
            and ( elevation_ft > $7 or $7 is null )
            and ( continent = upper($8) or $8 is null )
            and ( iso_country = upper($9) or $9 is null )
            and ( iso_region = upper($10) or $10 is null )
            and ( case when $11 then municipality = $12 else lower(municipality) like '%' || lower($12) || '%' end or $12 is null )
            and ( gps_code = $13 or $13 is null )
            and ( local_code = $14 or $14 is null )
            and ( coordinates = $15 or $15 is null )
            limit $16
            ",
        )
        .bind(search_query.iata_code)
        .bind(search_query.airport_type)
        .bind(search_query.name_exact)
        .bind(search_query.name)
        .bind(search_query.elevation_ft)
        .bind(search_query.elevation_ft_greater_then)
        .bind(search_query.elevation_ft_less_then)
        .bind(search_query.continent)
        .bind(search_query.iso_country)
        .bind(search_query.iso_region)
        .bind(search_query.municipality_exact)
        .bind(search_query.municipality)
        .bind(search_query.gps_code)
        .bind(search_query.local_code)
        .bind(search_query.coordinates)
        .bind(search_query.limit.unwrap_or(10).min(100))
        .fetch_all(&*self.pool)
        .await
    }
}

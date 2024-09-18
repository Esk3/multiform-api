use poem_openapi::Object;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Object)]
pub struct Lufthavn {
    /// https://en.wikipedia.org/wiki/International_Air_Transport_Association
    iata_code: String,
    airport_type: String,
    name: String,
    elevation_ft: Option<f32>,
    continent: String,
    /// https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes
    iso_country: String,
    iso_region: String,
    municipality: String,
    gps_code: Option<String>,
    local_code: Option<String>,
    coordinates: String,
}

pub struct SearchQuery {
    /// https://en.wikipedia.org/wiki/International_Air_Transport_Association
    pub iata_code: Option<String>,
    pub airport_type: Option<String>,
    pub name: Option<String>,
    pub name_exact: bool,
    pub elevation_ft: Option<f32>,
    pub elevation_ft_greater_then: Option<f32>,
    pub elevation_ft_less_then: Option<f32>,
    pub continent: Option<String>,
    /// https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes
    pub iso_country: Option<String>,
    pub iso_region: Option<String>,
    pub municipality: Option<String>,
    pub municipality_exact: bool,
    pub gps_code: Option<String>,
    pub local_code: Option<String>,
    pub coordinates: Option<String>,
    pub limit: Option<i32>
}

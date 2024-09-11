use std::str::FromStr;

#[must_use]
pub fn new_header(field: &str, value: &str) -> tiny_http::Header {
    tiny_http::Header::from_str(&format!("{field}: {value}")).unwrap()
}

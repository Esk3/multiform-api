use std::fs::File;
use tiny_http::Response;

pub trait IntoResponse {
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox;
}

impl IntoResponse for tiny_http::Response<File> {
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        let data_length = self.data_length();
        let data = self.into_reader();
        tiny_http::ResponseBox::new(200.into(), Vec::new(), Box::new(data), data_length, None)
    }
}

impl IntoResponse for Response<std::io::Cursor<Vec<u8>>> {
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        let status_code = self.status_code();
        let headers = self.headers().to_vec();
        let data_len = self.data_length();
        let reader = self.into_reader();
        tiny_http::ResponseBox::new(status_code, headers, Box::new(reader), data_len, None)
    }
}

impl IntoResponse for tiny_http::ResponseBox {
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        *self
    }
}

impl IntoResponse for () {
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        tiny_http::ResponseBox::new(
            200.into(),
            Vec::new(),
            Box::new(std::io::empty()),
            Some(0),
            None,
        )
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        match *self {
            Ok(t) => Box::new(t).into_response(),
            Err(e) => Box::new(e).into_response().with_status_code(500),
        }
    }
}

impl<T> IntoResponse for Option<T>
where
    T: IntoResponse,
{
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        match *self {
            Some(t) => Box::new(t).into_response(),
            None => Box::new((tiny_http::StatusCode(404), "Not found")).into_response(),
        }
    }
}

impl IntoResponse for String {
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        let len = Some(self.len());
        tiny_http::ResponseBox::new(
            200.into(),
            Vec::new(),
            Box::new(std::io::Cursor::new(*self)),
            len,
            None,
        )
    }
}

impl IntoResponse for &str {
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        tiny_http::ResponseBox::new(
            200.into(),
            Vec::new(),
            Box::new(std::io::Cursor::new(self.to_string())),
            Some(self.len()),
            None,
        )
    }
}

impl<R> IntoResponse for (tiny_http::StatusCode, R)
where
    R: IntoResponse,
{
    fn into_response(self: Box<Self>) -> tiny_http::ResponseBox {
        let (status_code, response) = *self;
        Box::new(response)
            .into_response()
            .with_status_code(status_code)
    }
}

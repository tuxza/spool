// i made this because I didnt want to use Response<Body>>
// ...
// is that a dumb reason?
// yes.

use axum::http::StatusCode;

impl From<StatusCode> for ErrorStatus {
    fn from(code: StatusCode) -> Self {
        Self { code, message: "" }
    }
}

impl From<(StatusCode, &'static str)> for ErrorStatus {
    fn from((code, message): (StatusCode, &'static str)) -> Self {
        Self { code, message }
    }
}

#[derive(Debug)]
pub struct ErrorStatus {
    code: StatusCode,
    message: &'static str,
}

impl axum::response::IntoResponse for ErrorStatus {
    fn into_response(self) -> axum::response::Response {
        (self.code, self.message).into_response()
    }
}

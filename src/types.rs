// i made this because I didnt want to use Response<Body>>
// ...
// is that a dumb reason?
// yes.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl From<StatusCode> for ErrorStatus {
    fn from(code: StatusCode) -> Self {
        Self {
            code,
            message: "no error message provided",
        }
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

impl IntoResponse for ErrorStatus {
    fn into_response(self) -> Response {
        println!("ERROR: {} {}", self.code, self.message);

        (self.code, self.message).into_response()
    }
}

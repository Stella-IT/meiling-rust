use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum OAuth2Error {
    #[display(fmt = "Client ID Not Found")]
    ClientIdIsNone,
    #[display(fmt = "Client Secret Not Found")]
    ClientSecretIsNone,
    #[display(fmt = "Unknown Error")]
    Unknown,
}

impl error::ResponseError for OAuth2Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            OAuth2Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
}

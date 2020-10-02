use actix_web::error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum OAuth2Error {
    #[display(fmt = "Client ID Not Found")]
    ClientIdIsNone,
    #[display(fmt = "Unknown Error")]
    Unknown,
}

impl error::ResponseError for OAuth2Error {}

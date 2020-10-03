use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct NewTokenRequest {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewTokenResponse {
    pub token: String,
}

#[derive(Debug, Error)]
pub enum OAuth2Error {
    #[error("Client not found or secret is wrong")]
    ClientIsNone,
    #[error("Unknown error")]
    Unknown,
}

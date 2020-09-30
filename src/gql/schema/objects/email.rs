use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::meiling::objects::email as meiling;

#[derive(GraphQLObject)]
pub struct Email {
    pub id: String,
    pub address: String,
    pub user_id: String,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: bool,
}

impl From<meiling::Email> for Email {
    fn from(email: meiling::Email) -> Self {
        Self {
            id: email.id.to_string(),
            address: email.address,
            user_id: email.user_id.to_string(),
            registration_date: email.registration_date,
            is_validated: email.is_validated,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewEmail {
    pub address: String,
    pub user_id: String,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: bool,
}

impl TryFrom<NewEmail> for meiling::NewEmail {
    type Error = Box<dyn Error>;

    fn try_from(email: NewEmail) -> Result<Self, Self::Error> {
        Ok(Self {
            address: email.address,
            user_id: Uuid::from_str(&email.user_id.as_str())?,
            registration_date: email.registration_date,
            is_validated: email.is_validated,
        })
    }
}

use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::database::model;

#[derive(GraphQLObject)]
pub struct Email {
    pub id: String,
    pub address: String,
    pub user_id: String,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: bool,
}

impl TryFrom<model::Email> for Email {
    type Error = Box<dyn Error>;

    fn try_from(email: model::Email) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&String::from_utf8(email.id)?)?.to_string(),
            address: email.address,
            user_id: Uuid::from_str(&String::from_utf8(email.user_id)?)?.to_string(),
            registration_date: email.registration_date,
            is_validated: match email.is_validated {
                0 => false,
                _ => true,
            },
        })
    }
}

#[derive(GraphQLInputObject)]
pub struct NewEmail {
    pub address: String,
    pub user_id: String,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: bool,
}

impl From<NewEmail> for model::NewEmail {
    fn from(email: NewEmail) -> Self {
        Self {
            address: email.address,
            user_id: email.user_id.into_bytes(),
            registration_date: email.registration_date,
            is_validated: match email.is_validated {
                true => 1,
                false => 0,
            },
        }
    }
}

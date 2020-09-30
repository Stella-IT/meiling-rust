use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::database::model;

pub struct Email {
    pub id: Uuid,
    pub address: String,
    pub user_id: Uuid,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: bool,
}

impl TryFrom<model::Email> for Email {
    type Error = Box<dyn Error>;

    fn try_from(email: model::Email) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&String::from_utf8(email.id)?)?,
            address: email.address,
            user_id: Uuid::from_str(&String::from_utf8(email.user_id)?)?,
            registration_date: email.registration_date,
            is_validated: match email.is_validated {
                0 => false,
                _ => true,
            },
        })
    }
}

pub struct NewEmail {
    pub address: String,
    pub user_id: Uuid,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: bool,
}

impl From<NewEmail> for model::NewEmail {
    fn from(email: NewEmail) -> Self {
        Self {
            address: email.address,
            user_id: email.user_id.as_bytes().to_vec(),
            registration_date: email.registration_date,
            is_validated: match email.is_validated {
                true => 1,
                false => 0,
            },
        }
    }
}

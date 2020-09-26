use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::database::model;

#[derive(GraphQLObject)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub user_id: String,
    pub creation_date: NaiveDateTime,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

impl TryFrom<model::User> for User {
    type Error = Box<dyn Error>;

    fn try_from(user: model::User) -> Result<Self, Self::Error> {
        let value = Self {
            uuid: Uuid::from_str(&String::from_utf8(user.id.clone())?)?.to_string(),
            name: user.name,
            user_id: user.user_id,
            creation_date: user.creation_date,
            image_url: user.image_url,
            gender: user.gender,
        };
        Ok(value)
    }
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub user_id: String,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

impl From<NewUser> for model::NewUser {
    fn from(new_user: NewUser) -> Self {
        model::NewUser {
            name: new_user.name,
            user_id: new_user.user_id,
            image_url: new_user.image_url,
            gender: new_user.gender,
        }
    }
}

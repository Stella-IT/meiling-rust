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

impl User {
    pub fn from_model(user: &model::User) -> Result<Self, Box<dyn Error>> {
        let uuid: Uuid = Uuid::from_str(&String::from_utf8(user.id.clone())?)?;
        Ok(Self {
            uuid: uuid.to_string(),
            name: user.name.clone(),
            user_id: user.user_id.clone(),
            creation_date: user.creation_date,
            image_url: user.image_url.clone(),
            gender: user.gender.clone(),
        })
    }
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub user_id: String,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

impl NewUser {
    pub fn into_model(self) -> model::NewUser {
        model::NewUser {
            name: self.name,
            user_id: self.user_id,
            image_url: self.image_url,
            gender: self.gender,
        }
    }

    pub fn to_model(&self) -> model::NewUser {
        model::NewUser {
            name: self.name.clone(),
            user_id: self.user_id.clone(),
            image_url: self.image_url.clone(),
            gender: self.gender.clone(),
        }
    }
}

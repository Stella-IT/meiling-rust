use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::meiling::objects::user as meiling;

#[derive(GraphQLObject)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub user_id: String,
    pub creation_date: NaiveDateTime,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

impl From<meiling::User> for User {
    fn from(user: meiling::User) -> Self {
        Self {
            uuid: user.uuid.to_string(),
            name: user.name,
            user_id: user.user_id,
            creation_date: user.creation_date,
            image_url: user.image_url,
            gender: user.gender,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
    pub user_id: String,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

impl From<NewUser> for meiling::NewUser {
    fn from(new_user: NewUser) -> Self {
        meiling::NewUser {
            name: new_user.name,
            user_id: new_user.user_id,
            image_url: new_user.image_url,
            gender: new_user.gender,
        }
    }
}

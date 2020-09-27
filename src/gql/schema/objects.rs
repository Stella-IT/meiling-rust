use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use chrono::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::database::{enums, model};
use crate::gql::schema::enums::AuthenticationMethod;

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

#[derive(GraphQLObject)]
pub struct AuthenticationInfo {
    pub uuid: String,
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: String,
}

impl TryFrom<model::AuthenticationInfo> for AuthenticationInfo {
    type Error = Box<dyn Error>;

    fn try_from(authentication_info: model::AuthenticationInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: Uuid::from_str(&String::from_utf8(authentication_info.id.clone())?)?.to_string(),
            auth_method: match authentication_info.auth_method {
                enums::AuthenticationMethod::Password => AuthenticationMethod::Password,
                enums::AuthenticationMethod::Pubkey => AuthenticationMethod::Pubkey,
                enums::AuthenticationMethod::OneTimePassword => {
                    AuthenticationMethod::OneTimePassword
                }
            },
            key: authentication_info.key,
            name: authentication_info.name,
            user_id: Uuid::from_str(&String::from_utf8(authentication_info.user_id.clone())?)?
                .to_string(),
        })
    }
}

#[derive(GraphQLInputObject)]
pub struct NewAuthenticationInfo {
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: String,
}

impl From<NewAuthenticationInfo> for model::NewAuthenticationInfo {
    fn from(new_authentication_info: NewAuthenticationInfo) -> Self {
        Self {
            auth_method: match new_authentication_info.auth_method {
                AuthenticationMethod::Password => enums::AuthenticationMethod::Password,
                AuthenticationMethod::Pubkey => enums::AuthenticationMethod::Pubkey,
                AuthenticationMethod::OneTimePassword => {
                    enums::AuthenticationMethod::OneTimePassword
                }
            },
            key: new_authentication_info.key,
            name: new_authentication_info.name,
            user_id: new_authentication_info.user_id.into_bytes(),
        }
    }
}

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
            id: uuid::Uuid::from_str(&String::from_utf8(email.id)?)?.to_string(),
            address: email.address,
            user_id: uuid::Uuid::from_str(&String::from_utf8(email.user_id)?)?.to_string(),
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

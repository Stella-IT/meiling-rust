use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use uuid::Uuid;

use crate::database::{database_enums, model};

use super::super::enums::AuthenticationMethod;

pub struct AuthenticationInfo {
    pub uuid: Uuid,
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: Uuid,
}

impl TryFrom<model::AuthenticationInfo> for AuthenticationInfo {
    type Error = Box<dyn Error>;

    fn try_from(authentication_info: model::AuthenticationInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: Uuid::from_str(&String::from_utf8(authentication_info.id.clone())?)?,
            auth_method: match authentication_info.auth_method {
                database_enums::AuthenticationMethod::Password => AuthenticationMethod::Password,
                database_enums::AuthenticationMethod::Pubkey => AuthenticationMethod::Pubkey,
                database_enums::AuthenticationMethod::OneTimePassword => {
                    AuthenticationMethod::OneTimePassword
                }
            },
            key: authentication_info.key,
            name: authentication_info.name,
            user_id: Uuid::from_str(&String::from_utf8(authentication_info.user_id.clone())?)?,
        })
    }
}

pub struct NewAuthenticationInfo {
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: Uuid,
}

impl From<NewAuthenticationInfo> for model::NewAuthenticationInfo {
    fn from(new_authentication_info: NewAuthenticationInfo) -> Self {
        Self {
            auth_method: match new_authentication_info.auth_method {
                AuthenticationMethod::Password => database_enums::AuthenticationMethod::Password,
                AuthenticationMethod::Pubkey => database_enums::AuthenticationMethod::Pubkey,
                AuthenticationMethod::OneTimePassword => {
                    database_enums::AuthenticationMethod::OneTimePassword
                }
            },
            key: new_authentication_info.key,
            name: new_authentication_info.name,
            user_id: new_authentication_info.user_id.as_bytes().to_vec(),
        }
    }
}

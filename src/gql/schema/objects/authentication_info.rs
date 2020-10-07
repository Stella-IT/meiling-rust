use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::gql::schema::enums::AuthenticationMethod;
use crate::meiling::{enums as meiling_enums, objects::authentication_info as meiling};

#[derive(GraphQLObject)]
pub struct AuthenticationInfo {
    pub uuid: String,
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: String,
}

impl From<meiling::AuthenticationInfo> for AuthenticationInfo {
    fn from(authentication_info: meiling::AuthenticationInfo) -> Self {
        Self {
            uuid: authentication_info.uuid.to_string(),
            auth_method: match authentication_info.auth_method {
                meiling_enums::AuthenticationMethod::Password => AuthenticationMethod::Password,
                meiling_enums::AuthenticationMethod::Pubkey => AuthenticationMethod::Pubkey,
                meiling_enums::AuthenticationMethod::OneTimePassword => {
                    AuthenticationMethod::OneTimePassword
                }
                meiling_enums::AuthenticationMethod::Fido2 => AuthenticationMethod::Fido2,
            },
            key: authentication_info.key,
            name: authentication_info.name,
            user_id: authentication_info.user_id.to_string(),
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewAuthenticationInfo {
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: String,
}

impl TryFrom<NewAuthenticationInfo> for meiling::NewAuthenticationInfo {
    type Error = Box<dyn Error>;
    fn try_from(new_authentication_info: NewAuthenticationInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            auth_method: match new_authentication_info.auth_method {
                AuthenticationMethod::Password => meiling_enums::AuthenticationMethod::Password,
                AuthenticationMethod::Pubkey => meiling_enums::AuthenticationMethod::Pubkey,
                AuthenticationMethod::OneTimePassword => {
                    meiling_enums::AuthenticationMethod::OneTimePassword
                }
                AuthenticationMethod::Fido2 => meiling_enums::AuthenticationMethod::Fido2,
            },
            key: new_authentication_info.key,
            name: new_authentication_info.name,
            user_id: Uuid::from_str(&new_authentication_info.user_id.as_str())?,
        })
    }
}

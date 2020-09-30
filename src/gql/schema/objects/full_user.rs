use super::*;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
pub struct FullUserCreationResult {
    pub user: user::User,
    pub emails: Vec<email::Email>,
    pub authentication_info: Vec<authentication_info::AuthenticationInfo>,
}

#[derive(GraphQLInputObject, Clone)]
pub struct NewFullUser {
    pub name: String,
    pub user_id: String,
    pub image_url: Option<String>,
    pub gender: Option<String>,
    pub email_address: String,
    pub default_password: String,
}

impl From<NewFullUser> for user::NewUser {
    fn from(new_full_user: NewFullUser) -> Self {
        Self {
            name: new_full_user.name,
            user_id: new_full_user.user_id,
            image_url: new_full_user.image_url,
            gender: new_full_user.gender,
        }
    }
}

use std::convert::TryFrom;

use juniper::FieldResult;

use crate::meiling;
use crate::meiling::user::{add_authentication_method, add_email, create_user};

use super::super::{context::Context, schema::objects::*};

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, new_user: user::NewUser) -> FieldResult<user::User> {
        let conn = context.database_pool.get()?;
        Ok(user::User::from(meiling::user::create_user(
            &conn,
            new_user.into(),
        )?))
    }

    fn add_authentication_method(
        context: &Context,
        new_authentication_info: authentication_info::NewAuthenticationInfo,
    ) -> FieldResult<Vec<authentication_info::AuthenticationInfo>> {
        let conn = context.database_pool.get()?;
        Ok(meiling::user::add_authentication_method(
            &conn,
            meiling::objects::authentication_info::NewAuthenticationInfo::try_from(
                new_authentication_info,
            )?,
        )?
        .into_iter()
        .map(|e| authentication_info::AuthenticationInfo::from(e))
        .collect())
    }

    fn add_email(context: &Context, new_email: email::NewEmail) -> FieldResult<Vec<email::Email>> {
        let conn = context.database_pool.get()?;
        Ok(meiling::user::add_email(
            &conn,
            meiling::objects::email::NewEmail::try_from(email::NewEmail::try_from(new_email)?)?,
        )?
        .into_iter()
        .map(|e| e.into())
        .collect())
    }

    fn create_full_user(
        context: &Context,
        new_full_user: full_user::NewFullUser,
    ) -> FieldResult<full_user::FullUserCreationResult> {
        let conn = context.database_pool.get()?;

        let user = create_user(&conn, user::NewUser::from(new_full_user.clone()).into())?;
        let default_email = new_full_user.email_address;
        let default_password = new_full_user.default_password;

        let email = add_email(
            &conn,
            meiling::objects::email::NewEmail {
                address: default_email,
                user_id: user.uuid.clone(),
                registration_date: None,
                is_validated: false,
            },
        )?;

        let authentication_info = add_authentication_method(
            &conn,
            meiling::objects::authentication_info::NewAuthenticationInfo {
                auth_method: meiling::enums::AuthenticationMethod::Password,
                key: default_password,
                name: "Initialized Password".into(),
                user_id: user.uuid.clone(),
            },
        )?;

        Ok(full_user::FullUserCreationResult {
            user: user.into(),
            emails: email.into_iter().map(|e| e.into()).collect(),
            authentication_info: authentication_info.into_iter().map(|e| e.into()).collect(),
        })
    }
}

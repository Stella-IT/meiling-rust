use std::convert::{TryFrom, TryInto};

use juniper::FieldResult;

use crate::meiling;

use super::super::{context::Context, schema::objects::*};

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, new_user: user::NewUser) -> FieldResult<user::User> {
        let conn = context.database_pool.get()?;
        Ok(user::User::from(meiling::user::create_user(
            &conn,
            new_user.try_into()?,
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
}

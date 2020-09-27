use juniper::FieldResult;

use crate::meiling;

use super::super::{context::Context, schema::objects};

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(
        context: &Context,
        new_user: objects::user::NewUser,
    ) -> FieldResult<objects::user::User> {
        let conn = context.database_pool.get()?;
        Ok(meiling::user::create_user(&conn, new_user)?)
    }

    fn add_authentication_method(
        context: &Context,
        new_authentication_info: objects::authentication_info::NewAuthenticationInfo,
    ) -> FieldResult<Vec<objects::authentication_info::AuthenticationInfo>> {
        let conn = context.database_pool.get()?;
        Ok(meiling::user::add_authentication_method(
            &conn,
            new_authentication_info,
        )?)
    }

    fn add_email(
        context: &Context,
        new_email: objects::email::NewEmail,
    ) -> FieldResult<Vec<objects::email::Email>> {
        let conn = context.database_pool.get()?;
        Ok(meiling::user::add_email(&conn, new_email)?)
    }
}

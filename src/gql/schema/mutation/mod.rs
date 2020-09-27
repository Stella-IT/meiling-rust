use juniper::FieldResult;

use crate::gql::schema::objects::NewAuthenticationInfo;
use crate::meiling;

use super::super::{context::Context, schema::objects};

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, new_user: objects::NewUser) -> FieldResult<objects::User> {
        let conn = context.database_pool.get()?;
        Ok(meiling::create_user(&conn, new_user)?)
    }

    fn add_authentication_method(
        context: &Context,
        new_authentication_info: NewAuthenticationInfo,
    ) -> FieldResult<Vec<objects::AuthenticationInfo>> {
        let conn = context.database_pool.get()?;
        Ok(meiling::add_authentication_method(
            &conn,
            new_authentication_info,
        )?)
    }
}

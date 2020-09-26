use std::convert::TryFrom;

use juniper::FieldResult;

use crate::database::model;

use super::super::{context::Context, schema::objects};

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, new_user: objects::NewUser) -> FieldResult<objects::User> {
        let conn = context.database_pool.get()?;
        let inserted_user: model::User = {
            use crate::database::schema::user::dsl::*;
            use diesel::prelude::*;

            let new_user_id = new_user.user_id.clone();

            diesel::insert_into(user)
                .values(model::NewUser::from(new_user))
                .execute(&conn)?;
            user.filter(user_id.eq(new_user_id)).get_result(&conn)
        }?;
        Ok(objects::User::try_from(inserted_user)?)
    }
}

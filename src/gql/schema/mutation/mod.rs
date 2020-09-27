use std::convert::TryFrom;
use std::str::FromStr;

use diesel::RunQueryDsl;
use juniper::FieldResult;
use uuid::Uuid;

use crate::database::model;
use crate::gql::schema::objects::NewAuthenticationInfo;

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

    fn add_authentication_method(
        context: &Context,
        new_authentication_info: NewAuthenticationInfo,
    ) -> FieldResult<Vec<objects::AuthenticationInfo>> {
        let conn = context.database_pool.get()?;
        let user: model::User = {
            use crate::database::schema::user::dsl::*;
            use diesel::prelude::*;

            let binary_user_id = Uuid::from_str(new_authentication_info.user_id.as_str())?
                .to_string()
                .into_bytes();

            user.filter(id.eq(binary_user_id)).get_result(&conn)?
        };

        println!("{:?}", &user);

        {
            use crate::database::schema::auth_info::dsl::*;
            use diesel::prelude::*;

            diesel::insert_into(auth_info)
                .values(model::NewAuthenticationInfo::from(new_authentication_info))
                .execute(&conn)?;
        }

        let authentication_method: Vec<model::AuthenticationInfo> = {
            use crate::database::schema::auth_info::dsl::*;
            use diesel::prelude::*;

            auth_info.filter(user_id.eq(user.id)).get_results(&conn)?
        };

        Ok(authentication_method
            .into_iter()
            .map(|info| objects::AuthenticationInfo::try_from(info).unwrap())
            .collect())
    }
}

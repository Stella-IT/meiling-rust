use std::error::Error;
use std::str::FromStr;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;
use uuid::Uuid;

use crate::database::model;
use crate::gql::schema as gql;
use std::convert::TryFrom;

pub fn create_user(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    new_user: gql::objects::NewUser,
) -> Result<gql::objects::User, Box<dyn Error>> {
    let inserted_user: model::User = {
        use crate::database::schema::user::dsl::*;
        use diesel::prelude::*;

        let new_user_id = new_user.user_id.clone();

        diesel::insert_into(user)
            .values(model::NewUser::from(new_user))
            .execute(conn)?;
        user.filter(user_id.eq(new_user_id)).get_result(conn)
    }?;
    Ok(gql::objects::User::try_from(inserted_user)?)
}

pub fn add_authentication_method(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    new_authentication_info: gql::objects::NewAuthenticationInfo,
) -> Result<Vec<gql::objects::AuthenticationInfo>, Box<dyn Error>> {
    let user: model::User = {
        use crate::database::schema::user::dsl::*;
        use diesel::prelude::*;

        let binary_user_id = Uuid::from_str(new_authentication_info.user_id.as_str())?
            .to_string()
            .into_bytes();

        user.filter(id.eq(binary_user_id)).get_result(conn)?
    };

    println!("{:?}", &user);

    {
        use crate::database::schema::auth_info::dsl::*;
        use diesel::prelude::*;

        diesel::insert_into(auth_info)
            .values(model::NewAuthenticationInfo::from(new_authentication_info))
            .execute(conn)?;
    }

    let authentication_method: Vec<model::AuthenticationInfo> = {
        use crate::database::schema::auth_info::dsl::*;
        use diesel::prelude::*;

        auth_info.filter(user_id.eq(user.id)).get_results(conn)?
    };

    Ok(authentication_method
        .into_iter()
        .map(|info| gql::objects::AuthenticationInfo::try_from(info).unwrap())
        .collect())
}

use std::convert::TryFrom;
use std::error::Error;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;
use uuid::Uuid;

use crate::database::model;

use super::objects;

pub fn create_user(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    new_user: objects::user::NewUser,
) -> Result<objects::user::User, Box<dyn Error>> {
    let inserted_user: model::User = {
        use crate::database::schema::user::dsl::*;
        use diesel::prelude::*;

        let new_user_id = new_user.user_id.clone();

        diesel::insert_into(user)
            .values(model::NewUser::from(new_user))
            .execute(conn)?;
        user.filter(user_id.eq(new_user_id)).get_result(conn)
    }?;
    Ok(objects::user::User::try_from(inserted_user)?)
}

fn get_user(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    uuid: &Uuid,
) -> Result<model::User, Box<dyn Error>> {
    use crate::database::schema::user::dsl::*;
    use diesel::prelude::*;

    let binary_user_id: Vec<u8> = uuid.to_string().into_bytes();

    Ok(user.filter(id.eq(binary_user_id)).get_result(conn)?)
}

pub fn add_authentication_method(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    new_authentication_info: objects::authentication_info::NewAuthenticationInfo,
) -> Result<Vec<objects::authentication_info::AuthenticationInfo>, Box<dyn Error>> {
    let user: model::User = get_user(conn, &new_authentication_info.user_id)?;

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
        .map(|info| objects::authentication_info::AuthenticationInfo::try_from(info).unwrap())
        .collect())
}

pub fn add_email(
    conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
    new_email: objects::email::NewEmail,
) -> Result<Vec<objects::email::Email>, Box<dyn Error>> {
    let user: model::User = get_user(conn, &new_email.user_id)?;

    {
        use crate::database::schema::email::dsl::*;
        use diesel::prelude::*;

        diesel::insert_into(email)
            .values(model::NewEmail::from(new_email))
            .execute(conn)?;
    }

    let emails: Vec<model::Email> = {
        use crate::database::schema::email::dsl::*;
        use diesel::prelude::*;

        email.filter(user_id.eq(user.id)).get_results(conn)?
    };

    Ok(emails
        .into_iter()
        .map(|info| objects::email::Email::try_from(info).unwrap())
        .collect())
}

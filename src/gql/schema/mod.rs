use diesel::{QueryDsl, RunQueryDsl};
use juniper::FieldResult;
use juniper::RootNode;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

use crate::database::model;
use crate::gql::context::Context;

pub mod enums;
pub mod objects;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }

    fn create_user(context: &Context, new_user: objects::NewUser) -> FieldResult<objects::User> {
        let conn = context.database_pool.get()?;
        let inserted_user: model::User = {
            use crate::database::schema::user::dsl::*;
            use diesel::prelude::*;

            let model = new_user.to_model();

            diesel::insert_into(user).values(model).execute(&conn)?;
            user.filter(user_id.eq(&new_user.user_id)).get_result(&conn)
        }?;
        Ok(objects::User::from_model(&inserted_user)?)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

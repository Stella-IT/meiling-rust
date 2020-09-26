use juniper::RootNode;

use mutation::MutationRoot;
use query::QueryRoot;

pub mod enums;
mod mutation;
pub mod objects;
mod query;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

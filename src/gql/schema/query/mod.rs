use super::super::context::Context;

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {}

use git2::Repository;
use std::path::Path;

mod model;

use model::*;

type Schema = juniper::RootNode<'static, Query, Mutation>;

pub struct Store {
    schema: Schema,
    repo: Repository,
}

pub struct Context {}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn notes(context: &Context) -> juniper::FieldResult<Note> {
        Ok(Note::new())
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {}

impl Store {
    pub fn new(path: &Path) -> Store {
        let repo = match Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        Store {
            schema: Schema::new(Query, Mutation),
            repo,
        }
    }

    pub fn query(&self) -> Result<juniper::Value, juniper::GraphQLError> {
        let variables = juniper::Variables::new();
        self.query_with_variables(&variables)
    }
    pub fn query_with_variables(
        &self,
        variables: &juniper::Variables,
    ) -> Result<juniper::Value, juniper::GraphQLError> {
        let ctx = Context {};
        let res = juniper::execute("query { notes {id} }", None, &self.schema, &variables, &ctx);
        res.and_then(|r| {
            let (v, _e) = r;
            Ok(v)
        })
    }
}

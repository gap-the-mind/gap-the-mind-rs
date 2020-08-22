use git2::Repository;
use std::path::Path;

mod model;
use model::*;

use juniper::{FieldResult, GraphQLError, Value, Variables};

type Schema = juniper::RootNode<'static, Query, Mutation>;

pub struct Store {
    schema: Schema,
    repo: Repository,
    context: StorageContext,
}

pub struct StorageContext {}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for StorageContext {}

pub struct Query;

#[juniper::object(
    Context = StorageContext
)]
impl Query {
    fn notes(ctx: &StorageContext) -> FieldResult<Note> {
        Ok(Note::new())
    }
}

pub struct Mutation;

#[juniper::object(
    Context = StorageContext
)]
impl Mutation {
    fn createNote(ctx: &StorageContext, note_input: NoteInput) -> FieldResult<Note> {
        let note = Note::new();

        Ok(note)
    }
}

impl Store {
    pub fn new(path: &Path, context: StorageContext) -> Store {
        let repo = match Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        Store {
            schema: Schema::new(Query, Mutation),
            repo,
            context,
        }
    }

    pub fn schema(&self) -> Result<Value, GraphQLError> {
        let schema = juniper::introspect(
            &self.schema,
            &self.context,
            juniper::IntrospectionFormat::default(),
        );

        schema.and_then(|s| {
            let (v, _e) = s;
            Ok(v)
        })
    }

    pub fn query(&self) -> Result<Value, GraphQLError> {
        let variables = Variables::new();
        self.query_with_variables(&variables)
    }

    pub fn query_with_variables(&self, variables: &Variables) -> Result<Value, GraphQLError> {
        let res = juniper::execute(
            "query { notes {id} }",
            None,
            &self.schema,
            &variables,
            &self.context,
        );

        res.and_then(|r| {
            let (v, _e) = r;
            Ok(v)
        })
    }
}

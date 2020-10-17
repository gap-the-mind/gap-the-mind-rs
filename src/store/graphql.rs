use super::model::*;
use super::{StorageContext, Store, StoreError};
use juniper::{FieldResult, GraphQLError, Value, Variables};

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn new_schema() -> Schema {
    Schema::new(Query, Mutation)
}

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
    fn createNote(ctx: &StorageContext, note_input: Option<NoteInput>) -> FieldResult<Note> {
        let note = Note::new();

        Ok(note)
    }

    fn editNote(ctx: &StorageContext, note_input: NoteInput) -> FieldResult<Note> {
        let note = Note::new();

        Ok(note)
    }
}

impl Store {
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

    pub fn query(&self, query: &str) -> Result<Value, StoreError> {
        let variables = Variables::new();
        self.query_with_variables(query, &variables)
    }

    pub fn query_with_variables(
        &self,
        query: &str,
        variables: &Variables,
    ) -> Result<Value, StoreError> {
        let res = juniper::execute(query, None, &self.schema, &variables, &self.context);

        match res {
            Ok(r) => {
                let (v, _e) = r;
                Ok(v)
            }
            Err(_) => Err(StoreError::QueryError),
        }
    }
}

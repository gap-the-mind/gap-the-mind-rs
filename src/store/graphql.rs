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
        let note: Note = Default::default();
        Ok(note)
    }
}

pub struct Mutation;

#[juniper::object(
    Context = StorageContext
)]
impl Mutation {
    fn create_note(ctx: &StorageContext, note_input: Option<NoteInput>) -> FieldResult<Note> {
        let note: Note = Default::default();

        Ok(note)
    }

    fn edit_note(ctx: &StorageContext, note_input: NoteInput) -> FieldResult<Note> {
        let note: Note = Default::default();

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

        schema.map(|s| s.0)
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

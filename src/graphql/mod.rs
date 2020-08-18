mod model;

use model::*;

pub struct Context {}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = Context,
)]
impl Query {
    fn notes(context: &Context) -> juniper::FieldResult<Note> {
        Note::new();
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {}

type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation)
}

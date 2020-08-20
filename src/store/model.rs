use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct Tag {
    id: String,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct Note {
    pub id: Uuid,
    pub text: String,
    pub tags: Vec<Tag>,
}

impl Note {
    pub fn new() -> Note {
        Note {
            id: Uuid::new_v4(),
            text: String::new(),
            tags: Vec::new(),
        }
    }
}

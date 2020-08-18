use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct Tag {
    id: String,
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct Note {
    pub text: String,
    pub tags: Vec<Tag>,
}

impl Note {
    pub fn new() -> Note {
        Note {
            text: String::new(),
            tags: Vec::new(),
        }
    }
}

use juniper::{GraphQLInputObject, GraphQLObject, GraphQLType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

trait Entity<'a>: Serialize + Deserialize<'a> + GraphQLType {
    fn id(&self) -> Uuid;
    fn nature(&self) -> &str;
}

#[derive(Serialize, Deserialize, GraphQLObject)]
pub struct Tag {
    id: Uuid,
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

#[derive(GraphQLInputObject)]
pub struct TagInput {
    id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct NoteInput {
    text: Option<String>,
    tags: Option<Vec<TagInput>>,
}

impl Entity<'_> for Note {
    fn id(&self) -> Uuid {
        self.id
    }

    fn nature(&self) -> &str {
        "note"
    }
}
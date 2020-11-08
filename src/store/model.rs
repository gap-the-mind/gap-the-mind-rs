use juniper::{GraphQLInputObject, GraphQLObject};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Entity: Serialize + DeserializeOwned {
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

impl Default for Note {
    fn default() -> Self {
        Note {
            id: Uuid::new_v4(),
            text: Default::default(),
            tags: Default::default(),
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

impl Entity for Note {
    fn id(&self) -> Uuid {
        self.id
    }

    fn nature(&self) -> &str {
        "note"
    }
}

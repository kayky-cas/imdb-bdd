use serde::Serialize;
use uuid::Uuid;

use crate::person::{Person, Role};

#[derive(Serialize)]
#[serde(rename = "trabalhador", rename_all = "camelCase")]
pub struct Worker {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "idade")]
    role: Role,
}

impl Worker {
    pub fn new(person: Person, role: Role) -> Self {
        Self {
            id: person.id,
            name: person.name,
            role,
        }
    }
}

use serde::Serialize;
use uuid::Uuid;

use crate::{
    person::{PERSON_FIRST_NAMES, PERSON_LAST_NAMES},
    Random,
};

#[derive(Serialize)]
#[serde(rename = "usuario", rename_all = "camelCase")]
struct User {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "idade")]
    age: u32,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "senha")]
    password: String,
    #[serde(rename = "avaliacoes")]
    ratings: Vec<Ratted>,
}

#[derive(Serialize)]
#[serde(rename = "avaliacao", rename_all = "camelCase")]
struct Ratted {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "filme")]
    movie: Uuid,
    #[serde(rename = "nota")]
    rating: u8,
}

impl Random for User {
    fn random() -> Self {
        let name = format!(
            "{} {}",
            PERSON_FIRST_NAMES[rand::thread_rng().gen_range(0..30)],
            PERSON_LAST_NAMES[rand::thread_rng().gen_range(0..30)]
        );

        Self {
            id: Uuid::new_v4(),
            name,
            age: 0,
            email: String::new(),
            password: String::new(),
            ratings: Vec::new(),
        }
    }
}

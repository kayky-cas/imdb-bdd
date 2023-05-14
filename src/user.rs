use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    person::{PERSON_FIRST_NAMES, PERSON_LAST_NAMES},
    Random,
};

#[derive(Serialize)]
#[serde(rename = "usuario", rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "dataNascimento")]
    birth_date: chrono::NaiveDate,
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
        let name = PERSON_FIRST_NAMES[rand::thread_rng().gen_range(0..PERSON_FIRST_NAMES.len())];
        let last_name = PERSON_LAST_NAMES[rand::thread_rng().gen_range(0..PERSON_LAST_NAMES.len())];

        let email = format!(
            "{}.{}.{}@imdb.com",
            name.to_lowercase(),
            last_name.to_lowercase(),
            rand::thread_rng().gen_range(0..100)
        );

        Self {
            id: Uuid::new_v4(),
            name: format!("{} {}", name, last_name),
            birth_date: chrono::NaiveDate::from_ymd_opt(
                rand::thread_rng().gen_range(1900..=2020),
                rand::thread_rng().gen_range(1..=12),
                rand::thread_rng().gen_range(1..=28),
            )
            .unwrap(),
            email,
            password: rand::thread_rng()
                .sample_iter(rand::distributions::Alphanumeric)
                .take(10)
                .map(char::from)
                .collect(),
            ratings: Vec::new(),
        }
    }
}

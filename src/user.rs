use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::{person::Person, ratting::Rate, Random};

#[derive(Serialize)]
#[serde(rename = "usuario", rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "dataNascimento")]
    birth_date: chrono::NaiveDate,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "senha")]
    password: String,
    #[serde(rename = "avaliacoes")]
    rated: Vec<Rate>,
}

impl Random for User {
    fn random() -> Self {
        let (name, email) = Person::generate_random_name_email();

        Self {
            id: Uuid::new_v4(),
            name,
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
            rated: Vec::new(),
        }
    }
}

use crate::Random;
use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::{genre::Genre, worker::Worker};

const SHOWS_TITLES: [&str; 20] = [
    "The Walking Dead",
    "Game of Thrones",
    "Breaking Bad",
    "The Big Bang Theory",
    "Friends",
    "The Office",
    "How I Met Your Mother",
    "The Simpsons",
    "Grey's Anatomy",
    "Supernatural",
    "The Vampire Diaries",
    "Arrow",
    "The Flash",
    "The 100",
    "The Blacklist",
    "The Originals",
    "Modern Family",
    "Family Guy",
    "Gotham",
    "Suits",
];

const EPISODES_TITLES: [&str; 15] = [
    "Piloto",
    "O que aconteceu e o que está acontecendo",
    "O dia virá em que você não estará",
    "O Governador",
    "Aquele com a galinha",
    "Aquele com o frango assado",
    "Jorge, o curioso",
    "Claudio é você?",
    "O dia em que a terra parou",
    "O dia em que a terra parou de novo",
    "O dia em que a terra parou de novo de novo",
    "Terra de ninguém",
    "Meu nome é ninguém",
    "Teste de fidelidade",
    "Teste de fidelidade 2",
];

#[derive(Serialize)]
#[serde(rename = "serie", rename_all = "camelCase")]
struct Show {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "titulo")]
    title: String,
    #[serde(rename = "lancamento")]
    release: chrono::NaiveDate,
    #[serde(rename = "genero")]
    genre: Genre,
    #[serde(rename = "episodios")]
    episodes: Vec<Episode>,
}

#[derive(Serialize)]
#[serde(rename = "espisodio", rename_all = "camelCase")]
struct Episode {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "titulo")]
    title: String,
    #[serde(rename = "lancamento")]
    release: chrono::NaiveDate,
    #[serde(rename = "trabalhadores")]
    workers: Vec<Worker>,
}

impl Random for Episode {
    fn random() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: EPISODES_TITLES[rand::thread_rng().gen_range(0..EPISODES_TITLES.len())]
                .to_string(),
            release: chrono::NaiveDate::from_ymd_opt(
                rand::thread_rng().gen_range(1900..=2021),
                rand::thread_rng().gen_range(1..=12),
                rand::thread_rng().gen_range(1..=28),
            )
            .unwrap(),
            workers: vec![],
        }
    }
}

use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::{genre::Genre, worker::Worker, Random, RandomVec};

pub const MOVIE_TILTES: [&str; 20] = [
    "O Poderoso Chefão",
    "O Poderoso Chefão II",
    "Batman",
    "12 Homens e uma Sentença",
    "A Lista de Schindler",
    "O Senhor dos Anéis",
    "Pulp Fiction",
    "Três Homens em Conflito",
    "O Senhor dos Anéis",
    "Forrest Gump",
    "O Senhor dos Anéis",
    "Matrix",
    "Os Bons Companheiros",
    "Interestelar",
    "O Silêncio dos Inocentes",
    "Vingadores",
    "Cidade de Deus",
    "O Rei Leão",
    "Gladiador",
    "Clube da Luta",
];

pub const MOVIE_SUBTITLE: [&str; 16] = [
    "A casa caiu",
    "A vingança",
    "A volta dos que não foram",
    "A volta por cima",
    "A volta triunfal",
    "A volta",
    "Será que vai?",
    "Será que não vai?",
    "Será que vai mesmo?",
    "A morte do herói",
    "A morte do vilão",
    "A morte do coadjuvante",
    "O despertar",
    "O retorno",
    "O renascimento",
    "O renascer",
];

#[derive(Serialize)]
#[serde(rename = "filme", rename_all = "camelCase")]
pub struct Movie {
    #[serde(rename = "_id")]
    pub id: Uuid,
    #[serde(rename = "titulo")]
    pub title: String,
    #[serde(rename = "lancamento")]
    release: chrono::NaiveDate,
    #[serde(rename = "genero")]
    genre: Genre,
    #[serde(rename = "trabalhadores")]
    workers: Vec<Worker>,
}

impl Random for Movie {
    fn random() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: format!(
                "{} {}",
                MOVIE_TILTES[rand::thread_rng().gen_range(0..MOVIE_TILTES.len())],
                MOVIE_SUBTITLE[rand::thread_rng().gen_range(0..MOVIE_SUBTITLE.len())]
            ),
            release: chrono::NaiveDate::from_ymd_opt(
                rand::thread_rng().gen_range(1900..2021),
                rand::thread_rng().gen_range(1..13),
                rand::thread_rng().gen_range(1..29),
            )
            .unwrap(),
            genre: Genre::random(),
            workers: Vec::new(),
        }
    }
}

impl Movie {
    pub fn add_worker(&mut self, worker: Worker) {
        self.workers.push(worker);
    }
}

use crate::{ratting::Rate, Random, RandomVec};
use chrono::Datelike;
use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::{genre::Genre, work::Work};

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
pub struct Show {
    #[serde(rename = "_id")]
    pub id: Uuid,
    #[serde(rename = "titulo")]
    pub title: String,
    #[serde(rename = "lancamento")]
    pub release: chrono::NaiveDate,
    #[serde(rename = "genero")]
    genre: Genre,
    #[serde(rename = "nota")]
    avarage_ratting: f32,
    #[serde(rename = "episodios")]
    pub episodes: Vec<Episode>,
}

#[derive(Serialize)]
#[serde(rename = "espisodio", rename_all = "camelCase")]
pub struct Episode {
    #[serde(rename = "_id")]
    pub id: Uuid,
    #[serde(rename = "titulo")]
    pub title: String,
    #[serde(rename = "numero")]
    pub number: usize,
    #[serde(rename = "lancamento")]
    release: chrono::NaiveDate,
    #[serde(rename = "trabalhadores")]
    workers: Vec<Work>,
    #[serde(rename = "nota")]
    ratting: Vec<Rate>,
    #[serde(rename = "notaMedia")]
    avarage_ratting: f32,
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
            ratting: vec![],
            number: 0,
            workers: vec![],
            avarage_ratting: 0f32,
        }
    }
}

impl Random for Show {
    fn random() -> Self {
        let mut episodes: Vec<Episode> =
            RandomVec::random_vec(rand::thread_rng().gen_range(1..=20));

        episodes.sort_by(|a, b| a.release.cmp(&b.release));

        for (idx, episode) in episodes.iter_mut().enumerate() {
            episode.number = idx + 1;
        }

        Self {
            id: Uuid::new_v4(),
            title: SHOWS_TITLES[rand::thread_rng().gen_range(0..SHOWS_TITLES.len())].to_string(),
            release: episodes[0].release,
            genre: Genre::random(),
            ratting: 0f32,
            episodes,
        }
    }
}

impl Episode {
    pub fn append_worker(&mut self, worker: Work) {
        self.workers.push(worker);
    }
}

impl Show {
    pub fn episodes_quantity(&self) -> usize {
        self.episodes.len()
    }
}

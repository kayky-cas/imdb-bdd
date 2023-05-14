use crate::{Random, RandomVec};
use chrono::Datelike;
use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::movie::Movie;

pub const PERSON_FIRST_NAMES: [&str; 30] = [
    "John",
    "Mary",
    "Robert",
    "Patricia",
    "Michael",
    "Jennifer",
    "William",
    "Elizabeth",
    "David",
    "Linda",
    "Richard",
    "Barbara",
    "Joseph",
    "Susan",
    "Thomas",
    "Jessica",
    "Charles",
    "Sarah",
    "Christopher",
    "Karen",
    "Daniel",
    "Nancy",
    "Matthew",
    "Margaret",
    "Anthony",
    "Lisa",
    "Donald",
    "Betty",
    "Mark",
    "Dorothy",
];

pub const PERSON_LAST_NAMES: [&str; 30] = [
    "Smith",
    "Johnson",
    "Williams",
    "Brown",
    "Jones",
    "Miller",
    "Davis",
    "Garcia",
    "Rodriguez",
    "Wilson",
    "Martinez",
    "Anderson",
    "Taylor",
    "Thomas",
    "Hernandez",
    "Moore",
    "Martin",
    "Jackson",
    "Thompson",
    "White",
    "Lopez",
    "Lee",
    "Gonzalez",
    "Harris",
    "Clark",
    "Lewis",
    "Robinson",
    "Walker",
    "Perez",
    "Hall",
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    #[serde(rename = "ator")]
    Actor(String),
    #[serde(rename = "diretor")]
    Director,
    #[serde(rename = "produtor")]
    Producer,
    #[serde(rename = "roteirista")]
    Writer,
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "cinematografo")]
    Cinematographer,
    #[serde(rename = "compositor")]
    Composer,
    #[serde(rename = "figurinista")]
    CostumeDesigner,
    #[serde(rename = "desenhista de producao")]
    ProductionDesigner,
    #[serde(rename = "maquiador")]
    MakeupArtist,
    #[serde(rename = "outro")]
    Other,
}

#[derive(Serialize)]
#[serde(rename = "pessoa", rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "_id")]
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "dataNascimento")]
    pub birth_date: chrono::NaiveDate,
    #[serde(rename = "oscars")]
    pub oscars: Vec<usize>,
    #[serde(rename = "trabalhos")]
    works: Vec<Works>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
enum Media {
    #[serde(rename = "filme")]
    Movie,
    #[serde(rename = "episodio")]
    Episode,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Works {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "tipo")]
    media: Media,
    #[serde(rename = "titulo")]
    title: String,
    #[serde(rename = "papel")]
    role: Role,
}

impl Random for Person {
    fn random() -> Self {
        let birth_date = chrono::NaiveDate::from_ymd_opt(
            rand::thread_rng().gen_range(1900..=2020),
            rand::thread_rng().gen_range(1..=12),
            rand::thread_rng().gen_range(1..=28),
        )
        .unwrap();

        let mut oscars = Vec::new();

        for _ in 0..rand::thread_rng().gen_range(0..=4) {
            oscars.push(
                rand::thread_rng().gen_range(birth_date.year() + 20..=birth_date.year() + 50)
                    as usize,
            );
        }

        Person {
            id: Uuid::new_v4(),
            name: format!(
                "{} {}",
                PERSON_FIRST_NAMES[rand::thread_rng().gen_range(0..30)],
                PERSON_LAST_NAMES[rand::thread_rng().gen_range(0..30)]
            ),
            birth_date,
            oscars,
            works: Vec::new(),
        }
    }
}

impl Works {
    pub fn new(movie: Movie, role: Role) -> Self {
        Self {
            media: Media::Movie,
            id: movie.id,
            title: movie.title,
            role,
        }
    }
}

impl Person {
    pub fn append_work(&mut self, movie: Movie, role: Role) {
        self.works.push(Works::new(movie, role));
    }
}

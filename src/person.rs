use crate::{work::Work, Random};
use chrono::Datelike;
use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

const PERSON_FIRST_NAMES: [&str; 30] = [
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

const PERSON_LAST_NAMES: [&str; 30] = [
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
    works: Vec<Work>,
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
            name: Person::generate_random_name_email().0,
            birth_date,
            oscars,
            works: Vec::new(),
        }
    }
}

impl Person {
    pub fn append_work(&mut self, work: Work) {
        self.works.push(work);
    }

    pub fn generate_random_name_email() -> (String, String) {
        let name = PERSON_FIRST_NAMES[rand::thread_rng().gen_range(0..PERSON_FIRST_NAMES.len())];
        let last_name = PERSON_LAST_NAMES[rand::thread_rng().gen_range(0..PERSON_LAST_NAMES.len())];

        (
            format!("{} {}", name, last_name),
            format!(
                "{}.{}.{}@imdb.com",
                name.to_lowercase(),
                last_name.to_lowercase(),
                rand::thread_rng().gen_range(0..100)
            ),
        )
    }
}

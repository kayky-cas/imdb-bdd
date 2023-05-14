use crate::Random;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Genre {
    #[serde(rename = "ação")]
    Action,
    #[serde(rename = "aventura")]
    Adventure,
    #[serde(rename = "comédia")]
    Comedy,
    #[serde(rename = "drama")]
    Drama,
    #[serde(rename = "terror")]
    Horror,
    #[serde(rename = "romance")]
    Romance,
    #[serde(rename = "ficção científica")]
    ScienceFiction,
    #[serde(rename = "musical")]
    Musical,
}

impl Random for Genre {
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..8) {
            0 => Genre::Action,
            1 => Genre::Adventure,
            2 => Genre::Comedy,
            3 => Genre::Drama,
            4 => Genre::Horror,
            5 => Genre::Romance,
            6 => Genre::ScienceFiction,
            _ => Genre::Musical,
        }
    }
}

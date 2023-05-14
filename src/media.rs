use rand::Rng;
use serde::Serialize;

use crate::Random;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Media {
    #[serde(rename = "filme")]
    Movie,
    #[serde(rename = "série")]
    Show(usize),
}

impl Random for Media {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..=1) {
            0 => Media::Movie,
            _ => Media::Show(0),
        }
    }
}

use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::{media::Media, user::User, Random};

const MAX_RATTED: u8 = 5;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone)]
#[serde(rename = "avaliacao", rename_all = "camelCase")]
pub struct Rate {
    #[serde(rename = "_id")]
    id: Uuid,
    #[serde(rename = "tipo")]
    media: Option<Media>,
    #[serde(rename = "midia_id")]
    media_id: Option<Uuid>,
    #[serde(rename = "user_id")]
    user_id: Option<Uuid>,
    #[serde(rename = "titulo")]
    title: Option<String>,
    #[serde(rename = "nome")]
    name: Option<String>,
    #[serde(rename = "nota")]
    pub rate: u8,
}

impl Random for Rate {
    fn random() -> Self {
        let rate = rand::thread_rng().gen_range(0..=MAX_RATTED);

        Self {
            id: Uuid::new_v4(),
            media: None,
            media_id: None,
            user_id: None,
            title: None,
            name: None,
            rate,
        }
    }
}

impl Rate {
    fn for_user(&self, media: Media, media_id: Uuid, title: String) -> Self {
        let mut rate = self.clone();

        rate.media = Some(media);
        rate.media_id = Some(media_id);
        rate.title = Some(title);

        rate
    }

    fn for_movie(&self, user: &User) -> Self {
        let mut rate = self.clone();

        rate.user_id = Some(user.id);
        rate.name = Some(user.name.clone());

        rate
    }
}

#[derive(Serialize, Clone)]
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

pub enum RoleType {
    Main,
    Prodution,
}

impl Random for RoleType {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..=1) {
            0 => RoleType::Main,
            _ => RoleType::Prodution,
        }
    }
}

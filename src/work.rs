use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use crate::{media::Media, person::Person, Random};

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

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone)]
#[serde(rename = "trabalhador", rename_all = "camelCase")]
pub struct Work {
    #[serde(rename = "user_id")]
    user_id: Option<Uuid>,
    #[serde(rename = "nome")]
    name: Option<String>,
    #[serde(rename = "tipo")]
    media: Option<Media>,
    #[serde(rename = "media_id")]
    media_id: Option<Uuid>,
    #[serde(rename = "title")]
    title: Option<String>,
    #[serde(rename = "idade")]
    role: Role,
}

impl Work {
    pub fn for_media(person: &Person, role: Role) -> Self {
        Self {
            user_id: Some(person.id),
            name: Some(person.name.clone()),
            role,
            media: None,
            media_id: None,
            title: None,
        }
    }

    pub fn for_person(media_id: Uuid, title: String, media: Media, role: Role) -> Self {
        Self {
            user_id: None,
            name: None,
            role,
            media: Some(media),
            media_id: Some(media_id),
            title: Some(title),
        }
    }
}

impl Role {
    pub fn random(role_type: &RoleType) -> Self {
        let range = match role_type {
            RoleType::Main => 0..4,
            RoleType::Prodution => 4..10,
        };

        match rand::thread_rng().gen_range(range) {
            0 => Role::Actor(Person::generate_random_name_email().0),
            1 => Role::Director,
            2 => Role::Producer,
            3 => Role::Writer,
            4 => Role::Editor,
            5 => Role::Cinematographer,
            6 => Role::Composer,
            7 => Role::CostumeDesigner,
            8 => Role::ProductionDesigner,
            9 => Role::MakeupArtist,
            _ => Role::Other,
        }
    }
}

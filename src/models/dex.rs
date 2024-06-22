use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Fire,
    Water,
    Grass,
    Electric,
    Sonic,
    Dark,
    Mystic,
    Legendary,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            Type::Fire => "Fire",
            Type::Water => "Water",
            Type::Grass => "Grass",
            Type::Electric => "Electric",
            Type::Sonic => "Sonic",
            Type::Dark => "Dark",
            Type::Mystic => "Mystic",
            Type::Legendary => "Legendary",
        };
        write!(f, "{}", type_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DexMon {
    pub id: u32,
    pub name: String,
    pub types: Vec<Type>,
}

pub fn load_dex() -> Vec<DexMon> {
    vec![
        DexMon {
            id: 0,
            name: "Shellora".to_string(),
            types: vec![Type::Mystic],
        },
        DexMon {
            id: 1,
            name: "Javascriptik".to_string(),
            types: vec![Type::Electric],
        },
        DexMon {
            id: 2,
            name: "Vuesaur".to_string(),
            types: vec![Type::Grass],
        },
        DexMon {
            id: 2,
            name: "Reacto".to_string(),
            types: vec![Type::Water],
        },
        DexMon {
            id: 3,
            name: "Sveltra".to_string(),
            types: vec![Type::Fire],
        },
        DexMon {
            id: 4,
            name: "Pythorus".to_string(),
            types: vec![Type::Water],
        },
        DexMon {
            id: 5,
            name: "Gogomon".to_string(),
            types: vec![Type::Sonic],
        },
        DexMon {
            id: 6,
            name: "Supaswift".to_string(),
            types: vec![Type::Mystic],
        },
        DexMon {
            id: 7,
            name: "Rustaking".to_string(),
            types: vec![Type::Legendary],
        },
    ]
}

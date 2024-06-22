use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Origin,
    Fire,
    Water,
    Grass,
    Electric,
    Sonic,
    Dark,
    Garbage,
    Mystic,
    Legendary,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            Type::Origin => "Origin",
            Type::Fire => "Fire",
            Type::Water => "Water",
            Type::Grass => "Grass",
            Type::Electric => "Electric",
            Type::Sonic => "Sonic",
            Type::Dark => "Dark",
            Type::Garbage => "Garbage",
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
            id: 1,
            name: "Shellora".to_string(),
            types: vec![Type::Origin],
        },
        DexMon {
            id: 2,
            name: "Javascriptik".to_string(),
            types: vec![Type::Electric],
        },
        DexMon {
            id: 3,
            name: "Vuesaur".to_string(),
            types: vec![Type::Grass],
        },
        DexMon {
            id: 4,
            name: "Reacto".to_string(),
            types: vec![Type::Water],
        },
        DexMon {
            id: 5,
            name: "Sveltra".to_string(),
            types: vec![Type::Fire],
        },
        DexMon {
            id: 6,
            name: "Gogomon".to_string(),
            types: vec![Type::Sonic],
        },
        DexMon {
            id: 7,
            name: "Cobolkool".to_string(),
            types: vec![Type::Garbage],
        },
        DexMon {
            id: 8,
            name: "Kotlinite".to_string(),
            types: vec![Type::Dark],
        },
        DexMon {
            id: 9,
            name: "Supaswift".to_string(),
            types: vec![Type::Mystic],
        },
        DexMon {
            id: 10,
            name: "Rustaking".to_string(),
            types: vec![Type::Legendary],
        },
        DexMon {
            id: 11,
            name: "Basharoo".to_string(),
            types: vec![Type::Origin],
        },
        DexMon {
            id: 12,
            name: "Typescriptik".to_string(),
            types: vec![Type::Electric],
        },
        DexMon {
            id: 13,
            name: "Angularon".to_string(),
            types: vec![Type::Grass],
        },
        DexMon {
            id: 14,
            name: "Django".to_string(),
            types: vec![Type::Water],
        },
        DexMon {
            id: 15,
            name: "Flaskon".to_string(),
            types: vec![Type::Fire],
        },
        DexMon {
            id: 16,
            name: "Elixira".to_string(),
            types: vec![Type::Sonic],
        },
        DexMon {
            id: 17,
            name: "Fortranix".to_string(),
            types: vec![Type::Garbage],
        },
        DexMon {
            id: 18,
            name: "Haskelion".to_string(),
            types: vec![Type::Dark],
        },
        DexMon {
            id: 19,
            name: "Elixira".to_string(),
            types: vec![Type::Mystic],
        },
        DexMon {
            id: 20,
            name: "Elrang".to_string(),
            types: vec![Type::Legendary],
        },
    ]
}

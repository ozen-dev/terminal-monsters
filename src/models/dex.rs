use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum Family {
    Scripting,
    Web,
    Mobile,
    Gaming,
    Database,
    Systems,
    Neural,
    Ancient,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DexMon {
    pub id: u32,
    pub name: String,
    pub family: Family,
    pub commands: Vec<String>,
}

pub fn load_dex() -> Vec<DexMon> {
    vec![
        // Scripting
        DexMon {
            id: 1,
            name: "Shellora".to_string(),
            family: Family::Scripting,
            commands: vec!["sh".to_string()],
        },
        DexMon {
            id: 2,
            name: "Basharoo".to_string(),
            family: Family::Scripting,
            commands: vec!["bash".to_string()],
        },
        // Web
        DexMon {
            id: 3,
            name: "Javascriptik".to_string(),
            family: Family::Web,
            commands: vec!["node".to_string(), "npm install".to_string()],
        },
        DexMon {
            id: 4,
            name: "Typescriptik".to_string(),
            family: Family::Web,
            commands: vec!["tsc".to_string(), "npm install -g typescript".to_string()],
        },
        DexMon {
            id: 5,
            name: "Reacto".to_string(),
            family: Family::Web,
            commands: vec![
                "npm create react-app".to_string(),
                "yarn create react-app".to_string(),
                "pnpm create react-app".to_string(),
                "bun create react-app".to_string(),
                "npx create-next-app@latest".to_string(),
                "npx create-remix".to_string(),
                "npx create-gatsby".to_string(),
                "npx create-expo-app".to_string(),
            ],
        },
        DexMon {
            id: 6,
            name: "Vuesaur".to_string(),
            family: Family::Web,
            commands: vec![
                "npm create vue@latest".to_string(),
                "pnpm create vue@latest".to_string(),
                "yarn create vue@latest".to_string(),
                "bun create vue@latest".to_string(),
            ],
        },
        DexMon {
            id: 7,
            name: "Sveltra".to_string(),
            family: Family::Web,
            commands: vec![
                "npm create svelte@latest".to_string(),
                "pnpm create svelte@latest".to_string(),
                "yarn create svelte@latest".to_string(),
                "bun create svelte@latest".to_string(),
            ],
        },
        DexMon {
            id: 8,
            name: "Solidon".to_string(),
            family: Family::Web,
            commands: vec![
                "npx degit solidjs/templates".to_string(),
                "npm install solid-js".to_string(),
                "yarn add solid-js".to_string(),
                "pnpm add solid-js".to_string(),
                "bun add solid-js".to_string(),
            ],
        },
        // Mobile
        DexMon {
            id: 9,
            name: "Supaswift".to_string(),
            family: Family::Mobile,
            commands: vec!["swift".to_string()],
        },
        DexMon {
            id: 10,
            name: "Kotlinite".to_string(),
            family: Family::Mobile,
            commands: vec![
                "kotlinc -script".to_string(),
                "gradle init --type kotlin-application".to_string(),
            ],
        },
        DexMon {
            id: 11,
            name: "Flutterix".to_string(),
            family: Family::Mobile,
            commands: vec!["flutter create".to_string()],
        },
        // Gaming
        DexMon {
            id: 12,
            name: "Unito".to_string(),
            family: Family::Gaming,
            commands: vec!["unity".to_string()],
        },
        DexMon {
            id: 13,
            name: "Luamon".to_string(),
            family: Family::Gaming,
            commands: vec!["lua".to_string()],
        },
        DexMon {
            id: 14,
            name: "Godotaur".to_string(),
            family: Family::Gaming,
            commands: vec!["godot".to_string()],
        },
        // Database
        DexMon {
            id: 15,
            name: "Squeel".to_string(),
            family: Family::Database,
            commands: vec!["mysql".to_string()],
        },
        DexMon {
            id: 16,
            name: "PostgreSqueel".to_string(),
            family: Family::Database,
            commands: vec!["psql".to_string(), "supabase".to_string()],
        },
        DexMon {
            id: 17,
            name: "Mongor".to_string(),
            family: Family::Database,
            commands: vec!["mongo".to_string(), "mongod".to_string()],
        },
        // Systems
        DexMon {
            id: 18,
            name: "Rustaking".to_string(),
            family: Family::Systems,
            commands: vec![
                "rustc".to_string(),
                "rustup".to_string(),
                "cargo".to_string(),
            ],
        },
        DexMon {
            id: 19,
            name: "Gogomon".to_string(),
            family: Family::Systems,
            commands: vec!["go mod init".to_string()],
        },
        DexMon {
            id: 20,
            name: "Ceezor".to_string(),
            family: Family::Systems,
            commands: vec!["gcc -o".to_string()],
        },
        DexMon {
            id: 21,
            name: "Ceepluzor".to_string(),
            family: Family::Systems,
            commands: vec!["g++ -o".to_string()],
        },
        // Neural
        DexMon {
            id: 22,
            name: "Pythor".to_string(),
            family: Family::Neural,
            commands: vec!["python".to_string(), "pip install".to_string()],
        },
        DexMon {
            id: 23,
            name: "Tensora".to_string(),
            family: Family::Neural,
            commands: vec!["import tensorflow".to_string()],
        },
        DexMon {
            id: 24,
            name: "Pandasora".to_string(),
            family: Family::Neural,
            commands: vec!["import pandas".to_string()],
        },
        // Ancient
        DexMon {
            id: 25,
            name: "Unixon".to_string(),
            family: Family::Ancient,
            commands: vec!["uname".to_string()],
        },
    ]
}

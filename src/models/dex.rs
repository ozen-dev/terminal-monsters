use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
    pub collect_cmds: HashSet<String>,
    pub exp_commands: HashSet<String>,
}

pub fn load_dex() -> Vec<DexMon> {
    vec![
        DexMon {
            id: 0,
            name: "Shellora".to_string(),
            family: Family::Scripting,
            collect_cmds: HashSet::from(["sh".to_string()]),
            exp_commands: HashSet::from(["sh".to_string()]),
        },
        DexMon {
            id: 1,
            name: "Basharoo".to_string(),
            family: Family::Scripting,
            collect_cmds: HashSet::from(["bash".to_string()]),
            exp_commands: HashSet::from(["bash".to_string()]),
        },
        DexMon {
            id: 2,
            name: "Zishan".to_string(),
            family: Family::Scripting,
            collect_cmds: HashSet::from(["zsh".to_string()]),
            exp_commands: HashSet::from(["zsh".to_string()]),
        },
        DexMon {
            id: 3,
            name: "Javascriptik".to_string(),
            family: Family::Web,
            collect_cmds: HashSet::from([
                "node".to_string(),
                "npm".to_string(),
                "pnpm".to_string(),
                "yarn".to_string(),
                "bun".to_string(),
            ]),
            exp_commands: HashSet::from([
                "npm install".to_string(),
                "npm run dev".to_string(),
                "pnpm run dev".to_string(),
                "yarn run dev".to_string(),
                "bun run dev".to_string(),
            ]),
        },
        DexMon {
            id: 4,
            name: "Typescriptik".to_string(),
            family: Family::Web,
            collect_cmds: HashSet::from(["tsc".to_string(), "tsc --init".to_string()]),
            exp_commands: HashSet::from([
                "npm install -g typescript".to_string(),
                "npm install @types/node".to_string(),
                "pnpm install @types/node".to_string(),
                "yarn global add typescript".to_string(),
                "bun add -g typescript".to_string(),
                "npm run dev".to_string(),
                "pnpm run dev".to_string(),
                "yarn run dev".to_string(),
                "bun run dev".to_string(),
            ]),
        },
        DexMon {
            id: 5,
            name: "Reacto".to_string(),
            family: Family::Web,
            collect_cmds: HashSet::from([
                "npm create react-app".to_string(),
                "yarn create react-app".to_string(),
                "pnpm create react-app".to_string(),
                "bun create react-app".to_string(),
                "npx create-next-app@latest".to_string(),
                "npx create-remix".to_string(),
                "npx create-gatsby".to_string(),
                "npx create-expo-app".to_string(),
            ]),
            exp_commands: HashSet::from([
                "npm run dev".to_string(),
                "pnpm run dev".to_string(),
                "yarn run dev".to_string(),
                "bun run dev".to_string(),
            ]),
        },
        DexMon {
            id: 6,
            name: "Vuesaur".to_string(),
            family: Family::Web,
            collect_cmds: HashSet::from([
                "npm create vue@latest".to_string(),
                "pnpm create vue@latest".to_string(),
                "yarn create vue@latest".to_string(),
                "bun create vue@latest".to_string(),
            ]),
            exp_commands: HashSet::from([
                "npm run dev".to_string(),
                "pnpm run dev".to_string(),
                "yarn run dev".to_string(),
                "bun run dev".to_string(),
            ]),
        },
        DexMon {
            id: 7,
            name: "Sveltra".to_string(),
            family: Family::Web,
            collect_cmds: HashSet::from([
                "npm create svelte@latest".to_string(),
                "pnpm create svelte@latest".to_string(),
                "yarn create svelte@latest".to_string(),
                "bun create svelte@latest".to_string(),
            ]),
            exp_commands: HashSet::from([
                "npm run dev".to_string(),
                "pnpm run dev".to_string(),
                "yarn run dev".to_string(),
                "bun run dev".to_string(),
            ]),
        },
        DexMon {
            id: 8,
            name: "Solidon".to_string(),
            family: Family::Web,
            collect_cmds: HashSet::from([
                "npx degit solidjs/templates".to_string(),
                "npm install solid-js".to_string(),
                "yarn add solid-js".to_string(),
                "pnpm add solid-js".to_string(),
                "bun add solid-js".to_string(),
            ]),
            exp_commands: HashSet::from([
                "npm run dev".to_string(),
                "pnpm run dev".to_string(),
                "yarn run dev".to_string(),
                "bun run dev".to_string(),
            ]),
        },
        DexMon {
            id: 9,
            name: "Supaswift".to_string(),
            family: Family::Mobile,
            collect_cmds: HashSet::from(["swift".to_string()]),
            exp_commands: HashSet::from(["swift".to_string()]),
        },
        DexMon {
            id: 10,
            name: "Kotlinite".to_string(),
            family: Family::Mobile,
            collect_cmds: HashSet::from([
                "kotlinc -script".to_string(),
                "gradle init --type kotlin-application".to_string(),
            ]),
            exp_commands: HashSet::from(["kotlinc -script".to_string()]),
        },
        DexMon {
            id: 11,
            name: "Flutterix".to_string(),
            family: Family::Mobile,
            collect_cmds: HashSet::from(["flutter create".to_string()]),
            exp_commands: HashSet::from(["flutter run".to_string()]),
        },
        DexMon {
            id: 12,
            name: "Unito".to_string(),
            family: Family::Gaming,
            collect_cmds: HashSet::from(["unity".to_string()]),
            exp_commands: HashSet::from(["unity".to_string()]),
        },
        DexMon {
            id: 13,
            name: "Luamon".to_string(),
            family: Family::Gaming,
            collect_cmds: HashSet::from(["lua".to_string()]),
            exp_commands: HashSet::from(["lua".to_string()]),
        },
        DexMon {
            id: 14,
            name: "Godotaur".to_string(),
            family: Family::Gaming,
            collect_cmds: HashSet::from(["godot".to_string()]),
            exp_commands: HashSet::from(["godot".to_string()]),
        },
        DexMon {
            id: 15,
            name: "Squeel".to_string(),
            family: Family::Database,
            collect_cmds: HashSet::from(["mysql".to_string()]),
            exp_commands: HashSet::from(["mysql".to_string()]),
        },
        DexMon {
            id: 16,
            name: "Squeelite".to_string(),
            family: Family::Database,
            collect_cmds: HashSet::from(["psql".to_string(), "supabase".to_string()]),
            exp_commands: HashSet::from(["psql".to_string()]),
        },
        DexMon {
            id: 17,
            name: "Mongor".to_string(),
            family: Family::Database,
            collect_cmds: HashSet::from(["mongo".to_string(), "mongod".to_string()]),
            exp_commands: HashSet::from(["mongo".to_string()]),
        },
        DexMon {
            id: 18,
            name: "Rustaking".to_string(),
            family: Family::Systems,
            collect_cmds: HashSet::from([
                "rustc".to_string(),
                "rustup".to_string(),
                "cargo".to_string(),
                "tauri".to_string(),
            ]),
            exp_commands: HashSet::from(["cargo run".to_string()]),
        },
        DexMon {
            id: 19,
            name: "Gogomon".to_string(),
            family: Family::Systems,
            collect_cmds: HashSet::from(["go mod init".to_string()]),
            exp_commands: HashSet::from(["go mod init".to_string()]),
        },
        DexMon {
            id: 20,
            name: "Ceezor".to_string(),
            family: Family::Systems,
            collect_cmds: HashSet::from(["gcc -o".to_string()]),
            exp_commands: HashSet::from(["gcc -o".to_string()]),
        },
        DexMon {
            id: 21,
            name: "Ceepluzor".to_string(),
            family: Family::Systems,
            collect_cmds: HashSet::from(["g++ -o".to_string()]),
            exp_commands: HashSet::from(["g++ -o".to_string()]),
        },
        DexMon {
            id: 22,
            name: "Pythor".to_string(),
            family: Family::Neural,
            collect_cmds: HashSet::from(["python".to_string()]),
            exp_commands: HashSet::from(["pip install".to_string()]),
        },
        DexMon {
            id: 23,
            name: "Tensora".to_string(),
            family: Family::Neural,
            collect_cmds: HashSet::from(["import tensorflow".to_string()]),
            exp_commands: HashSet::from(["import tensorflow".to_string()]),
        },
        DexMon {
            id: 24,
            name: "Pandasora".to_string(),
            family: Family::Neural,
            collect_cmds: HashSet::from(["import pandas".to_string()]),
            exp_commands: HashSet::from(["import pandas".to_string()]),
        },
        DexMon {
            id: 25,
            name: "Unixon".to_string(),
            family: Family::Ancient,
            collect_cmds: HashSet::from(["uname".to_string()]),
            exp_commands: HashSet::from(["uname".to_string()]),
        },
    ]
}

pub fn get_dex_mon_by_id(id: u32) -> Option<DexMon> {
    load_dex().into_iter().find(|mon| mon.id == id)
}

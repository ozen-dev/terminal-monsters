use crate::models::dex::{load_dex, DexMon};
use dirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartyMon {
    pub dex_entry: DexMon,
    pub level: u32,
    pub experience_range: (u32, u32),
}

impl PartyMon {
    pub fn new(dex_entry: DexMon, level: u32, experience_range: (u32, u32)) -> Self {
        Self {
            dex_entry,
            level,
            experience_range,
        }
    }
}

fn get_party_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push("Documents/Games/TerminalMonsters/.data");

    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }

    path.push(".party.json");
    path
}

/// Initializes the party by creating a new one from the dex if the file does not exist.
pub fn initialize_party() -> io::Result<Vec<PartyMon>> {
    match load_party() {
        Ok(party) => Ok(party),
        Err(_) => {
            let starter_mon = load_dex().first().unwrap().clone();
            let party = vec![PartyMon::new(starter_mon, 1, (0, 100))];
            save_party(&party)?;
            Ok(party)
        }
    }
}

/// Loads the party from a JSON file.
pub fn load_party() -> io::Result<Vec<PartyMon>> {
    let file_path = get_party_file_path();

    if !file_path.exists() {
        let starter_mon = load_dex().first().unwrap().clone();
        let party = vec![PartyMon::new(starter_mon, 1, (0, 100))];
        save_party(&party)?;
    }

    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);
    let mut party: Vec<PartyMon> = serde_json::from_reader(reader)?;
    party.sort_by_key(|mon| mon.dex_entry.id);

    Ok(party)
}

/// Saves the party to a JSON file.
pub fn save_party(party: &[PartyMon]) -> io::Result<()> {
    let file_path = get_party_file_path();
    let file = File::create(&file_path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, party)?;

    Ok(())
}

use crate::models::dex::{load_dex, DexMon};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

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

/// Initializes the party by loading from file or creating a new one from the dex.
pub fn initialize_party() -> io::Result<Vec<PartyMon>> {
    let mut party = match load_party() {
        Ok(party) => party,
        Err(_) => {
            // let starter_mon = load_dex().first().unwrap().clone();
            // vec![PartyMon::new(starter_mon, 1, (0, 100))]
            // DEV MODE ONLY:
            let dex = load_dex();
            get_all_mons(&dex)
        }
    };
    party.sort_by_key(|mon| mon.dex_entry.id);
    save_party(&party)?;
    Ok(party)
}

/// Saves the party to a JSON file.
pub fn save_party(party: &[PartyMon]) -> io::Result<()> {
    let file = File::create("party.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, party)?;
    Ok(())
}

/// Loads the party from a JSON file.
pub fn load_party() -> io::Result<Vec<PartyMon>> {
    let file = File::open("party.json")?;
    let reader = BufReader::new(file);
    let party: Vec<PartyMon> = serde_json::from_reader(reader)?;
    Ok(party)
}

/// DEV MODE ONLY:
/// Converts all DexMon entries to PartyMon.
#[allow(dead_code)]
pub fn get_all_mons(dex: &[DexMon]) -> Vec<PartyMon> {
    dex.iter()
        .map(|mon| PartyMon::new(mon.clone(), 1, (0, 100)))
        .collect()
}

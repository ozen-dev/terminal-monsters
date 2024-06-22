use crate::models::dex::{load_dex, DexMon};
use rand::seq::SliceRandom;
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

pub fn initialize_party() -> Vec<PartyMon> {
    load_party().unwrap_or_else(|_| {
        let dex = load_dex();
        vec![PartyMon::new(dex[0].clone(), 1, (0, 100))]
    })
}

pub fn save_party(party: &[PartyMon]) -> io::Result<()> {
    let file = File::create("party.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, party)?;
    Ok(())
}

pub fn load_party() -> io::Result<Vec<PartyMon>> {
    let file = File::open("party.json")?;
    let reader = BufReader::new(file);
    let party = serde_json::from_reader(reader)?;
    Ok(party)
}

pub fn get_random_party_mon(dex: &[DexMon]) -> PartyMon {
    let mut rng = rand::thread_rng();
    let dex_entry = dex.choose(&mut rng).unwrap().clone();
    PartyMon::new(dex_entry, 1, (0, 100))
}

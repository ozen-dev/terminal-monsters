#[allow(unused_imports)]
use app::models::dex::{get_dex_mon_by_id, load_dex, DexMon, Family};
use app::models::party::{initialize_party, save_party, PartyMon};
use colored::*;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[allow(dead_code)]
fn main() -> io::Result<()> {
    let dex = load_dex();

    // Hash command to monster
    let mut command_map: HashMap<&str, &DexMon> = HashMap::new();
    for dex_mon in &dex {
        for command in &dex_mon.commands {
            command_map.insert(command, dex_mon);
        }
    }

    // Read input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Distribute 1 experience points to all monsters in the party
    let mut party = initialize_party().unwrap_or_else(|_| vec![]);
    for mon in &mut party {
        mon.gain_experience(1);
    }

    // Check if input contains a command from the dex
    for line in reader.lines() {
        let command = line?;

        // Check if the command contains any of the known commands
        if let Some((_, dex_mon)) = command_map.iter().find(|(cmd, _)| command.contains(*cmd)) {
            // Check if the monster is already in the party
            if let Some(mon) = party.iter_mut().find(|mon| mon.dex_id == dex_mon.id) {
                // Attribute 33 experience points to the specific monster
                mon.gain_experience(33);
            } else {
                // Add monster to party
                party.push(PartyMon::new(dex_mon.id, 1, (0, 100)));

                // Notify user
                println!("{}", "+ Terminal Monsters Inc. -----------------+".green());
                println!(
                    "{}",
                    format!(
                        "{} {} joined your party!",
                        "|".green(),
                        dex_mon.name.bold().color(match dex_mon.family {
                            Family::Scripting => "dark gray",
                            Family::Web => "red",
                            Family::Mobile => "green",
                            Family::Gaming => "blue",
                            Family::Database => "yellow",
                            Family::Systems => "cyan",
                            Family::Neural => "magenta",
                            Family::Ancient => "brown",
                        })
                    )
                );
                println!("{}", "+-----------------------------------------+".green());
            }
            save_party(&party)?;
        }
    }

    Ok(())
}

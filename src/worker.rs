use app::models::dex::{load_dex, DexMon, Family};
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

    // Check if input contains a command from the dex
    for line in reader.lines() {
        let command = line?;

        // Check if the command contains any of the known commands
        if let Some((_, dex_mon)) = command_map.iter().find(|(cmd, _)| command.contains(*cmd)) {
            let mut party = initialize_party().unwrap_or_else(|_| vec![]);

            // Skip if the monster is already in the party
            if party.iter().any(|mon| mon.dex_entry.id == dex_mon.id) {
                continue;
            }

            // Add monster to party
            party.push(PartyMon::new((*dex_mon).clone(), 1, (0, 100)));
            save_party(&party)?;

            // Notify user
            println!("{}", "-- Terminal Monsters Inc. --------".green());
            println!(
                "{}",
                format!(
                    "{} joined your party!",
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
                .green()
            );
            println!("{}", "----------------------------------".green());
        }
    }

    Ok(())
}

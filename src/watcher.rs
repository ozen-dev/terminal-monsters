use gitmons::models::dex::load_dex;
use gitmons::models::party::{get_random_party_mon, load_party, save_party};
use std::env;
use std::process::{exit, Command};

fn main() {
    // Collect all command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the command is a git command
    if args.len() > 1 && args[1] == "git" {
        handle_git_command(&args[2..]);
    } else {
        eprintln!("This script should be used to wrap git commands.");
        exit(1);
    }
}

fn handle_git_command(args: &[String]) {
    if args.contains(&"commit".to_string()) {
        if let Err(e) = collect_gitmon() {
            eprintln!("Failed to collect Gitmon: {}", e);
        }
    }

    let status = Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute git command");

    exit(status.code().unwrap_or(1));
}

fn collect_gitmon() -> Result<(), Box<dyn std::error::Error>> {
    let dex = load_dex();
    let mut party = load_party().unwrap_or_else(|_| vec![]);

    let new_party_mon = get_random_party_mon(&dex);
    party.push(new_party_mon);

    save_party(&party)?;
    Ok(())
}

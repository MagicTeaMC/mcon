use colored::Colorize;
use std::collections::HashMap;
use std::io::{self};
use std::path::Path;

mod properties;

// I should make this changable later (probably)
const DEFAULT_PROPERTIES_PATH: &str = "server.properties";

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!(
            "{} {} {}",
            "Usage:".red(),
            args[0].red().bold(),
            "[v/property=value ...]".red().bold()
        );
        println!("{}", "Examples:".cyan());
        println!(
            "  {} {}",
            args[0].green(),
            "difficulty=hard gamemode=survival".green()
        );
        println!(
            "  {} {}    {}",
            args[0].green(),
            "v".green(),
            "# View all current settings".dimmed()
        );
        return Ok(());
    }

    let properties_path = DEFAULT_PROPERTIES_PATH;

    if args.len() == 2 && args[1] == "v" {
        return properties::view_properties(properties_path);
    }

    // create a HashMap of properties to update
    let mut updates = HashMap::new();
    for arg in args.iter().skip(1) {
        if arg == "v" {
            continue; // will handle view after processing any updates
        }

        if let Some((key, value)) = arg.split_once('=') {
            updates.insert(key.to_string(), value.to_string());
        } else {
            println!(
                "{} {}",
                "Warning: Skipping invalid argument:".red().bold(),
                arg.red().bold()
            );
        }
    }

    // read existing properties or create an empty set
    let mut properties = if Path::new(properties_path).exists() {
        properties::read_properties(properties_path)?
    } else {
        println!(
            "{}",
            "No existing server.properties found. Creating a new one...".cyan()
        );
        HashMap::new()
    };

    // apply updates
    for (key, value) in updates {
        properties.insert(key.clone(), value.clone());
        println!(
            "{}{}{}{}",
            "Setting ".cyan(),
            key.yellow(),
            "=",
            value.purple()
        );
    }

    // write back to file
    properties::write_properties(properties_path, &properties)?;

    println!("{}", "Successfully updated server.properties".green());

    // If "v" is among the arguments, show the properties
    if args.iter().any(|arg| arg == "v") {
        println!("{}", "\nCurrent settings:".cyan());
        println!("{}", "----------------------------------".cyan());
        properties::display_properties(&properties);
    }

    Ok(())
}

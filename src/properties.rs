use chrono::Local;
use colored::Colorize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn read_properties(path: &str) -> io::Result<HashMap<String, String>> {
    let mut properties = HashMap::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // skip comments and empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            properties.insert(key.to_string(), value.to_string());
        }
    }

    Ok(properties)
}

pub fn write_properties(path: &str, properties: &HashMap<String, String>) -> io::Result<()> {
    let mut file = File::create(path)?;

    // add a header comment
    writeln!(file, "#Minecraft server properties")?;
    writeln!(file, "#{}", Local::now().format("%a %b %d %H:%M:%S %Z %Y"))?;
    writeln!(file, "#Generated/modified by MCON")?;

    // Get sorted keys for consistent output
    let mut keys: Vec<&String> = properties.keys().collect();
    keys.sort();

    // write all properties
    for key in keys {
        if let Some(value) = properties.get(key) {
            writeln!(file, "{}={}", key, value)?;
        }
    }

    Ok(())
}

pub fn view_properties(path: &str) -> io::Result<()> {
    if !Path::new(path).exists() {
        println!("{} {}", "File does not exist:".red(), path.red());
        return Ok(());
    }

    let properties = read_properties(path)?;

    if properties.is_empty() {
        println!("{} {}", "No properties found in:".red(), path.red());
        return Ok(());
    }

    println!("{}", "Current server.properties settings:".cyan());
    println!("{}", "----------------------------------".cyan());

    display_properties(&properties);

    Ok(())
}

pub fn display_properties(properties: &HashMap<String, String>) {
    // get sorted keys for consistent output
    let mut keys: Vec<&String> = properties.keys().collect();
    keys.sort();

    for key in keys {
        if let Some(value) = properties.get(key) {
            println!("{}{}{}", key.yellow(), "=", value.purple());
        }
    }
}

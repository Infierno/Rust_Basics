use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    id: u32,
    name: String,
    value: String,
}

use std::fs;
use serde_json;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "data.json";

    // 1. Read existing data (if any)
    let mut entries: Vec<Entry> = if fs::metadata(file_path).is_ok() {
        let json_string = fs::read_to_string(file_path)?;
        serde_json::from_str(&json_string)?
    } else {
        Vec::new() // Start with an empty vector if file doesn't exist
    };

    // 2. Create a new entry
    let new_entry = Entry {
        id: entries.len() as u32 + 1, // Simple ID generation
        name: "New Item".to_string(),
        value: "Some Value".to_string(),
    };

    // 3. Add the new entry
    entries.push(new_entry);

    // 4. Serialize the updated data
    let updated_json_string = serde_json::to_string_pretty(&entries)?;

    // 5. Write to the JSON file
    fs::write(file_path, updated_json_string)?;

    println!("Entry added and data saved to {}", file_path);
    
    // leave only ok... 
    // Read the Json File content
    let json_rederino = fs::read_to_string(file_path)?;
    
    // Parse the JSON string into a serde_json::Value 
    let json_valuerino: Value = serde_json::from_str(&json_rederino)?;

    // Print the entire JSON content 
    println!("Json content:\n{}", serde_json::to_string_pretty(&json_valuerino)?);


    Ok(())




}




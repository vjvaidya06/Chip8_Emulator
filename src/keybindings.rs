use std::collections::HashMap;

//Reads and verifies keybindings file, returns None otherwise
//CPU Object will set default keybindings if it receives None
pub fn get_keybindings() -> Option<HashMap<char, usize>>{
    let keybindings_path = "keybindings.json";
    let data = match std::fs::read_to_string(keybindings_path){
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to open keybindings.json file with error {e}");
            println!("Falling backs to defaults.");
            return None;
        }
    };
    match serde_json::from_str::<HashMap<char, usize>>(&data){
        Ok(map) if ((0..=15).all(|n| map.values().any(|&v| v == n))) => Some(map),
        Ok(_) => {
            println!("Keybinding values must include all numbers from 0 to 15 inclusive, falling back to default");
            None
        }
        Err(_) =>{
            println!("Failed to read keybindings from keybindings.json.");
            println!("Is it formatted correctly?");
            println!("It should map your chosen bindings to 0 to 15 inclusive");
            println!("Falling back to defaults.");
            None
        }
    }
}

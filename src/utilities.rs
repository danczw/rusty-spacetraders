pub mod util {
    use dirs::home_dir;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::process;

    // GAME STATUS UTILS

    pub fn set_home_dir_path(file_name: &str) -> PathBuf {
        let mut path = home_dir().unwrap();
        path.push(file_name);
        path
    }

    pub fn read_game(game_file_path: &PathBuf) -> HashMap<String, String> {
        // Read game file
        let saved = std::fs::read_to_string(game_file_path).unwrap_or("".to_string());

        // Parse game  to HashMap
        let mut game_status: HashMap<String, String> = HashMap::new();
        for line in saved.lines() {
            let mut line_iter = line.split("=");
            let key = line_iter.next().unwrap_or("").to_string();
            let value = line_iter.next().unwrap_or("").to_string();
            game_status.insert(key, value);
        }

        game_status
    }

    pub fn save_game(game_file_path: &PathBuf, game_status: &HashMap<String, String>) {
        let mut game_string = String::new();
        for (key, value) in game_status {
            game_string.push_str(&format!("{}={}\n", key, value));
        }

        std::fs::write(game_file_path, game_string).unwrap();
    }

    pub fn overwrite_status_consent(game_status: &HashMap<String, String>) {
        println!(
            "Current local callsign {} will be overwritten. Continue? (y/n)",
            game_status.get("callsign").unwrap()
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            println!("Aborting...");
            process::exit(1);
        }
    }
}

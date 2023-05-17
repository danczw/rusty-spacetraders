pub mod api;
pub mod commands;
pub mod utils;

use crate::api::process_command;
use crate::commands::cli;
use crate::utils::status;
use std::process::exit;

#[tokio::main]
async fn main() -> () {
    // parse command line arguments
    let matches = cli().get_matches();

    // set game status file path
    const GAME_FILE_NAME: &str = ".spacetraders";
    let game_file_path = status::set_home_dir_path(GAME_FILE_NAME);

    // read exising game status if available
    let mut game_status = status::read_game(&game_file_path);

    // initialize TradersApi struct for API calls
    let process_result = process_command(matches, &mut game_status).await;

    if process_result.is_err() {
        println!("Error: {}", process_result.unwrap_err());
        exit(1);
    }

    // save existing game status
    status::save_game(&game_file_path, &game_status);
}

pub mod api;
pub mod cli;
pub mod handler;
pub mod utils;

use crate::cli::cli;
use crate::handler::process_command;
use crate::utils::status;
use colored::*;
use std::process::exit;

#[tokio::main]
async fn main() {
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
        println!("{} {}", "Error! ".red(), process_result.unwrap_err());
        exit(1);
    }

    // save existing game status
    status::save_game(&game_file_path, &game_status);
}

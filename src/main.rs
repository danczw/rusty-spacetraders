pub mod api;
pub mod commands;
pub mod utils;

use crate::api::get_traders_api;
use crate::commands::cli;
use crate::utils::status::{read_game, save_game, set_home_dir_path};
use std::process::exit;

#[tokio::main]
async fn main() -> () {
    // parse command line arguments
    let matches = cli().get_matches();

    // set game status file path
    const GAME_FILE_NAME: &str = ".spacetraders";
    let game_file_path = set_home_dir_path(GAME_FILE_NAME);

    // read exising game status if available
    let mut game_status = read_game(&game_file_path);

    // initialize TradersApi struct for API calls
    let traders_api = get_traders_api();
    let process_result = traders_api.process_command(matches, &mut game_status).await;

    if process_result.is_err() {
        println!("Error: {}", process_result.unwrap_err());
        exit(1);
    }

    // save existing game status
    save_game(&game_file_path, &game_status);
}

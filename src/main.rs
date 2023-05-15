pub mod api;
pub mod commands;
pub mod utilities;

use crate::api::api::get_traders_api;
use crate::commands::cmnds::cli;
use crate::utilities::util::{read_game, save_game, set_home_dir_path};

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

    traders_api.process_command(matches, &mut game_status).await;

    // save existing game status
    save_game(&game_file_path, &game_status);
    // println!("{:#?}", game_status);
}

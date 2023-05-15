pub mod commands;
pub mod utilities;

use clap::Parser;

use crate::commands::cmds::init_traders_api;
use crate::utilities::util::{read_game, save_game, set_home_dir_path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    callsign: String,
    // #[arg(short, long)]
    // token: String,
}

#[tokio::main]
async fn main() {
    // parse command line arguments
    let args = Args::parse();

    println!("Hello {}!", args.callsign);

    // set game status file path
    const GAME_FILE_NAME: &str = ".spacetraders";
    let game_file_path = set_home_dir_path(GAME_FILE_NAME);

    // read exising game status
    let game_status = read_game(&game_file_path);
    println!("{:?}", game_status);

    // initialize TradersApi struct
    let traders_api = init_traders_api();

    // call api based on command line arguments
    traders_api.call_api(&args).await;

    // save existing game status
    save_game(&game_file_path, game_status)
}

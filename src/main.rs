pub mod gameflow;
pub mod playeragent;
pub mod tradersapi;
pub mod utilities;

use gameflow::game::ask_first_game;
use playeragent::agent::TradersAgent;
use tradersapi::api::init_traders_api;
use tradersapi::api::TradersApi;

#[tokio::main]
async fn main() {
    println!("Welcome to a rusty game of Space Traders!");
    let traders_api: TradersApi = init_traders_api();

    let my_agent: TradersAgent = ask_first_game(&traders_api).await;
    println!("Welcome, {}!", my_agent.callsign());
}

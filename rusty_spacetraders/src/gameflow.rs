pub mod game {
    use crate::playeragent::agent::init_player_agent;
    use crate::playeragent::agent::TradersAgent;
    use crate::tradersapi::api::TradersApi;
    use crate::utilities::utils::get_user_input;

    pub async fn ask_first_game(traders_api: &TradersApi) -> TradersAgent {
        let my_agent: TradersAgent = loop {
            // Ask if this is the first time playing
            println!("Is this your first time playing? (y/n)");
            let first_game_input: String = get_user_input();

            match first_game_input.as_str() {
                // If no, ask for a callsign
                "n" => {
                    break not_first_game(traders_api).await;
                }
                // If yes, ask for a callsign and register the agent
                "y" => {
                    break is_first_game(traders_api).await;
                }
                _ => println!("Wrong input..."),
            }
        };

        my_agent
    }

    async fn is_first_game(traders_api: &TradersApi) -> TradersAgent {
        println!("Welcome!");
        println!("What should your call sign be?");
        let new_callsign: String = get_user_input();

        // Create a new agent
        let my_agent = init_player_agent(new_callsign, "".to_string());
        // Register the agent
        my_agent.register_new_agent(traders_api).await;

        my_agent
    }

    async fn not_first_game(traders_api: &TradersApi) -> TradersAgent {
        let my_agent: TradersAgent = loop {
            // Ask for callsign
            println!("What is your call sign?");
            let existing_callsign: String = get_user_input();

            // Ask for token
            println!("What is your token?");
            let existing_token: String = get_user_input();

            // Get agent status from Space Traders
            let my_agent = init_player_agent(existing_callsign, existing_token.to_string());

            my_agent.get_agent_status(traders_api).await;

            if my_agent.token().is_empty() {
                println!("Wrong callsign or token...");
            } else {
                break my_agent;
            }
        };

        my_agent
    }
}

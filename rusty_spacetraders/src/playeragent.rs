pub mod agent {
    use crate::tradersapi::api::TradersApi;
    use reqwest::Client;
    use std::collections::HashMap;

    pub struct TradersAgent {
        pub callsign: String,
        pub token: String,
    }

    impl TradersAgent {
        // Immutable access to callsign via getter
        pub fn callsign(&self) -> &str {
            &self.callsign
        }

        // Immutable access to token via getter
        pub fn token(&self) -> &str {
            &self.token
        }
    }

    impl TradersAgent {
        pub async fn register_new_agent(&self, traders_api: &TradersApi) {
            // Build the URL
            let url = format!(
                "{}{}",
                traders_api.api_url_root(),
                traders_api.api_suburl_register()
            );

            // Register new agent with Space Traders
            let mut map = HashMap::new();
            map.insert("symbol", self.callsign());
            map.insert("faction", "COSMIC");

            let client: Client = reqwest::Client::new();
            let resp = client.post(url).json(&map).send().await;
            println!("{:#?}", resp);
        }

        pub async fn get_agent_status(&self, traders_api: &TradersApi) {
            // Build the URL
            let url = format!(
                "{}{}",
                traders_api.api_url_root(),
                traders_api.api_suburl_status()
            );

            // Get agent status from Space Traders
            let client: Client = reqwest::Client::new();
            let resp = client.get(url).send().await;
            println!("{:#?}", resp);
        }
    }

    pub fn init_player_agent(callsign: String, token: String) -> TradersAgent {
        // Initialize TradersAgent struct with default values
        TradersAgent {
            callsign: callsign,
            token: token,
        }
    }
}

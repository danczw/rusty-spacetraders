pub mod cmds {
    use super::super::Args;
    use reqwest::Client;
    use std::collections::HashMap;

    pub struct TradersApi {
        api_url_root: String,
        api_suburl_register: String,
        api_suburl_status: String,
    }

    impl TradersApi {
        // Immutable access to api_url_root via getter
        pub fn api_url_root(&self) -> &str {
            &self.api_url_root
        }

        // Immutable access to api_suburl_register via getter
        pub fn api_suburl_register(&self) -> &str {
            &self.api_suburl_register
        }

        // Immutable access to api_suburl_status via getter
        pub fn api_suburl_status(&self) -> &str {
            &self.api_suburl_status
        }
    }

    impl TradersApi {
        pub async fn call_api(&self, args: &Args) {
            if args.callsign.len() > 0 {
                println!("Registering new agent: {}", args.callsign);
                self.register_new_agent(&args.callsign).await;
            } else {
                println!("Getting agent status");
                self.get_agent_status().await;
            }
        }

        async fn register_new_agent(&self, callsign: &str) {
            // Build the URL
            let url = format!("{}{}", self.api_url_root(), self.api_suburl_register());

            // Register new agent with Space Traders
            let mut map = HashMap::new();
            map.insert("symbol", callsign);
            map.insert("faction", "COSMIC");

            let client: Client = reqwest::Client::new();
            let resp = client.post(url).json(&map).send().await;
            println!("{:#?}", resp);
        }

        async fn get_agent_status(&self) {
            // Build the URL
            let url = format!("{}{}", self.api_url_root(), self.api_suburl_status());

            // Get agent status from Space Traders
            let client: Client = reqwest::Client::new();
            let resp = client.get(url).send().await;
            println!("{:#?}", resp);
        }
    }

    pub fn init_traders_api() -> TradersApi {
        // Initialize TradersApi struct with default values
        TradersApi {
            api_url_root: "https://api.spacetraders.io/v2/".to_string(),
            api_suburl_register: "register".to_string(),
            api_suburl_status: "my/agent".to_string(),
        }
    }
}

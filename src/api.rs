pub mod api {
    use clap::ArgMatches;
    use reqwest::Client;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::process;

    use crate::utilities::util::overwrite_status_consent;

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
        pub async fn process_command(
            &self,
            matches: ArgMatches,
            game_status: &mut HashMap<String, String>,
        ) {
            // match subcommands and call api functions
            match matches.subcommand() {
                Some(("status", sub_matches)) => self.get_status(&game_status, sub_matches).await,

                Some(("new", sub_matches)) => {
                    let callsign = sub_matches.get_one::<String>("id_callsign").unwrap();
                    let _ = self.register_new_agent(game_status, callsign).await;
                }

                Some(("login", sub_matches)) => {
                    self.login_agent(game_status, sub_matches).await;
                }

                _ => println!("No command found."),
            }
        }

        pub async fn get_status(
            &self,
            game_status: &HashMap<String, String>,
            sub_matches: &ArgMatches,
        ) {
            // Check if local or remote status is requested
            if sub_matches.get_flag("id_local") {
                println!("Getting local status...");
                println!("{:#?}", game_status);
            } else if sub_matches.get_flag("id_remote") {
                println!("Getting remote status...");
                // TODO: implement remote status

                // Build the URL
                // let url = format!("{}{}", self.api_url_root(), self.api_suburl_status());

                // Get agent status from Space Traders
                // let client: Client = reqwest::Client::new();
                // let resp = client.get(url).send().await;

                // handle response
                // match resp {
                //     Ok(resp) => {
                //         if resp.status().is_success() {
                //             println!("{:#?}", resp);
                //         } else {
                //             println!("Failed to get online status");
                //             process::exit(1);
                //         }
                //     }
                //     Err(e) => {
                //         println!("Error getting status: {}", e);
                //         process::exit(1);
                //     }
                // }
                println!("Not yet implemented! :(")
            } else {
                println!("Add -l for local status and -r for remote status");
            }
        }

        pub async fn register_new_agent(
            &self,
            game_status: &mut HashMap<String, String>,
            callsign: &str,
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Check if local status already has a callsign
            overwrite_status_consent(game_status);

            // Build the URL
            let url = format!("{}{}", self.api_url_root(), self.api_suburl_register());

            // Register new agent with Space Traders
            let mut map = HashMap::new();
            map.insert("symbol", callsign);
            map.insert("faction", "COSMIC");

            let client: Client = reqwest::Client::new();
            let resp_text = client.post(url).json(&map).send().await?.text().await?;

            // deserialize response
            let resp_value: Value = serde_json::from_str(&resp_text)?;

            // check if is error
            if resp_value["error"]["code"] == 422 {
                println!(
                    "Failed to register new agent '{}': {}",
                    callsign, resp_value["error"]["data"]["symbol"][0]
                );
                process::exit(1);
            } else if resp_value["data"]["agent"]["symbol"] == callsign.to_uppercase() {
                println!("Successfully registered new agent '{}'!", callsign);
                game_status.insert("callsign".to_string(), callsign.to_string());
                game_status.insert("token".to_string(), resp_value["data"]["token"].to_string());
            } else {
                // TODO: handle other errors
                println!("Failed to register new agent '{}'", callsign);
                println!("{:#?}", resp_value);
                process::exit(1);
            }

            Ok(())
        }

        pub async fn login_agent(
            &self,
            game_status: &mut HashMap<String, String>,
            sub_matches: &ArgMatches,
        ) {
            // Check if local status already has a callsign
            overwrite_status_consent(game_status);
            println!("Logging in...");

            // Get callsign and token from command line
            let callsign = sub_matches.get_one::<String>("id_callsign").unwrap();
            let token = sub_matches.get_one::<String>("id_token").unwrap();

            // TODO: implement online status call fn

            // reset and update local status
            game_status.clear();
            game_status.insert("callsign".to_string(), callsign.to_string());
            game_status.insert("token".to_string(), token.to_string());
        }
    }

    pub fn get_traders_api() -> TradersApi {
        // Initialize TradersApi struct with default values
        TradersApi {
            api_url_root: "https://api.spacetraders.io/v2/".to_string(),
            api_suburl_register: "register".to_string(),
            api_suburl_status: "my/agent".to_string(),
        }
    }
}

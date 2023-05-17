use clap::ArgMatches;
use std::collections::HashMap;

use crate::utils::helpers::location_split;
use crate::utils::request as req;
use crate::utils::status::{check_local_token, overwrite_status_consent};

pub struct TradersApi {
    // TODO: move to request.rs
    api_url_root: String,
    api_suburl_register: String,
    api_suburl_status: String,
    api_suburl_location: String,
}

pub fn get_traders_api() -> TradersApi {
    // Initialize TradersApi struct with default values
    TradersApi {
        api_url_root: "https://api.spacetraders.io/v2/".to_string(),
        api_suburl_register: "register".to_string(),
        api_suburl_status: "my/agent".to_string(),
        api_suburl_location: "systems/".to_string(),
    }
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

    // Immutable access to api_suburl_location via getter
    pub fn api_suburl_location(&self) -> &str {
        &self.api_suburl_location
    }
}

impl TradersApi {
    pub async fn process_command(
        &self,
        matches: ArgMatches,
        game_status: &mut HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // match subcommands and call api functions
        match matches.subcommand() {
            Some(("status", sub_matches)) => {
                return self.get_status(&game_status, sub_matches).await;
            }

            Some(("new", sub_matches)) => {
                let callsign = sub_matches.get_one::<String>("id_callsign").unwrap();
                return self.register_new_agent(game_status, callsign).await;
            }

            Some(("login", sub_matches)) => {
                return self.login_agent(game_status, sub_matches).await;
            }

            Some(("location", sub_matches)) => {
                return self.view_location(&game_status, sub_matches).await;
            }

            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No command found.",
            ))),
        }
    }

    pub async fn get_status(
        &self,
        game_status: &HashMap<String, String>,
        sub_matches: &ArgMatches,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if local or remote status is requested
        if sub_matches.get_flag("id_local") {
            println!("Getting local status...");
            println!("callsign: {}", game_status.get("callsign").unwrap());
            println!("token:\n{}", game_status.get("token").unwrap());
            Ok(())
        } else if sub_matches.get_flag("id_remote") || !sub_matches.get_flag("id_local") {
            // Check if token is present
            if !check_local_token(game_status) {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No token found. Please login first.",
                )));
            }

            println!("Getting remote status...");
            // Build the URL
            let url = format!("{}{}", self.api_url_root(), self.api_suburl_status());
            let req_result = req::online_status_req(&game_status, url).await;

            if req_result.is_ok() {
                println!("{:#?}", req_result.unwrap()["data"]);
                return Ok(());
            } else {
                let req_result_err_msg = req_result.unwrap_err().to_string();
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    req_result_err_msg,
                )));
            }
        } else {
            // handle unknown error - should never happen ;)
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error getting status due to unknown reason.",
            )));
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
        return req::reg_agent_req(game_status, url, callsign).await;
    }

    pub async fn login_agent(
        &self,
        game_status: &mut HashMap<String, String>,
        sub_matches: &ArgMatches,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if local status already has a callsign
        overwrite_status_consent(game_status);
        println!("Logging in...");

        // Get callsign and token from command line
        let callsign = sub_matches.get_one::<String>("id_callsign").unwrap();
        let token = sub_matches.get_one::<String>("id_token").unwrap();

        // reset and update local status
        game_status.clear();
        game_status.insert("callsign".to_string(), callsign.to_string());
        game_status.insert("token".to_string(), token.to_string());

        // Get remote status
        let url = format!("{}{}", self.api_url_root(), self.api_suburl_status());
        let req_result = req::online_status_req(&game_status, url).await;

        match req_result {
            Ok(_) => {
                println!("Login successful!");

                // reset and update local status
                game_status.clear();
                game_status.insert("callsign".to_string(), callsign.to_string());
                game_status.insert("token".to_string(), token.to_string());
                println!("{:#?}", req_result.unwrap()["data"]);

                return Ok(());
            }
            Err(_) => {
                let req_result_err_msg = req_result.unwrap_err().to_string();
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    req_result_err_msg,
                )));
            }
        }
    }

    pub async fn view_location(
        &self,
        game_status: &HashMap<String, String>,
        sub_matches: &ArgMatches,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if token is present
        if !check_local_token(game_status) {
            // TODO: move to function
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No token found. Please login first.",
            )));
        }

        if sub_matches.contains_id("id_waypoint") {
            let location_passed = sub_matches.get_one::<String>("id_waypoint").unwrap();
            println!("Getting data for waypoint {}...", location_passed);

            // Divide provided location into system and waypoint coords
            let sys_waypoint_tup = location_split(location_passed);

            // Get waypoint data
            let url = format!(
                "{}{}/{}/waypoints/{}",
                self.api_url_root(),
                self.api_suburl_location(),
                sys_waypoint_tup.0,
                sys_waypoint_tup.1
            );

            let _ = req::location_req(game_status, url).await;
            return Ok(());
        } else {
            println!("Getting data for headquarter waypoint...");
            // Get remote status
            let url = format!("{}{}", self.api_url_root(), self.api_suburl_status());
            let status_req_result = req::online_status_req(&game_status, url).await;

            match status_req_result {
                Ok(status_req_result) => {
                    let hq_location = status_req_result["data"]
                        .get("headquarters")
                        .unwrap()
                        .to_string();
                    // Divide provided location into system and waypoint coords
                    let sys_waypoint_tup = location_split(&hq_location);

                    println!("Headquarter detected at {}...", hq_location);
                    // Get waypoint data
                    let url = format!(
                        "{}{}/{}/waypoints/{}",
                        self.api_url_root(),
                        self.api_suburl_location(),
                        sys_waypoint_tup.0,
                        sys_waypoint_tup.1
                    );

                    let _ = req::location_req(game_status, url).await;

                    return Ok(());
                }
                Err(status_req_result) => {
                    let status_req_result_err_msg = status_req_result.to_string();
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        status_req_result_err_msg,
                    )));
                }
            }
        }
    }
}

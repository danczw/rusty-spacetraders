use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct TradersApi {
    api_url_root: String,
    api_suburl_register: String,
    api_suburl_status: String,
    api_suburl_location: String,
    api_suburl_contracts: String,
}

pub fn get_traders_api() -> TradersApi {
    // Initialize TradersApi struct with default values
    TradersApi {
        api_url_root: "https://api.spacetraders.io/v2/".to_string(),
        api_suburl_register: "register".to_string(),
        api_suburl_status: "my/agent".to_string(),
        api_suburl_location: "systems".to_string(),
        api_suburl_contracts: "my/contracts".to_string(),
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

    // Immutable access to api_suburl_contracts via getter
    pub fn api_suburl_contracts(&self) -> &str {
        &self.api_suburl_contracts
    }
}

impl TradersApi {
    pub async fn remote_status_req(
        &self,
        game_status: &HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Build the URL
        let url = format!("{}{}", self.api_url_root(), self.api_suburl_status());

        // Get agent status from Space Traders
        let client: Client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?
            .text()
            .await?;
        let resp_value: Value = serde_json::from_str(&resp)?;

        // check response
        if resp_value["data"]["symbol"].is_string() {
            return Ok(resp_value);
        } else if resp_value["error"]["code"].is_number() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting remote status - {}",
                    resp_value["error"]["message"]
                        .to_string()
                        .replace("\\\"", "")
                ),
            )));
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting remote status due to unforeseen reason - {}",
                    resp_value["error"]["message"]
                ),
            )));
        }
    }

    pub async fn reg_agent_req(
        &self,
        game_status: &mut HashMap<String, String>,
        callsign: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Build url
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
        if resp_value["data"]["agent"]["symbol"] == callsign.to_uppercase() {
            game_status.insert("callsign".to_string(), callsign.to_string());
            game_status.insert(
                "token".to_string(),
                resp_value["data"]["token"]
                    .to_string()
                    .trim_matches('"')
                    .to_string(),
            );
            println!("Registered new agent '{}'.", callsign);
            println!("{:#?}", resp_value);
            return Ok(());
        } else if resp_value["error"]["code"].is_number() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to register new agent '{}': {}",
                    callsign, resp_value["error"]["data"]["symbol"][0]
                ),
            )));
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error registering new agent {} due to unforeseen reason - {}",
                    callsign, resp_value["error"]["message"]
                ),
            )));
        }
    }

    pub async fn loc_waypoint_req(
        &self,
        game_status: &HashMap<String, String>,
        sys_waypoint_tup: (String, String),
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Build url
        let url = format!(
            "{}{}/{}/waypoints/{}",
            self.api_url_root(),
            self.api_suburl_location(),
            sys_waypoint_tup.0,
            sys_waypoint_tup.1
        );

        // Get waypoint data from Space Traders
        let client: Client = reqwest::Client::new();
        let resp_text = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?
            .text()
            .await?;
        let resp_value: Value = serde_json::from_str(&resp_text)?;

        // check if is error
        if resp_value["data"]["symbol"].is_string() {
            println!("{:#?}", resp_value["data"]);
            return Ok(());
        } else if resp_value["error"]["code"].is_number() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting location - {}",
                    resp_value["error"]["message"]
                ),
            )));
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting waypoint data due to unforeseen reason - {}",
                    resp_value["error"]["message"]
                ),
            )));
        }
    }

    pub async fn loc_system_req(
        &self,
        game_status: &HashMap<String, String>,
        sys_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Build url
        let url = format!(
            "{}{}/{}/waypoints",
            self.api_url_root(),
            self.api_suburl_location(),
            sys_name
        );

        // Get system data from Space Traders
        let client: Client = reqwest::Client::new();
        let resp_text = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?
            .text()
            .await?;
        let resp_value: Value = serde_json::from_str(&resp_text)?;

        // check if is error
        if resp_value["data"].is_array() {
            println!("{:#?}", resp_value["data"]);
            return Ok(());
        } else if resp_value["error"]["code"].is_number() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting location - {}",
                    resp_value["error"]["message"]
                ),
            )));
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting system data due to unforeseen reason - {}",
                    resp_value["error"]["message"]
                ),
            )));
        }
    }

    pub async fn all_contracts_req(
        &self,
        game_status: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Build url
        let url = format!("{}{}", self.api_url_root(), self.api_suburl_contracts());

        let client: Client = reqwest::Client::new();
        let resp_text = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?
            .text()
            .await?;
        let resp_value: Value = serde_json::from_str(&resp_text)?;

        if resp_value["data"].is_array() {
            println!("{:#?}", resp_value);
            return Ok(());
        } else if resp_value["error"]["code"].is_number() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting all contracts - {}",
                    resp_value["error"]["message"]
                ),
            )));
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error getting waypoint data due to unforeseen reason - {}",
                    resp_value["error"]["message"]
                ),
            )));
        }
    }
}

use reqwest::{Client, StatusCode};
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
        api_suburl_register: "register/".to_string(),
        api_suburl_status: "my/agent/".to_string(),
        api_suburl_location: "systems/".to_string(),
        api_suburl_contracts: "my/contracts/".to_string(),
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

    // Response checker: check response status and returns appropriate data or error
    pub async fn check_response(
        &self,
        response: reqwest::Response,
        error_msg: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Get response status
        let resp_status = response.status();

        // Get response text
        let resp_text = response.text().await?;

        // Deserialize response text
        let resp_value: Value = serde_json::from_str(&resp_text)?;

        // check response
        match resp_status {
            StatusCode::OK => Ok(resp_value),
            StatusCode::CREATED => Ok(resp_value),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "{} - {}",
                    error_msg,
                    resp_value["error"]["message"]
                        .to_string()
                        .replace("\\\"", "")
                ),
            ))),
        }
    }
}

impl TradersApi {
    pub async fn remote_status_req(
        &self,
        game_status: &HashMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Build the URL
        let url = format!("{}{}", self.api_url_root(), self.api_suburl_status());

        // Initialize client and send request
        let client: Client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?;

        // check response
        self.check_response(resp, "Error getting remote status")
            .await
    }

    pub async fn reg_agent_req(&self, callsign: &str) -> Result<Value, Box<dyn std::error::Error>> {
        // Build url
        let url = format!("{}{}", self.api_url_root(), self.api_suburl_register());

        // Build request body with callsign and faction
        let mut map = HashMap::new();
        map.insert("symbol", callsign);
        map.insert("faction", "COSMIC");

        // Initialize client and send request
        let client: Client = reqwest::Client::new();
        let resp = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&map)
            .send()
            .await?;

        // check response
        self.check_response(resp, "Error registering new agent")
            .await
    }

    pub async fn loc_waypoint_req(
        &self,
        game_status: &HashMap<String, String>,
        sys_waypoint_tup: (String, String),
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Build url
        let url = format!(
            "{}{}{}/waypoints/{}",
            self.api_url_root(),
            self.api_suburl_location(),
            sys_waypoint_tup.0,
            sys_waypoint_tup.1
        );

        // Initialize client and send request
        let client: Client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?;

        // Check response
        self.check_response(resp, "Error getting waypoint data")
            .await
    }

    pub async fn loc_system_req(
        &self,
        game_status: &HashMap<String, String>,
        sys_name: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Build url
        let url = format!(
            "{}{}{}/waypoints",
            self.api_url_root(),
            self.api_suburl_location(),
            sys_name
        );

        // Initialize client and send request
        let client: Client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?;

        // Check response
        self.check_response(resp, "Error getting system data").await
    }

    pub async fn contract_data_req(
        &self,
        game_status: &HashMap<String, String>,
        contract_id: Option<&String>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Build url
        let url: String = match contract_id {
            None => {
                format!("{}{}", self.api_url_root(), self.api_suburl_contracts())
            }
            Some(id) => {
                format!(
                    "{}{}/{}",
                    self.api_url_root(),
                    self.api_suburl_contracts(),
                    id
                )
            }
        };

        // Initialize client and send request
        let client: Client = reqwest::Client::new();
        let resp_text = client
            .get(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .send()
            .await?;

        // Check response
        self.check_response(resp_text, "Error getting contract data")
            .await
    }

    pub async fn contract_interact_req(
        &self,
        game_status: &HashMap<String, String>,
        contract_id: &str,
        interact_type: &str,
        request_body: HashMap<&str, &str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Build url
        let base_url = format!(
            "{}{}/{}",
            self.api_url_root(),
            self.api_suburl_contracts(),
            contract_id
        );
        let url = match interact_type {
            "accept" => format!("{}/accept", base_url),
            "fulfill" => format!("{}/fulfill", base_url),
            _ => panic!("Invalid contract interaction type"),
        };

        // Initialize client and send request
        let client: Client = reqwest::Client::new();
        let resp_text = client
            .post(url)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + game_status.get("token").unwrap(),
            )
            .header("Accept", "application/json")
            .json(&request_body)
            .send()
            .await?;

        // Check response
        self.check_response(resp_text, "Error interacting with contract")
            .await
    }
}

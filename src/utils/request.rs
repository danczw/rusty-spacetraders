use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub async fn online_status_req(
    game_status: &HashMap<String, String>,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
        println!("{:#?}", resp_value["data"]);
        return Ok(());
    } else if resp_value["error"]["code"].is_number() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Error getting online status: {}",
                resp_value["error"]["message"]
            ),
        )));
    } else {
        // TODO: better handle other errors
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Error getting online status due to unforeseen reason: {}",
                resp_value["error"]["message"]
            ),
        )));
    }
}

pub async fn reg_agent_req(
    game_status: &mut HashMap<String, String>,
    url: String,
    callsign: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
        game_status.insert("token".to_string(), resp_value["data"]["token"].to_string());
        println!("Registered new agent '{}'.", callsign);
        println!("{:#?}", resp_value);
        return Ok(());
    } else if resp_value["error"]["code"] == 422 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to register new agent '{}': {}",
                callsign, resp_value["error"]["data"]["symbol"][0]
            ),
        )));
    } else {
        // TODO: better handle other errors
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Error registering new agent {} due to unforeseen reason: {}",
                callsign, resp_value["error"]["message"]
            ),
        )));
    }
}

use colored::Colorize;
use serde_json::Value;

pub fn location_split(location: &str) -> (String, String) {
    // Divide provided location into system and waypoint coords
    // input X1-DF55-20250Z to system: X1-DF55 and waypoint: 20250Z
    let location_split: Vec<&str> = location.split('-').collect();
    let system = format!(
        "{}-{}",
        location_split[0].trim_matches('"'),
        location_split[1].trim_matches('"')
    );
    let waypoint = location.trim_matches('"').to_string();

    (system, waypoint)
}

pub fn no_token_error() -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No token found. Please login first.",
    )))
}

pub fn act_on_req_result(
    req_result: Result<Value, Box<dyn std::error::Error>>,
    print_statement: &str,
    print_data: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match req_result {
        Ok(data) => {
            println!("{}", print_statement.green());
            if print_data {
                println!("{:#?}", data["data"]);
            }
            Ok(())
        }
        Err(data) => {
            let err_msg = data.to_string();
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err_msg,
            )))
        }
    }
}

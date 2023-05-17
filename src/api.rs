use clap::ArgMatches;
use colored::*;
use std::collections::HashMap;
use std::io;

use crate::utils::helpers as hlp;
use crate::utils::request as req;
use crate::utils::status;

pub async fn process_command(
    matches: ArgMatches,
    game_status: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // initialize TradersApi struct for API calls
    let api = req::get_traders_api();

    // match subcommands and call api functions
    match matches.subcommand() {
        Some(("status", sub_matches)) => {
            return get_status(api, game_status, sub_matches).await;
        }

        Some(("new", sub_matches)) => {
            return register_new_agent(api, game_status, sub_matches).await;
        }

        Some(("login", sub_matches)) => {
            return login_agent(api, game_status, sub_matches).await;
        }

        Some(("location", sub_matches)) => {
            return view_location(api, game_status, sub_matches).await;
        }

        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No command found.",
        ))),
    }
}

pub async fn get_status(
    api: req::TradersApi,
    game_status: &HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if local or remote status is requested
    if sub_matches.get_flag("id_local") {
        println!("Getting local status...");
        println!(
            "{} {}",
            "callsign: ".green(),
            game_status.get("callsign").unwrap()
        );
        println!(
            "{} {}",
            "token: ".green(),
            game_status.get("token").unwrap()
        );
        Ok(())
    } else if sub_matches.get_flag("id_remote") || !sub_matches.get_flag("id_local") {
        // Check if token is present
        if !status::check_local_token(game_status) {
            return hlp::no_token_error();
        }

        println!("Getting remote status...");
        let req_result = api.remote_status_req(&game_status).await;
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
    api: req::TradersApi,
    game_status: &mut HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if local status already has a callsign
    status::overwrite_status_consent(game_status);

    // Get callsign from command line argument and register new agent
    let callsign = sub_matches.get_one::<String>("id_callsign").unwrap();
    return api.reg_agent_req(game_status, callsign).await;
}

pub async fn login_agent(
    api: req::TradersApi,
    game_status: &mut HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ask for user for token
    println!("Enter token: ");
    let mut token = String::new();
    io::stdin()
        .read_line(&mut token)
        .expect("Failed to read line");
    let token = match token.trim().parse() {
        Ok(tkn) => tkn,
        Err(msg) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                msg,
            )))
        }
    };

    // Check if local status already has a callsign
    status::overwrite_status_consent(game_status);
    println!("Logging in...");

    // Get callsign from command line argument
    let callsign = sub_matches.get_one::<String>("id_callsign").unwrap();

    // Update local status and get remote status
    let game_status = status::reset_local_status(game_status, callsign.to_string(), token);
    let req_result = api.remote_status_req(&game_status).await;

    // Check if login was successful
    match req_result {
        Ok(_) => {
            println!("{}", "Login successful!".green());
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
    api: req::TradersApi,
    game_status: &HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if token is present
    if !status::check_local_token(game_status) {
        return hlp::no_token_error();
    }

    if sub_matches.contains_id("id_waypoint") {
        // Get waypoint location from command line argument
        let location_passed = sub_matches.get_one::<String>("id_waypoint").unwrap();
        println!("Getting data for waypoint {}...", location_passed);

        // Divide provided location into system and waypoint coords
        let sys_waypoint_tup = hlp::location_split(location_passed);

        // Get waypoint data
        let _ = api.location_req(game_status, sys_waypoint_tup).await;
        return Ok(());
    } else {
        println!("Getting data for headquarter waypoint...");
        // Get remote status
        let status_req_result = api.remote_status_req(&game_status).await;

        // Check if location view request was successful
        match status_req_result {
            Ok(status_req_result) => {
                let hq_location = status_req_result["data"]
                    .get("headquarters")
                    .unwrap()
                    .to_string();
                println!("Headquarter detected at {}...", hq_location);

                // Divide provided location into system and waypoint coords
                let sys_waypoint_tup = hlp::location_split(&hq_location);

                // Get waypoint data
                let _ = api.location_req(game_status, sys_waypoint_tup).await;
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

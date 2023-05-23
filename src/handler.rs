use clap::ArgMatches;
use colored::*;
use std::collections::HashMap;
use std::io;

use crate::api::api;
use crate::cli::ALL_COMMANDS;
use crate::utils::helpers as hlp;
use crate::utils::status;

pub async fn process_command(
    matches: ArgMatches,
    game_status: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // initialize TradersApi struct for API calls
    let api = api::get_traders_api();

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

        Some(("contract", sub_matches)) => {
            return view_contract(api, game_status, sub_matches).await;
        }

        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No command found.",
        ))),
    }
}

pub async fn get_status(
    api: api::TradersApi,
    game_status: &HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if local or remote status is requested
    if sub_matches.get_flag(ALL_COMMANDS.arg_local.1) {
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
    } else if sub_matches.get_flag(ALL_COMMANDS.arg_remote.1)
        || !sub_matches.get_flag(ALL_COMMANDS.arg_local.1)
    {
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
    api: api::TradersApi,
    game_status: &mut HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if local status already has a callsign
    status::overwrite_status_consent(game_status);

    // Get callsign from command line argument and register new agent
    let callsign = sub_matches
        .get_one::<String>(ALL_COMMANDS.arg_callsign.1)
        .unwrap();
    return api.reg_agent_req(game_status, callsign).await;
}

pub async fn login_agent(
    api: api::TradersApi,
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
    let callsign = sub_matches
        .get_one::<String>(ALL_COMMANDS.arg_callsign.1)
        .unwrap();

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
    api: api::TradersApi,
    game_status: &HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if token is present
    if !status::check_local_token(game_status) {
        return hlp::no_token_error();
    }

    if sub_matches.contains_id(ALL_COMMANDS.arg_waypoint.1) {
        // Get waypoint location from command line argument
        let waypoint_passed = sub_matches
            .get_one::<String>(ALL_COMMANDS.arg_waypoint.1)
            .unwrap();
        println!("Getting data for waypoint {}...", waypoint_passed);

        // Divide provided location into system and waypoint coords
        let sys_waypoint_tup = hlp::location_split(waypoint_passed);

        // Get waypoint data
        let _ = api.loc_waypoint_req(game_status, sys_waypoint_tup).await;
        return Ok(());
    } else if sub_matches.contains_id(ALL_COMMANDS.arg_system.1) {
        // Get system location from command line argument
        let system_passed = sub_matches
            .get_one::<String>(ALL_COMMANDS.arg_system.1)
            .unwrap();
        println!("Getting data for system {}...", system_passed);

        // Get system data
        let _ = api.loc_system_req(game_status, system_passed).await;
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
                let _ = api.loc_waypoint_req(game_status, sys_waypoint_tup).await;
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

pub async fn view_contract(
    api: api::TradersApi,
    game_status: &HashMap<String, String>,
    _sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if token is present
    if !status::check_local_token(game_status) {
        return hlp::no_token_error();
    }

    // Get contract data
    println!("Getting data for all your contracts...");
    let _ = api.all_contracts_req(game_status).await;
    return Ok(());
}

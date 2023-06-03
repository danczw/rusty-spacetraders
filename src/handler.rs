use clap::ArgMatches;
use colored::*;
use std::collections::HashMap;
use std::io;

use crate::api::requests;
use crate::cli::ALL_COMMANDS;
use crate::utils::helpers as hlp;
use crate::utils::status;

pub async fn process_command(
    matches: ArgMatches,
    game_status: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // initialize TradersApi struct for API calls
    let api = requests::get_traders_api();

    // match subcommands and call api functions
    match matches.subcommand() {
        Some(("status", sub_matches)) => get_status(api, game_status, sub_matches).await,
        Some(("new", sub_matches)) => register_new_agent(api, game_status, sub_matches).await,
        Some(("login", sub_matches)) => login_agent(api, game_status, sub_matches).await,
        Some(("location", sub_matches)) => view_location(api, game_status, sub_matches).await,
        Some(("contract", sub_matches)) => view_contract(api, game_status, sub_matches).await,
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No command found.",
        ))),
    }
}

pub async fn get_status(
    api: requests::TradersApi,
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
        let req_result = api.remote_status_req(game_status).await;

        // Check if request was successful
        hlp::act_on_req_result(req_result, "Retrieval successful!", true)
    } else {
        // handle unknown error - should never happen ;)
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error getting status due to unknown reason.",
        )));
    }
}

pub async fn register_new_agent(
    api: requests::TradersApi,
    game_status: &mut HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if local status already has a callsign
    status::overwrite_status_consent(game_status);

    // Get callsign from command line argument and register new agent
    let callsign = sub_matches
        .get_one::<String>(ALL_COMMANDS.arg_callsign.1)
        .unwrap();
    let req_resp = api.reg_agent_req(callsign).await;

    // Check if registration was successful
    match req_resp {
        Ok(resp_value) => {
            // Update local status
            game_status.insert("callsign".to_string(), callsign.to_string());
            game_status.insert(
                "token".to_string(),
                resp_value["data"]["token"]
                    .to_string()
                    .trim_matches('"')
                    .to_string(),
            );
            println!("{}", "Registration successful!".green());
            println!("Registered new agent '{}'.", callsign);
            println!("{:#?}", resp_value);
            Ok(())
        }
        Err(req_result) => {
            let req_result_err_msg = req_result.to_string();
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                req_result_err_msg,
            )))
        }
    }
}

pub async fn login_agent(
    api: requests::TradersApi,
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
    let req_result = api.remote_status_req(game_status).await;

    // Check if login was successful
    hlp::act_on_req_result(req_result, "Login successful!", true)
}

pub async fn view_location(
    api: requests::TradersApi,
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
        let req_result = api.loc_waypoint_req(game_status, sys_waypoint_tup).await;

        // Check if location view request was successful
        hlp::act_on_req_result(req_result, "Retrieval successful!", true)
    } else if sub_matches.contains_id(ALL_COMMANDS.arg_system.1) {
        // Get system location from command line argument
        let system_passed = sub_matches
            .get_one::<String>(ALL_COMMANDS.arg_system.1)
            .unwrap();
        println!("Getting data for system {}...", system_passed);

        // Get system data
        let req_result = api.loc_system_req(game_status, system_passed).await;

        // Check if location view request was successful
        hlp::act_on_req_result(req_result, "Retrieval successful!", true)
    } else {
        println!("Getting data for headquarter waypoint...");
        // Get remote status
        let status_req_result = api.remote_status_req(game_status).await;

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
                let loc_req_result = api.loc_waypoint_req(game_status, sys_waypoint_tup).await;

                // Check if location view request was successful
                hlp::act_on_req_result(loc_req_result, "Retrieval successful!", true)
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
    api: requests::TradersApi,
    game_status: &HashMap<String, String>,
    sub_matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if token is present
    if !status::check_local_token(game_status) {
        return hlp::no_token_error();
    }

    // match passed argument to argument id of ALL_COMMANDS and call appropriate function
    if sub_matches.contains_id(ALL_COMMANDS.arg_id.1) {
        // Get contract id from command line argument
        let contract_id = sub_matches
            .get_one::<String>(ALL_COMMANDS.arg_id.1)
            .unwrap();
        println!("Getting data for contract {}...", contract_id);

        // Get contract data
        let req_result = api.contract_data_req(game_status, Some(contract_id)).await;

        // Check if contract data request was successful
        hlp::act_on_req_result(req_result, "Retrieval successful!", true)
    } else if sub_matches.contains_id(ALL_COMMANDS.arg_accept.1) {
        // Get contract id from command line argument
        let contract_id = sub_matches
            .get_one::<String>(ALL_COMMANDS.arg_accept.1)
            .unwrap();
        println!("Accepting contract {}...", contract_id);

        // Accept contract
        let map: HashMap<&str, &str> = HashMap::new();
        let req_result = api
            .contract_interact_req(game_status, contract_id, "accept", map)
            .await;

        // Check if contract was accepted
        hlp::act_on_req_result(req_result, "Contract accepted!", true)
    } else if sub_matches.contains_id(ALL_COMMANDS.arg_fulfill.1) {
        // Get contract id from command line argument
        let contract_id = sub_matches
            .get_one::<String>(ALL_COMMANDS.arg_fulfill.1)
            .unwrap();
        println!("Fulfilling contract {}...", contract_id);

        // Fulfill contract
        let map: HashMap<&str, &str> = HashMap::new();
        let req_result = api
            .contract_interact_req(game_status, contract_id, "fulfill", map)
            .await;

        // Check if contract was fulfilled
        hlp::act_on_req_result(req_result, "Contract fulfilled!", true)
    } else {
        // Get all contracts data
        println!("Getting data for all your contracts...");
        let req_result = api.contract_data_req(game_status, None).await;

        // Check if contract data was retrieved
        hlp::act_on_req_result(req_result, "Retrieval successful!", true)
    }
}

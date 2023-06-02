use clap::{Arg, ArgAction, Command};

// define the command strings with a static str for each command
// and a tuple for each Argument: (name/long, id, short)
pub struct CommandStrings {
    // subcommands
    pub sc_contract: &'static str,
    pub sc_location: &'static str,
    pub sc_login: &'static str,
    pub sc_new: &'static str,
    pub sc_status: &'static str,
    // Args
    pub arg_accept: (&'static str, &'static str, char),
    pub arg_callsign: (&'static str, &'static str, char),
    pub arg_fulfill: (&'static str, &'static str, char),
    pub arg_id: (&'static str, &'static str, char),
    pub arg_local: (&'static str, &'static str, char),
    pub arg_remote: (&'static str, &'static str, char),
    pub arg_system: (&'static str, &'static str, char),
    pub arg_waypoint: (&'static str, &'static str, char),
}

pub static ALL_COMMANDS: CommandStrings = CommandStrings {
    // subcommands
    sc_contract: "contract",
    sc_location: "location",
    sc_login: "login",
    sc_new: "new",
    sc_status: "status",
    // Args
    arg_accept: ("accept", "id_accept", 'a'),
    arg_callsign: ("callsign", "id_callsign", 'c'),
    arg_fulfill: ("fulfill", "id_fulfill", 'f'),
    arg_id: ("id", "id_id", 'i'),
    arg_local: ("local", "id_local", 'l'),
    arg_remote: ("remote", "id_remote", 'r'),
    arg_system: ("system", "id_system", 's'),
    arg_waypoint: ("waypoint", "id_waypoint", 'w'),
};

pub fn cli() -> Command {
    Command::new("rst")
            .about("A rust based SpaceTraders CLI.")
            .arg_required_else_help(true)
            .subcommand_required(true)
            // subcommand for local status
            .subcommand(
                Command::new(ALL_COMMANDS.sc_status)
                    .about("Get the status of the game. Defaults to remote (online) status.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_local.0)
                            .help("To get the local saved status of the game: callsign and token.")
                            .id(ALL_COMMANDS.arg_local.1)
                            .short(ALL_COMMANDS.arg_local.2)
                            .long(ALL_COMMANDS.arg_local.0)
                            .exclusive(true)
                            .action(ArgAction::SetTrue)
                    )
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_remote.0)
                            .help("To get the remote (online) status of the game.")
                            .id(ALL_COMMANDS.arg_remote.1)
                            .short(ALL_COMMANDS.arg_remote.2)
                            .long(ALL_COMMANDS.arg_remote.0)
                            .exclusive(true)
                            .action(ArgAction::SetTrue)
                    )
            )
            // subcommand for new game
            .subcommand(
                Command::new(ALL_COMMANDS.sc_new)
                    .about("Register a new agent with Space Traders. Will overwrite existing local game status.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_callsign.0)
                            .help("The callsign for a new agent to register.")
                            .id(ALL_COMMANDS.arg_callsign.1)
                            .short(ALL_COMMANDS.arg_callsign.2)
                            .long(ALL_COMMANDS.arg_callsign.0)
                            .required(true)
                    )
                    .arg_required_else_help(true),
                    // TODO: add flag for faction and email:
                    // https://spacetraders.stoplight.io/docs/spacetraders/86ed6bbe4f5d7-register-new-agent
            )
            // manually set local game status
            .subcommand(
                Command::new(ALL_COMMANDS.sc_login)
                    .about("Login to an existing agent. Will overwrite existing local game status.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_callsign.0)
                            .help("The callsign of an existing agent to login with.")
                            .id(ALL_COMMANDS.arg_callsign.1)
                            .short(ALL_COMMANDS.arg_callsign.2)
                            .long(ALL_COMMANDS.arg_callsign.0)
                            .required(true)
                    )
                    .arg_required_else_help(true)
            )
            // check waypoint
            .subcommand(
                Command::new(ALL_COMMANDS.sc_location)
                    .about("View locations data. Defaults to view agent headquarter.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_waypoint.0)
                            .help("The waypoint to get data for, e.g., X1-DF55-20250Z.")
                            .id(ALL_COMMANDS.arg_waypoint.1)
                            .short(ALL_COMMANDS.arg_waypoint.2)
                            .long(ALL_COMMANDS.arg_waypoint.0)
                            .action(ArgAction::Set)
                            .exclusive(true)
                    )
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_system.0)
                            .help("The system to view all waypoint data for, e.g., X1-VS75.")
                            .id(ALL_COMMANDS.arg_system.1)
                            .short(ALL_COMMANDS.arg_system.2)
                            .long(ALL_COMMANDS.arg_system.0)
                            .action(ArgAction::Set)
                            .exclusive(true)
                    )
            )
            // check contracts
            .subcommand(
                Command::new(ALL_COMMANDS.sc_contract)
                    .about("Interact with contracts. Defaults to view all contracts given to the agent.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_id.0)
                            .help("The contract ID to view data for, e.g., clhzd3zrx1sufs60dc58k5vyj")
                            .id(ALL_COMMANDS.arg_id.1)
                            .short(ALL_COMMANDS.arg_id.2)
                            .long(ALL_COMMANDS.arg_id.0)
                            .action(ArgAction::Set)
                            .exclusive(true)
                    )
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_accept.0)
                            .help("The ID of a contract to accept, e.g., clhzd3zrx1sufs60dc58k5vyj")
                            .id(ALL_COMMANDS.arg_accept.1)
                            .short(ALL_COMMANDS.arg_accept.2)
                            .long(ALL_COMMANDS.arg_accept.0)
                            .action(ArgAction::Set)
                            .exclusive(true)
                    )
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_fulfill.0)
                            .help("The ID of a contract to fulfill, e.g., clhzd3zrx1sufs60dc58k5vyj")
                            .id(ALL_COMMANDS.arg_fulfill.1)
                            .short(ALL_COMMANDS.arg_fulfill.2)
                            .long(ALL_COMMANDS.arg_fulfill.0)
                            .action(ArgAction::Set)
                            .exclusive(true)
                    )
            )
}

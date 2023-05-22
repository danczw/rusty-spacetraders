use clap::{Arg, ArgAction, Command};

// define the command strings with a static str for each command
// and a tuple for each Argument: (name/long, id, short)
struct CommandStrings {
    // subcommands
    sc_status: &'static str,
    sc_new: &'static str,
    sc_login: &'static str,
    sc_location: &'static str,
    // Args
    arg_local: (&'static str, &'static str, char),
    arg_remote: (&'static str, &'static str, char),
    arg_callsign: (&'static str, &'static str, char),
    arg_waypoint: (&'static str, &'static str, char),
}

static ALL_COMMANDS: CommandStrings = CommandStrings {
    // subcommands
    sc_status: "status",
    sc_new: "new",
    sc_login: "login",
    sc_location: "location",
    // Args
    arg_local: ("local", "id_local", 'l'),
    arg_remote: ("remote", "id_remote", 'r'),
    arg_callsign: ("callsign", "id_callsign", 'c'),
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
                    .about("Get the status of the game, add -l for local saved status and -r for remote (online) status.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_local.0)
                            .help("Get the local saved status of the game: callsign and token.")
                            .id(ALL_COMMANDS.arg_local.1)
                            .short(ALL_COMMANDS.arg_local.2)
                            .long(ALL_COMMANDS.arg_local.0)
                            .exclusive(true)
                            .action(ArgAction::SetTrue)
                    )
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_remote.0)
                            .help("Get the remote (online) status of the game.")
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
                            .help("The callsign for your new agent.")
                            .id(ALL_COMMANDS.arg_callsign.1)
                            .short(ALL_COMMANDS.arg_callsign.2)
                            .long(ALL_COMMANDS.arg_callsign.0)
                            .required(true)
                    )
                    .arg_required_else_help(true),
            )
            // manually set local game status
            .subcommand(
                Command::new(ALL_COMMANDS.sc_login)
                    .about("Login to an existing agent. Will overwrite existing local game status.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_callsign.0)
                            .help("The callsign of your existing agent.")
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
                    .about("View a waypoint location. Defaults to current agent headquarter.")
                    .arg(
                        Arg::new(ALL_COMMANDS.arg_waypoint.0)
                            .help("The waypoint to check. E.g., X1-DF55-20250Z")
                            .id(ALL_COMMANDS.arg_waypoint.1)
                            .short(ALL_COMMANDS.arg_waypoint.2)
                            .long(ALL_COMMANDS.arg_waypoint.0)
                            .action(ArgAction::Set)
                    )
            )
}

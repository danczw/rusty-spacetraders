use clap::{Arg, ArgAction, Command};

// TODO: move command strings to one place, e.g., enum
pub fn cli() -> Command {
    Command::new("rst")
            .about("A rust based SpaceTraders CLI.")
            .arg_required_else_help(true)
            .subcommand_required(true)
            // subcommand for local status
            .subcommand(
                Command::new("status")
                    .about("Get the status of the game, add -l for local status and -r for remote status.")
                    .arg(
                        Arg::new("local")
                            .help("Get the local status of the game: saved callsign and token.")
                            .id("id_local")
                            .short('l')
                            .long("local")
                            .exclusive(true)
                            .action(ArgAction::SetTrue)
                    )
                    .arg(
                        Arg::new("remote")
                            .help("Get the online status of the game.")
                            .id("id_remote")
                            .short('r')
                            .long("remote")
                            .exclusive(true)
                            .action(ArgAction::SetTrue)
                    )
            )
            // subcommand for new game
            .subcommand(
                Command::new("new")
                    .about("Register a new agent with Space Traders. Will overwrite existing local game status.")
                    .arg(
                        Arg::new("callsign")
                            .help("The callsign of your existing agent.")
                            .id("id_callsign")
                            .short('c')
                            .long("callsign")
                            .required(true)
                    )
                    .arg_required_else_help(true),
            )
            // manually set local game status
            .subcommand(
                Command::new("login")
                    .about("Login to an existing agent. Will overwrite existing local game status.")
                    .arg(
                        Arg::new("callsign")
                            .help("The callsign of your existing agent.")
                            .id("id_callsign")
                            .short('c')
                            .long("callsign")
                            .required(true)
                    )
                    .arg_required_else_help(true)
            )
            // check waypoint
            .subcommand(
                Command::new("location")
                    .about("View a waypoint location. Defaults to current agent headquarter.")
                    .arg(
                        Arg::new("waypoint")
                            .help("The waypoint to check. E.g., X1-DF55-20250Z")
                            .id("id_waypoint")
                            .short('w')
                            .long("waypoint")
                            .action(ArgAction::Set)
                    )
            )
}

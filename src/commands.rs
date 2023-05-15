pub mod cmnds {
    use clap::{Arg, ArgAction, Command};

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
                            .help("Get the local status of the game.")
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
                    .arg(
                        Arg::new("token")
                            .help("The token of your existing agent.")
                            .id("id_token")
                            .short('t')
                            .long("token")
                            .required(true)
                    )
                    .arg_required_else_help(true)
            )
    }
}

<div align="center"><h1>
<b>Rusty SpaceTraders CLI</b>
</h1></div>

<br>

A command line interface for the [SpaceTraders](https://spacetraders.io/) game, written in rust.

**Currently in Alpha Version !**

<br>

---

## Setup

Dependencies are managed with [Cargo](https://doc.rust-lang.org/cargo/).
- [Clap](https://docs.rs/clap/latest/clap/index.html) crate is used for command line parsing
- Making requests to the SpaceTraders API using [reqwest](https://docs.rs/reqwest/latest/reqwest/#) crate
- serde_json for JSON serialization.

Start a new game and get your callsign and token from the [SpaceTraders](https://spacetraders.io/) website using the `$ rst new` command.
Agent callsign and token are saved in user root directory in a file named `.spacetraders`. For the use of existing agents, create this file manually adding the content as shown below or or use the `$ rst login` command to create it.

```
callsign=<YOUR CALLSING>
token=<YOUR TOKEN>
```

---

## Command Docs

```
USAGE:
    rst [FLAGS] [OPTIONS] <SUBCOMMAND>
```
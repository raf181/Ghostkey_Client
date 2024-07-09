//cli.rc 
use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("command_sender")
        .version("0.1.0")
        .author("raf181_")
        .about("Sends commands to ESP32 devices")
        .subcommand(
            Command::new("register_user")
                .about("Registers a new user")
                .arg(Arg::new("secret_key").required(true).help("Secret key"))
                .arg(Arg::new("username").required(true).help("Username"))
                .arg(Arg::new("password").required(true).help("Password")),
        )
        .subcommand(
            Command::new("login")
                .about("Logs in a user")
                .arg(Arg::new("username").required(true).help("Username"))
                .arg(Arg::new("password").required(true).help("Password")),
        )
        .subcommand(
            Command::new("send_command")
                .about("Sends a command to a device")
                .arg(Arg::new("esp_id").required(true).help("ESP ID"))
                .arg(Arg::new("command").required(true).help("Command text")),
        )
        .subcommand(
            Command::new("active_boards")
                .about("Gets a list of active boards"),
        )
        .subcommand(
            Command::new("get_all_commands")
                .about("Gets all commands for a device")
                .arg(Arg::new("esp_id").required(true).help("ESP ID")),
        )
        .subcommand(
            Command::new("register_device")
                .about("Registers a new ESP32 device")
                .arg(Arg::new("esp_id").required(true).help("ESP ID"))
                .arg(Arg::new("secret_key").required(true).help("Secret key")),
        )
}

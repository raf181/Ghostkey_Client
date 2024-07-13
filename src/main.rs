use clap::ArgMatches;
use reqwest::Client;
use std::error::Error;
use tokio::runtime::Runtime;

mod cli;
mod request;
use cli::build_cli;
use request::*;

const SERVER_ADDRESS: &str = "http://192.168.10.62:5000";

fn main() -> Result<(), Box<dyn Error>> {
    let matches = build_cli().get_matches();
    let client = Client::new();
    let rt = Runtime::new()?;

    match matches.subcommand() {
        Some(("register_user", sub_m)) => {
            println!("Registering user...");
            rt.block_on(register_user_cmd(sub_m, &client))?
        }
        Some(("login", sub_m)) => {
            println!("Logging in...");
            rt.block_on(login_cmd(sub_m, &client))?
        }
        Some(("send_command", sub_m)) => {
            println!("Sending command...");
            rt.block_on(send_command_cmd(sub_m, &client))?
        }
        Some(("active_boards", _)) => {
            println!("Getting active boards...");
            rt.block_on(active_boards_cmd(&client))?
        }
        Some(("get_all_commands", sub_m)) => {
            println!("Getting all commands...");
            rt.block_on(get_all_commands_cmd(sub_m, &client))?
        }
        Some(("get_loaded_commands", sub_m)) => {
            println!("Getting loaded commands...");
            rt.block_on(get_loaded_commands_cmd(sub_m, &client))?
        }
        // [Test, Not ready for release]
        Some(("register_device", sub_m)) => {
            println!("Registering device...");
            rt.block_on(register_device_cmd(sub_m, &client))?
        }
        // [Test, Not ready for release] Not implemented in the server
        Some(("remove_device", sub_m)) => {
            println!("Deleting device...");
            rt.block_on(remove_device_cmd(sub_m, &client))?
        }
        // [Test, Not ready for release] Not implemented in the server
        Some(("export_database", _)) => {
            println!("Exporting database...");
            // rt.block_on(export_database_cmd(&client))?
        }
        Some((name, _)) => {
            println!("Unmatched subcommand: {}", name);
        }
        None => {
            println!("░▒▓██████▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░ ░▒▓███████▓▒░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░ ");
            println!("░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ "); 
            println!("░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ ");
            println!("░▒▓█▓▒▒▓███▓▒░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░   ░▒▓█▓▒░   ░▒▓███████▓▒░░▒▓██████▓▒░  ░▒▓██████▓▒░  ");
            println!("░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░     ");
            println!("░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░     ");
            println!(" ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓███████▓▒░   ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░  ░▒▓█▓▒░     ");
            println!("==========================================================================================================");
            println!("\x1b[93m Using version: 0.1.0 \x1b[0m");
            println!(" ");
            println!("Please use a subcommand. Run with --help for more information.");
        }
    }

    Ok(())
}

async fn register_user_cmd(matches: &ArgMatches, client: &Client) -> Result<(), Box<dyn Error>> {
    let secret_key = matches.get_one::<String>("secret_key").unwrap();
    let username = matches.get_one::<String>("username").unwrap();
    let password = matches.get_one::<String>("password").unwrap();
    register_user(client, SERVER_ADDRESS, secret_key, username, password).await
}

async fn login_cmd(matches: &ArgMatches, client: &Client) -> Result<(), Box<dyn Error>> {
    let username = matches.get_one::<String>("username").unwrap();
    let password = matches.get_one::<String>("password").unwrap();
    login(client, SERVER_ADDRESS, username, password).await
}

async fn send_command_cmd(matches: &ArgMatches, client: &Client) -> Result<(), Box<dyn Error>> {
    let esp_id = matches.get_one::<String>("esp_id").unwrap();
    let command = matches.get_one::<String>("command").unwrap();
    send_command(client, SERVER_ADDRESS, esp_id, command).await
}

async fn active_boards_cmd(client: &Client) -> Result<(), Box<dyn Error>> {
    active_boards(client, SERVER_ADDRESS).await
}

async fn get_loaded_commands_cmd(matches: &ArgMatches, client: &Client) -> Result<(), Box<dyn Error>> {
    let esp_id = matches.get_one::<String>("esp_id").unwrap();
    get_loaded_commands(client, SERVER_ADDRESS, esp_id).await
}

async fn get_all_commands_cmd(matches: &ArgMatches, client: &Client) -> Result<(), Box<dyn Error>> {
    let esp_id = matches.get_one::<String>("esp_id").unwrap();
    get_all_commands(client, SERVER_ADDRESS, esp_id).await
}

// [Test, Not ready for release]
async fn register_device_cmd(matches: &ArgMatches, client: &Client) -> Result<(), Box<dyn Error>> {
    let esp_id = matches.get_one::<String>("esp_id").expect("ESP ID is required");
    let secret_key = matches.get_one::<String>("secret_key").expect("Secret key is required");
    register_device(client, SERVER_ADDRESS, esp_id, secret_key).await
}

// [Test, Not ready for release]
async fn remove_device_cmd(matches: &ArgMatches, client: &Client) -> Result<(), Box<dyn Error>> {
    let esp_id = matches.get_one::<String>("esp_id").unwrap();
    let secret_key = matches.get_one::<String>("secret_key").expect("Secret key is required");
    remove_device(client, SERVER_ADDRESS, esp_id, secret_key).await
}

// [Test, Not ready for release]
// async fn export_database_cmd(client: &Client) -> Result<(), Box<dyn Error>> {
//     export_database(client, SERVER_ADDRESS).await
// }
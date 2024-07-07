use clap::ArgMatches;
use reqwest::blocking::Client;
use std::error::Error;

mod cli;
use cli::build_cli;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("register_user", sub_m)) => {
            println!("Registering user...");
            register_user(sub_m)?
        }
        Some(("login", sub_m)) => {
            println!("Logging in...");
            login(sub_m)?
        }
        Some(("send_command", sub_m)) => {
            println!("Sending command...");
            send_command(sub_m)?
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

fn register_user(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let secret_key = matches.get_one::<String>("secret_key").unwrap();
    let username = matches.get_one::<String>("username").unwrap();
    let password = matches.get_one::<String>("password").unwrap();

    let client = Client::new();
    let res = client.post("http://192.168.10.62:5000/register_user")
        .form(&[("secret_key", secret_key), ("username", username), ("password", password)])
        .send()?;

    println!("{:#?}", res.text()?);
    Ok(())
}

fn login(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let username = matches.get_one::<String>("username").unwrap();
    let password = matches.get_one::<String>("password").unwrap();

    let client = Client::new();
    let res = client.post("http://192.168.10.62:5000/login")
        .form(&[("username", username), ("password", password)])
        .send()?;

    println!("{:#?}", res.text()?);
    Ok(())
}

fn send_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let esp_id = matches.get_one::<String>("esp_id").unwrap();
    let command = matches.get_one::<String>("command").unwrap();

    let client = Client::new();
    let res = client.post("http://192.168.10.62:5000/command")
        .form(&[("esp_id", esp_id), ("command", command)])
        .send()?;

    println!("{:#?}", res.text()?);
    Ok(())
}

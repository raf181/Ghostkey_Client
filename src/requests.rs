// request.rs - Module for handling requests to the server
use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use serde_json::Value;

#[derive(Serialize)]
struct RegisterUser {
    secret_key: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct SendCommand {
    esp_id: String,
    command: String,
}

#[derive(Deserialize, Debug)]
struct ActiveBoard {
    esp_id: String,
    last_request_duration: String,
}

#[derive(Deserialize, Debug)]
struct ActiveBoardsResponse {
    active_boards: Vec<ActiveBoard>,
}

pub async fn register_user(client: &Client, base_url: &str, secret_key: &str, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/register_user", base_url);
    let user = RegisterUser {
        secret_key: secret_key.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    };

    let resp = client.post(&url).json(&user).send().await?;
    println!("Response: {:?}", resp.text().await?);
    Ok(())
}

pub async fn login(client: &Client, base_url: &str, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/login", base_url);
    let login_info = Login {
        username: username.to_string(),
        password: password.to_string(),
    };

    let resp = client.post(&url).json(&login_info).send().await?;
    println!("Response: {:?}", resp.text().await?);
    Ok(())
}

pub async fn send_command(client: &Client, base_url: &str, esp_id: &str, command: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/command", base_url);
    let cmd = SendCommand {
        esp_id: esp_id.to_string(),
        command: command.to_string(),
    };

    let resp = client.post(&url).json(&cmd).send().await?;
    println!("Response: {:?}", resp.text().await?);
    Ok(())
}

pub async fn active_boards(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/active_boards", base_url);

    let resp = client.get(&url).send().await?;
    let text = resp.text().await?;
    let active_boards_response: ActiveBoardsResponse = serde_json::from_str(&text)?;

    for board in active_boards_response.active_boards {
        println!("ESP ID: {}", board.esp_id);
        println!("Last Request Duration: {}", board.last_request_duration);
        println!("------------------------");
    }

    Ok(())
}

pub async fn get_all_commands(client: &Client, base_url: &str, esp_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/get_all_commands", base_url);
    let params = [("esp_id", esp_id)];
    
    let resp = client.get(&url).query(&params).send().await?;
    let text = resp.text().await?;
    let json: Value = serde_json::from_str(&text)?;
    println!("Response: {}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

// [Test, Not redy for release]
pub async fn register_device(client: &Client, base_url: &str, esp_id: &str, secret_key: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}//register_device", base_url);
    let params = RegisterEsp {
        esp_id: esp_id.to_string(),
        secret_key: secret_key.to_string(),
    };

    let resp = client.post(&url).json(&params).send().await?;
    println!("Response: {:?}", resp.text().await?);
    Ok(())
}
// [Test, Not redy for release] Not implemented in the server
pub async fn delete_device(client: &Client, base_url: &str, esp_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/delete_device", base_url);
    let params = [("esp_id", esp_id)];
    
    let resp = client.delete(&url).query(&params).send().await?;
    println!("Response: {:?}", resp.text().await?);
    Ok(())
}

// [Test, Not redy for release] Not implemented in the server
pub async fn export_database(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/export_database", base_url);

    let resp = client.get(&url).send().await?;
    let text = resp.text().await?;
    let json: Value = serde_json::from_str(&text)?;
    println!("Response: {}", serde_json::to_string_pretty(&json)?);
    Ok(())
}
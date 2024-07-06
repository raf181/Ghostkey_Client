use slint::prelude::*;
use reqwest::Client;
use tokio::runtime::Runtime;

slint::include_modules!();

fn main() {
    let ui = AppWindow::new();
    let rt = Runtime::new().unwrap();
    let client = Client::new();

    ui.on_clear(move || {
        println!("Clear button clicked");
        // Add code here to clear the form
    });

    ui.on_send_command(move || {
        println!("Send Command button clicked");
        // Add code here to send the command to the server
        rt.spawn(send_command(client.clone()));
    });

    ui.run().unwrap();
}

async fn send_command(client: Client) {
    let command_url = "http://localhost:8080/command";
    let esp_id = "your_esp_id";  // Replace with the actual ESP ID
    let command_text = "your_command";  // Replace with the actual command text

    let params = [("esp_id", esp_id), ("command", command_text)];
    match client.post(command_url)
        .form(&params)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Command sent successfully");
                } else {
                    println!("Failed to send command: {}", response.status());
                }
            }
            Err(e) => {
                println!("Error sending command: {}", e);
            }
        }
}

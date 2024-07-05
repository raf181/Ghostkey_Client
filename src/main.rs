use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Orientation};
use reqwest::blocking::Client;
use serde_json::Value;
use std::time::Duration;

const API_HOST: &str = "192.168.1.62";  // Replace with your API host address
const API_PORT: u16 = 5000;              // Replace with your API port
const ESP_ID: &str = "esp32_1";          // Replace with your ESP ID
const ESP_SECRET_KEY: &str = "your_esp_secret_key";  // Replace with your ESP Secret Key
const API_ENDPOINT: &str = "/get_command";  // Replace with your API endpoint for fetching commands

fn main() {
    // Initialize GTK application
    let application = Application::builder()
        .application_id("org.example.ghostkey_client")
        .build();

    application.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Ghostkey Client")
            .default_width(600)
            .default_height(400)
            .build();

        // Create a vertical box layout
        let vbox = gtk::Box::new(Orientation::Vertical, 5);
        window.set_child(Some(&vbox));

        // Add a label for status messages
        let status_label = Label::new(None);
        vbox.append(&status_label);

        // Add a button to fetch command
        let fetch_button = Button::builder()
            .label("Fetch Command")
            .build();
        vbox.append(&fetch_button);

        // Add an event handler for the fetch button
        let client = Client::new();
        let fetch_button_clone = fetch_button.clone();
        fetch_button.connect_clicked(move |_| {
            fetch_command(&client, &fetch_button_clone, &status_label);
        });

        // Show the window and all widgets
        window.show_all();
    });

    // Run the application
    application.run();
}

fn fetch_command(client: &Client, fetch_button: &Button, status_label: &Label) {
    fetch_button.set_label("Fetching...");
    let api_url = format!("http://{}:{}{}", API_HOST, API_PORT, API_ENDPOINT);
    let params = [
        ("esp_id", ESP_ID),
        ("esp_secret_key", ESP_SECRET_KEY),
    ];

    // Send GET request to API
    let response = client.get(&api_url).query(&params).send();
    match response {
        Ok(response) => {
            match response.json::<Value>() {
                Ok(json) => {
                    let command = json["command"].as_str().unwrap_or("No command received").to_string();
                    show_command_dialog(&command);
                    status_label.set_text(&format!("Command Received: {}", command));
                }
                Err(e) => {
                    println!("Error parsing JSON: {}", e);
                    status_label.set_text("Error parsing JSON response");
                }
            }
        }
        Err(e) => {
            println!("Request failed: {}", e);
            status_label.set_text("Failed to connect to server");
        }
    }

    fetch_button.set_label("Fetch Command");
}

fn show_command_dialog(command: &str) {
    let dialog = gtk::MessageDialog::new(None::<&gtk::Window>,
                                         gtk::DialogFlags::MODAL,
                                         gtk::MessageType::Info,
                                         gtk::ButtonsType::Ok,
                                         &format!("Command Received: {}", command));
    dialog.run();
    dialog.close();
}

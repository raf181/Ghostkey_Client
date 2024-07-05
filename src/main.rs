use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Orientation};
use reqwest::Client;
use std::cell::RefCell;
use std::rc::Rc;

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
            .default_width(400)
            .default_height(300)
            .build();

        // Create a vertical box layout
        let vbox = gtk::Box::new(Orientation::Vertical, 5);
        window.set_child(Some(&vbox));

        // Add a label
        let label = Label::builder()
            .label("Ghostkey Client")
            .build();
        vbox.append(&label);

        // Add a button to fetch command
        let fetch_button = Button::builder()
            .label("Fetch Command")
            .build();
        vbox.append(&fetch_button);

        // Add an event handler for the fetch button
        let client = Client::new();
        let fetch_button_clone = fetch_button.clone();
        fetch_button.connect_clicked(move |_| {
            fetch_command(&client, &fetch_button_clone);
        });

        // Show the window and all widgets
        window.show_all();
    });

    // Run the application
    application.run();
}

fn fetch_command(client: &Client, fetch_button: &Button) {
    fetch_button.set_label("Fetching...");
    let api_url = format!("http://{}:{}{}", API_HOST, API_PORT, API_ENDPOINT);
    let params = [
        ("esp_id", ESP_ID),
        ("esp_secret_key", ESP_SECRET_KEY),
    ];

    // Send GET request to API
    let request = client.get(api_url).query(&params);
    let future = async {
        match request.send().await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        let command = json["command"].as_str().unwrap_or("No command received").to_string();
                        show_command_dialog(&command);
                    }
                    Err(e) => {
                        println!("Error parsing JSON: {}", e);
                        show_error_dialog("Error parsing JSON response");
                    }
                }
            }
            Err(e) => {
                println!("Request failed: {}", e);
                show_error_dialog("Failed to connect to server");
            }
        }
    };

    futures::executor::block_on(future);
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

fn show_error_dialog(message: &str) {
    let dialog = gtk::MessageDialog::new(None::<&gtk::Window>,
                                         gtk::DialogFlags::MODAL,
                                         gtk::MessageType::Error,
                                         gtk::ButtonsType::Ok,
                                         message);
    dialog.run();
    dialog.close();
}

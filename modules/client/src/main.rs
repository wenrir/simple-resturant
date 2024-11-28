//! main.rs
/// Main entrypoint of the client
use anyhow::Result;
use inquire::Text;
use inquire::{validator::Validation, CustomType};
use inquire::{InquireError, Select};
use log::{error, info};
use reqwest::Client;
use serde_json::json;
use std::{env::var, process::exit};

const HOST_URL: &str = "localhost";
const HOST_PORT: &str = "8080";

/// Prompt user for an integer.
macro_rules! iprompt {
    ($type:ty, $prompt:expr, $help:expr, $default:expr) => {
        CustomType::<$type>::new($prompt)
            .with_starting_input($default)
            .with_help_message($help)
            .with_validator(|val: &$type| {
                if *val <= 0 {
                    Ok(Validation::Invalid("Please enter a valid number".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .expect("Unable to read input")
    };
}

/// HTTP Get macro.
macro_rules! get {
    ($client:expr, $url:expr) => {{
        let response = $client.get($url).send().await;
        match response {
            Ok(res) => println!("Response: {:?}", res.text().await),
            Err(err) => eprintln!("Error during GET: {:?}", err),
        }
    }};
}

/// HTTP Post macro.
macro_rules! post {
    ($client:expr, $url:expr, $json_body:expr) => {{
        let response = $client.post($url).json(&$json_body).send().await;
        match response {
            Ok(res) => println!("Response: {:?}", res.text().await),
            Err(err) => eprintln!("Error during POST: {:?}", err),
        }
    }};
}

/// HTTP Delete macro.
macro_rules! delete {
    ($client:expr, $url:expr) => {{
        let response = $client.delete($url).send().await;
        match response {
            Ok(res) => println!("Response: {:?}", res.text().await),
            Err(err) => eprintln!("Error during DELETE: {:?}", err),
        }
    }};
}

/// Main entrypoint of client.
#[tokio::main]
async fn main() {
    let client = Client::new();

    let host = var("HOST_URL").unwrap_or_else(|_| HOST_URL.to_string());
    let port = var("HOST_PORT").unwrap_or_else(|_| HOST_PORT.to_string());
    let base_url = format!("http://{}:{}/api/v1", host, port);

    loop {
        let options: Vec<&str> = vec![
            "Add item(s)",
            "List items",
            "Get item by id",
            "Create order",
            "Get all orders",
            "Delete order",
            "Get all customers",
            "Get customer",
            "Check in new customer",
            "Exit",
        ];

        let ans: Result<&str, InquireError> = Select::new(
            "What would you like to do? (please select an option)",
            options,
        )
        .prompt();

        match ans {
            Ok(choice) => match choice {
                "Add item(s)" => {
                    let description = Text::new("Enter item description:")
                        .prompt()
                        .expect("Unable to read description!");
                    let minutes: i32 =
                        iprompt!(i32, "Enter estimated minutes:", "Minutes for the item", "0");

                    let url = format!("{}/items", base_url);
                    post!(
                        client,
                        &url,
                        json!({
                            "description": description.to_string(),
                            "estimated_minutes": minutes
                        })
                    );
                }
                "List items" => {
                    let url = format!("{}/items", base_url);
                    get!(client, &url);
                }
                "Get item by id" => {
                    let id: i32 = iprompt!(i32, "Enter item id:", "Item ID to fetch", "0");
                    let url = format!("{}/items/{}", base_url, id);
                    get!(client, &url);
                }
                "Create order" => {
                    let item: i32 = iprompt!(
                        i32,
                        "Enter item id:",
                        "Item ID to include in the order",
                        "1"
                    );
                    let customer: i32 =
                        iprompt!(i32, "Enter customer id:", "Customer ID for the order", "1");
                    let quantity: i32 = iprompt!(i32, "Enter quantity:", "Quantity of items", "1");

                    let url = format!("{}/orders", base_url);
                    post!(
                        client,
                        &url,
                        json!({
                            "item_id": item,
                            "customer_id": customer,
                            "quantity": quantity
                        })
                    );
                }
                "Get all orders" => {
                    let url = format!("{}/orders", base_url);
                    get!(client, &url);
                }
                "Delete order" => {
                    let order_id: i32 = iprompt!(i32, "Enter order id:", "Order ID to delete", "1");
                    let url = format!("{}/orders/{}", base_url, order_id);
                    delete!(client, &url);
                }
                "Get all customers" => {
                    let url = format!("{}/customers", base_url);
                    get!(client, &url);
                }
                "Get customer" => {
                    let customer_id: i32 =
                        iprompt!(i32, "Enter customer id:", "Customer ID to fetch", "1");
                    let url = format!("{}/customers/{}", base_url, customer_id);
                    get!(client, &url);
                }
                "Check in new customer" => {
                    let url = format!("{}/customers/check_in", base_url);

                    let table: i32 = iprompt!(
                        i32,
                        "Enter table id:",
                        "Table ID to place an order for",
                        "0"
                    );

                    post!(
                        client,
                        &url,
                        json!({

                                "table_number": table,
                        })
                    );
                }
                "Exit" => {
                    info!("Exiting... Goodbye!");
                    exit(0);
                }
                _ => {
                    error!("Unknown option selected.");
                }
            },
            Err(_) => error!("There was an error, please try again"),
        }
    }
}

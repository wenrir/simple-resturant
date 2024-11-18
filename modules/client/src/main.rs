//! main.rs
use anyhow::Result;
/// Main entrypoint of the client
use inquire::{InquireError, Select};
use log::{error, info};
use reqwest::Client;
use serde_json::json;
use std::env::var;

const HOST_URL: &str = "127.0.0.1";
const HOST_PORT: &str = "8080";

#[tokio::main]
async fn main() {
    // Not very userfriendly ;)
    let client = Client::new();

    let host = var("HOST_URL").unwrap_or_else(|_| HOST_URL.to_string());
    let port = var("HOST_PORT").unwrap_or_else(|_| HOST_PORT.to_string());
    let base_url = format!("http://{}:{}/api/v1", host, port);

    loop {
        let options: Vec<&str> = vec![
            "Add item(s)",
            "Create order",
            "Delete item(s)",
            "Get tables",
            "Check in customer",
            "Exit",
        ];
        let ans: Result<&str, InquireError> =
            Select::new("What would you like to do?", options).prompt();
        match ans {
            Ok(choice) => match choice {
                "Add item(s)" => {
                    let item_result = || async {
                        use std::io::{self, Write};

                        print!("Enter item description: ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut description = String::new();
                        io::stdin()
                            .read_line(&mut description)
                            .expect("Unable to read stdin");

                        print!("Enter estimated minutes: ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut minutes = String::new();
                        io::stdin()
                            .read_line(&mut minutes)
                            .expect("unable to read line");
                        let minutes: i32 = minutes.trim().parse().unwrap_or(0);

                        let item = create_item(&client, &base_url, description.trim(), minutes)
                            .await
                            .expect("Unable to create item");
                        info!("Item created: {:?}", item);
                    };
                    item_result().await;
                }
                "Create order" => {
                    let order_result = || async {
                        use std::io::{self, Write};

                        print!("Enter table number: ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut table = String::new();
                        io::stdin()
                            .read_line(&mut table)
                            .expect("Unable to read stdin");

                        print!("Enter item: ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut item = String::new();
                        io::stdin()
                            .read_line(&mut item)
                            .expect("Unable to read stdin");

                        print!("Enter customer (id): ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut c = String::new();
                        io::stdin().read_line(&mut c).expect("Unable to read stdin");

                        print!("Enter quantity: ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut q = String::new();
                        io::stdin().read_line(&mut q).expect("Unable to read stdin");

                        let order = create_order(
                            &client,
                            &base_url,
                            table.parse::<i32>().expect("Unable to unwrap table number"),
                            item.parse::<i32>().expect("Unable to unwrap item id"),
                            c.parse::<i32>().expect("Unable to unwrap customer id"),
                            q.parse::<i32>().expect("Unable to unwrap quantity"),
                        );
                        info!("Order created: {:?}", order.await);
                    };
                    order_result().await;
                }
                "Delete order" => {
                    let delete_item = || async {
                        use std::io::{self, Write};
                        print!("Enter order id (not reversible): ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut q = String::new();
                        io::stdin().read_line(&mut q).expect("Unable to read stdin");
                        let url = format!(
                            "{}/orders/{}",
                            base_url,
                            q.parse::<i32>().expect("Unable to unwrap quantity"),
                        );
                        let response = client.delete(&url).send().await;
                        info!("Order created: {:?}", response);
                    };

                    delete_item().await;
                }
                "Get tables" => {
                    let get_tables = || async {
                        use std::io::{self, Write};
                        print!("Enter table : ");
                        io::stdout().flush().expect("Unable to flush stdout");
                        let mut q = String::new();
                        io::stdin().read_line(&mut q).expect("Unable to read stdin");
                        let url = format!("{}/tables", base_url);
                        let response = client
                            .post(&url)
                            .json(&json!({
                                "id": q.parse::<i32>().expect("Unable to unwrap quantity"),
                            }))
                            .send()
                            .await;
                        info!("Order created: {:?}", response);
                    };

                    get_tables().await;
                }
                "Check in customer" => {
                    let _ = handle_check_in_customer(&client, &base_url).await;
                }
                "Exit" => {
                    info!("Exiting... Goodbye!");
                    break;
                }
                _ => {
                    error!("Unknown option selected.");
                }
            },
            Err(_) => error!("There was an error, please try again"),
        }
    }
}

async fn create_item(
    client: &Client,
    base_url: &str,
    description: &str,
    estimated_minutes: i32,
) -> Result<String> {
    let url = format!("{}/items", base_url);
    let response = client
        .post(&url)
        .json(&json!({
            "description": description.to_string(),
            "estimated_minutes": estimated_minutes
        }))
        .send()
        .await?;
    Ok(response.json::<String>().await?)
}

async fn create_order(
    client: &Client,
    base_url: &str,
    table_number: i32,
    item_id: i32,
    customer_id: i32,
    quantity: i32,
) -> Result<String> {
    let url = format!("{}/orders", base_url);
    let response = client
        .post(&url)
        .json(&json!({
        "table_number": table_number,
        "item_id": item_id,
        "customer_id": customer_id,
        "quantity": quantity,
            }))
        .send()
        .await?;
    Ok(response.json::<String>().await?)
}

async fn handle_check_in_customer(client: &Client, base_url: &str) -> Result<()> {
    let url = format!("{}/tables/check_in", base_url);
    let _ = client.post(&url).send().await?;
    Ok(())
}

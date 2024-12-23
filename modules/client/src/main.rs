//! main.rs
/// Main entrypoint of the client
use anyhow::Result;
use inquire::Text;
use inquire::{validator::Validation, CustomType};
use inquire::{InquireError, Select};
use log::{error, info};
use rand::Rng;
use reqwest::Client;
use serde_json::json;
use std::{env::var, process::exit};
use tokio_util::task::TaskTracker;

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
    ($client:expr, $url:expr, $msg:expr) => {{
        let response = $client.get($url).send().await;
        match response {
            Ok(res) => {
                let text = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to read response".to_string());
                println!("{:?}: {:?}", $msg, text)
            }
            Err(err) => eprintln!("Error during GET: {:?}", err),
        }
    }};
}

/// HTTP Post macro.
macro_rules! post {
    ($client:expr, $url:expr, $json_body:expr, $msg:expr) => {{
        let response = $client.post($url).json(&$json_body).send().await;
        match response {
            Ok(res) => {
                let text = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to read response".to_string());
                println!("{:?}: {:?}", $msg, text)
            }
            Err(err) => eprintln!("Error during POST: {:?}", err),
        }
    }};
}

/// HTTP Delete macro.
macro_rules! delete {
    ($client:expr, $url:expr, $msg:expr) => {{
        let response = $client.delete($url).send().await;
        match response {
            Ok(res) => {
                let text = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to read response".to_string());
                println!("{:?}: {:?}", $msg, text)
            }
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
            "Item operations",
            "Table operations",
            "Order operations",
            "Simulation",
            "Exit",
        ];

        let ans: Result<&str, InquireError> = Select::new(
            "What would you like to do? (please select an option, sub menu will be available)",
            options,
        )
        .prompt();

        match ans {
            Ok(choice) => match choice {
                "Item operations" => {
                    let item_options: Vec<&str> =
                        vec!["Add item(s)", "List items", "Get item by id", "Back"];
                    let a: Result<&str, InquireError> = Select::new(
                        "[item] What would you like to do? (please select an option)",
                        item_options,
                    )
                    .prompt();
                    match a {
                        Ok(c) => match c {
                            "Back" => {}
                            "Add item" => {
                                let description = Text::new("Enter item description:")
                                    .prompt()
                                    .expect("Unable to read description!");

                                let price: i32 =
                                    iprompt!(i32, "Enter item price:", "Item price", "1");

                                let url = format!("{}/items", base_url);
                                post!(
                                    client,
                                    &url,
                                    json!({
                                        "description": description.to_string(),
                                        "price": price,
                                    }),
                                    "Added item"
                                );
                            }
                            "List items" => {
                                let url = format!("{}/items", base_url);
                                get!(client, &url, "Items");
                            }
                            "Get item by id" => {
                                let id: i32 =
                                    iprompt!(i32, "Enter item id:", "Item ID to fetch", "0");
                                let url = format!("{}/items/{}", base_url, id);
                                get!(client, &url, "Items");
                            }
                            _ => {}
                        },
                        Err(_) => error!("There was an error, please try again"),
                    }
                }
                "Table operations" => {
                    let table_options: Vec<&str> = vec![
                        "Get all tables",
                        "Get table information (including orders)",
                        "Get order information for table",
                        "Check in new table",
                        "Check out table",
                        "Delete table order",
                        "Back",
                    ];
                    let a: Result<&str, InquireError> = Select::new(
                        "[table] What would you like to do? (please select an option)",
                        table_options,
                    )
                    .prompt();
                    match a {
                        Ok(c) => match c {
                            "Back" => {}
                            "Get all tables" => {
                                let url = format!("{}/tables", base_url);
                                get!(client, &url, "All tables");
                            }
                            "Get table information (including orders)" => {
                                let table_id: i32 = iprompt!(
                                    i32,
                                    "Enter table number:",
                                    "Table Number to fetch",
                                    "1"
                                );
                                {
                                    let url = format!("{}/tables/{}", base_url, table_id);
                                    get!(client, &url, "Table information");
                                }
                                println!("Reading orders for table {:?}", table_id);
                                {
                                    let url = format!("{}/tables/{}/orders", base_url, table_id);
                                    get!(client, &url, "Table orders");
                                }
                            }
                            "Get order information for table" => {
                                let table_id: i32 = iprompt!(
                                    i32,
                                    "Enter table number:",
                                    "Table Number to fetch",
                                    "1"
                                );
                                let item_id: i32 =
                                    iprompt!(i32, "Enter order id:", "Item ID to fetch", "1");
                                let url =
                                    format!("{}/tables/{}/items/{}", base_url, table_id, item_id);
                                get!(client, &url, "Table order information");
                            }
                            "Check in new table" => {
                                let url = format!("{}/tables/check_in", base_url);

                                let table: i32 = iprompt!(
                                    i32,
                                    "Enter table number:",
                                    "Table Number to place an order for",
                                    "0"
                                );

                                post!(
                                    client,
                                    &url,
                                    json!({"table_number": table,}),
                                    "Checked in table"
                                );
                            }
                            "Check out table" => {
                                let table: i32 = iprompt!(
                                    i32,
                                    "Enter table number:",
                                    "Table number to place an order for",
                                    "0"
                                );
                                let url = format!("{}/tables/{}/check_out", base_url, table);

                                post!(client, &url, json!({}), "Checked out table");
                            }
                            "Delete table order" => {
                                let table_id: i32 = iprompt!(
                                    i32,
                                    "Enter table number:",
                                    "Table Number to delete order for",
                                    "1"
                                );
                                let order_id: i32 =
                                    iprompt!(i32, "Enter order id:", "Order ID to delete", "1");
                                let url =
                                    format!("{}/tables/{}/orders/{}", base_url, table_id, order_id);
                                delete!(client, &url, "Deleted order");
                            }
                            _ => {}
                        },
                        Err(_) => error!("There was an error, please try again"),
                    }
                }
                "Order operations" => {
                    let order_options: Vec<&str> = vec![
                        "Create order",
                        "Get all orders",
                        "Get order by id",
                        "Delete order",
                        "Back",
                    ];
                    let a: Result<&str, InquireError> = Select::new(
                        "[order] What would you like to do? (please select an option)",
                        order_options,
                    )
                    .prompt();
                    match a {
                        Ok(c) => match c {
                            "Back" => {}
                            "Create order" => {
                                let table: i32 = iprompt!(
                                    i32,
                                    "Enter table number:",
                                    "Table number for the order",
                                    "1"
                                );
                                let mut items = vec![];
                                let url = format!("{}/orders", base_url);
                                // The nesting is getting out of control ...
                                loop {
                                    let item: i32 = iprompt!(
                                        i32,
                                        "Enter item id:",
                                        "Item ID to include in the order",
                                        "1"
                                    );
                                    let quantity: i32 =
                                        iprompt!(i32, "Enter quantity:", "Quantity of items", "1");
                                    // TODO: Change structure of the order creation to accept a list of items instead.
                                    // E.g.
                                    // {
                                    //  "table_id": 1,
                                    //  "items": [
                                    //    { "item_id": 101, "quantity": 2 },
                                    //    { "item_id": 102, "quantity": 5 }
                                    //  ]
                                    //}

                                    items.push(json!({
                                        "table_id": table,
                                        "item_id": item,
                                        "quantity": quantity
                                    }));

                                    let add_more = Text::new("Add another item? (yes/no)")
                                        .with_default("no")
                                        .prompt()
                                        .expect("Failed to read input");
                                    if add_more.to_lowercase() != "yes" {
                                        break;
                                    }
                                }
                                post!(client, &url, json!(items), "Created orders");
                            }
                            "Get all orders" => {
                                let url = format!("{}/orders", base_url);
                                get!(client, &url, "All orders");
                            }
                            "Get order by id" => {
                                let order_id: i32 =
                                    iprompt!(i32, "Enter order id:", "Order ID to find", "1");
                                let url = format!("{}/orders/{}", base_url, order_id);
                                get!(client, &url, "Order");
                            }
                            "Delete order" => {
                                let order_id: i32 =
                                    iprompt!(i32, "Enter order id:", "Order ID to delete", "1");
                                let url = format!("{}/orders/{}", base_url, order_id);
                                delete!(client, &url, "Order deleted");
                            }
                            _ => {}
                        },
                        Err(_) => error!("There was an error, please try again"),
                    }
                }
                "Simulation" => {
                    let nclients: usize = iprompt!(
                        usize,
                        "How many clients to simulate? ",
                        "Number of clients to simulate",
                        "1"
                    );
                    let tracker = TaskTracker::new();

                    let items = vec![
                        json!({ "description": "Yakitori", "price": 3 }),
                        json!({ "description": "Takoyaki", "price": 3 }),
                        json!({ "description": "Highball", "price": 2 }),
                    ];
                    let itemurl = format!("{}/items", base_url);
                    for item in items {
                        post!(client, &itemurl, item, "Added items to db");
                    }
                    for id in 1..=nclients {
                        // TODO don't clone ...
                        let cloned_client = client.clone();
                        let cloned_url = base_url.clone();
                        let mut rng = rand::thread_rng();
                        let num_orders = rng.gen_range(1..15);
                        let mut items = vec![];
                        for _ in 0..num_orders {
                            items.push(json!({
                                "table_id": id,
                                "item_id": rng.gen_range(1..3),
                                "quantity": rng.gen_range(1..5)
                            }));
                        }
                        let checkin_url = format!("{}/tables/check_in", cloned_url);
                        let checkout_url = format!("{}/tables/{}/check_out", base_url, id);
                        let order_url = format!("{}/orders", cloned_url);
                        tracker.spawn(async move {
                            post!(
                                cloned_client,
                                &checkin_url,
                                json!({"table_number": id,}),
                                format!("Checked in table {:?}", id)
                            );
                            for item in items {
                                post!(
                                    cloned_client,
                                    &order_url,
                                    json!([item]),
                                    format!("Added item {:?} for table {:?}", item, id)
                                );
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            }
                            post!(
                                cloned_client,
                                &checkout_url,
                                json!({}),
                                format!("Checked out table, table total")
                            );
                        });
                    }
                    tracker.close();

                    tracker.wait().await;
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

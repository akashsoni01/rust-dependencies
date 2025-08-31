use std::collections::HashMap;
use std::pin::Pin;
use chrono::{DateTime, Local};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HttpBin {
    origin: String,
}


struct Environment {
    date: Box<dyn Fn() -> DateTime<Local>>,
    http_bin: Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<HttpBin, reqwest::Error>>>>>,
}

impl Environment {
    fn new() -> Self {
        let client = Client::new();

        Self {
            date: Box::new(|| Local::now()),
            http_bin: Box::new(move || {
                let client = client.clone();
                Box::pin(async move {
                    let response = client
                        .get("https://httpbin.org/get")
                        .send()
                        .await?
                        .json::<serde_json::Value>()
                        .await?;

                    Ok(HttpBin {
                        origin: response["origin"]
                            .as_str()
                            .unwrap_or("unknown")
                            .to_string(),
                    })
                })
            }),
        }
    }

    // async fn get_http_bin_data(&self) -> Result<HttpBin, reqwest::Error> {
    //     let client = Client::new();
    //     let response = client
    //         .get("https://httpbin.org/get")
    //         .send()
    //         .await?
    //         .json::<serde_json::Value>()
    //         .await?;
    // 
    //     Ok(HttpBin {
    //         origin: response["origin"]
    //             .as_str()
    //             .unwrap_or("unknown")
    //             .to_string(),
    //     })
    // }

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

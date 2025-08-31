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
    http_bin: Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<HttpBin, reqwest::Error>>>>>,
    // .
    // .
    // .
    // .
    // other dependencies 
}

impl Environment {
    fn prod() -> Self {
        let client = Client::new();

        Self {
            date: Box::new(|| Local::now()),
            http_bin: Box::new(move |url_string: String| {
                let client = client.clone();
                Box::pin(async move {
                    let response = client
                        .get(url_string.as_str())
                        .send()
                        .await?
                        .json::<serde_json::Value>()
                        .await?;
                    let bin = HttpBin {
                        origin: response["origin"]
                            .as_str()
                            .unwrap_or("unknown")
                            .to_string(),
                    };
                    println!("prod = {:?}", bin);
                    Ok(bin)
                })
            }),
        }
    }

    fn mock() -> Self {
        let mock_date = DateTime::<Local>::from(std::time::UNIX_EPOCH + std::time::Duration::from_secs(557152051));
        
        Self {
            date: Box::new(move || mock_date),
            http_bin: Box::new(|url_string: String| {
                let mock_bin = HttpBin {
                    origin: "mock".to_string(),
                };

                Box::pin(async move {
                    println!("mock = {:?}", mock_bin);
                    Ok(mock_bin)
                })
            }),
        }
    }
}


#[tokio::main]
async fn main()  {
    let env = Environment::prod();
    (env.http_bin)("https://httpbin.org/get".to_string()).await.unwrap();
    println!(
        "Hello, world! at {}",
        (env.date)().format("%Y-%m-%d %H:%M:%S")
    );
    
    let mock = Environment::mock();
    (mock.http_bin)("https://httpbin.org/get".to_string()).await.unwrap();
    println!(
        "Hello, world! at {}",
        (mock.date)().format("%Y-%m-%d %H:%M:%S")
    );
}

/*
Output call 1 
prod = HttpBin { origin: "103.48.69.244" }
Hello, world! at 2025-08-31 11:26:47
mock = HttpBin { origin: "mock" }
Hello, world! at 1987-08-28 17:57:31


Output call 2
prod = HttpBin { origin: "103.48.69.244" }
Hello, world! at 2025-08-31 11:26:58
mock = HttpBin { origin: "mock" }
Hello, world! at 1987-08-28 17:57:31

*/
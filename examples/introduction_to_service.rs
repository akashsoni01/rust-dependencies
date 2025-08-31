use chrono::{DateTime, Local};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HttpBin {
    origin: String,
}

trait HttpRequestDependency {
    async fn get_request(&self, url_string: String) -> Result<HttpBin, reqwest::Error>;
}

trait DateDependency {
    async fn current(&self) -> DateTime<Local>;
}

struct HttpBinServiceDependency {
    client: reqwest::Client,
}
impl HttpBinServiceDependency {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl HttpRequestDependency for HttpBinServiceDependency {
    async fn get_request(&self, url_string: String) -> Result<HttpBin, Error> {
            let response = self.client
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
    }
}
struct HttpBinServiceDependencyMock {
    
}
impl HttpRequestDependency for HttpBinServiceDependencyMock {
    async fn get_request(&self, url_string: String) -> Result<HttpBin, Error> {
        let bin = HttpBin {
            origin: "mock".to_string(),
        };
        println!("mock = {:?}", bin);
        Ok(bin)
    }
}


struct DateServiceDependency {
}

impl DateDependency for DateServiceDependency {
    async fn current(&self) -> DateTime<Local> {
        Local::now()
    }
}

struct DateServiceDependencyMock {
    
}
impl DateDependency for DateServiceDependencyMock {
    async fn current(&self) -> DateTime<Local> {
        DateTime::<Local>::from(std::time::UNIX_EPOCH + std::time::Duration::from_secs(557152051))
    }
}
#[tokio::main]
async fn main() {
    let date_service = DateServiceDependency {};
    let http_bin_service = HttpBinServiceDependency {
        client: reqwest::Client::new(),   
    };
    let date = date_service.current().await;
    let http_bin = http_bin_service.get_request("https://httpbin.org/get".to_string()).await;
    println!("date = {:?}", date);
    println!("http_bin = {:?}", http_bin);

    
    let date_service = DateServiceDependencyMock {};
    let http_bin_service = HttpBinServiceDependencyMock {};
    let date = date_service.current().await;
    let http_bin = http_bin_service.get_request("https://httpbin.org/get".to_string()).await;
    println!("mock date = {:?}", date);
    println!("mock http_bin = {:?}", http_bin);

}

/*
call 1 
prod = HttpBin { origin: "103.48.69.244" }
date = 2025-08-31T15:32:54.326387+05:30
http_bin = Ok(HttpBin { origin: "103.48.69.244" })
mock = HttpBin { origin: "mock" }
mock date = 1987-08-28T17:57:31+05:30
mock http_bin = Ok(HttpBin { origin: "mock" })

call 2 
prod = HttpBin { origin: "103.48.69.244" }
date = 2025-08-31T15:33:32.847570+05:30
http_bin = Ok(HttpBin { origin: "103.48.69.244" })
mock = HttpBin { origin: "mock" }
mock date = 1987-08-28T17:57:31+05:30
mock http_bin = Ok(HttpBin { origin: "mock" })
*/
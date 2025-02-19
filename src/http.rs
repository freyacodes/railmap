use reqwest::{header, RequestBuilder, Response};
use std::time::Duration;

pub async fn handle_request(request: RequestBuilder) -> Response {
    let max_attempts = 3;
    for attempt in 1..=max_attempts {
        let response = match request.try_clone().unwrap().send().await {
            Ok(r) => r,
            Err(error) => {
                println!("Error during request after attempt {}/{}: {}", attempt, max_attempts, error);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue
            }
        };

        if response.status() == 429 {
            match response.headers().get(header::RETRY_AFTER) {
                None => {
                    panic!("Got 429, but no retry-after header");
                }
                Some(value) => {
                    let seconds = value
                        .to_str()
                        .unwrap()
                        .parse::<u64>()
                        .expect("Failed to parse retry-after header");
                    println!("Ratelimited, waiting {} seconds", seconds);
                    tokio::time::sleep(Duration::from_secs(seconds)).await;
                }
            }
        } else if response.status() != 200 {
            panic!("Unexpected status: {}", response.status());
        }

        return response
    }
   
    panic!("Failed after {} attempts", max_attempts) 
}
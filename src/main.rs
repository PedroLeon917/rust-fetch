use reqwest::header::HeaderMap;
use dotenv::dotenv;
// use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("JA_API_KEY")?;

    let mut headers = HeaderMap::new();

    headers.insert("x-api-key", api_key.parse().unwrap());

    let client = reqwest::Client::new();

    let resp = client 
        .get("https://farecache-lm.prod.jetsm.art/farecache-lm/timetable?departure=BUE&destination=PMC&currency=CLP")
        .headers(headers)
        .send()
        .await?;

    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    println!("{resp:#?}");
    Ok(())
}
use reqwest::header::HeaderMap;
use dotenv::dotenv;
use std::env;
use serde::Serialize;
use serde::Deserialize;
use serde_json::to_string_pretty as stringify;

#[derive(Debug, Serialize, Deserialize)]
struct Outbound {
    #[serde(rename = "departureDate")]
    departure_date: String,
    price: i32,
    currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    outbound: Vec<Outbound>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlackoutDates {
    departure: OneWay,
    #[serde(rename = "return")]
    arrival: OneWay,
}

#[derive(Debug, Serialize, Deserialize)]
struct OneWay {
    #[serde(rename = "minDate")]
    min_date: String,
    #[serde(rename = "maxDate")]
    max_date: String,
    #[serde(rename = "validDaysOfWeek")]
    valid_days_of_week: Vec<String>,
    #[serde(rename = "includedDates")]
    included_dates: Vec<String>,
    #[serde(rename = "includedDates")]
    excluded_dates: Vec<String>,
    #[serde(rename = "excludedDateRanges")]
    excluded_date_ranges: Vec<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("JA_API_KEY")?;

    let mut headers = HeaderMap::new();

    headers.insert("x-api-key", api_key.parse().unwrap());

    let client = reqwest::Client::new();

    let d_resp = client 
        .get("https://farecache-lm.prod.jetsm.art/farecache-lm/timetable?departure=BOG&destination=SCL&currency=CLP")
        .headers(headers.clone())
        .send()
        .await?;

    let d_resp_json = d_resp.json::<Root>().await?;

    // println!("Response JSON: {:#?}", resp_json);

    let mut route: Vec<String> = Vec::new();

    for outbound in &d_resp_json.outbound {
        route.push(outbound.departure_date.to_string());
    }

    let departure_route = OneWay {
        min_date: route[0].to_string(),
        max_date: route[route.len()-1].to_string(),
        valid_days_of_week: vec![],
        included_dates: route.clone(),
        excluded_dates: vec![],
        excluded_date_ranges: vec![],
    };

    let r_resp = client 
        .get("https://farecache-lm.prod.jetsm.art/farecache-lm/timetable?departure=SCL&destination=BOG&currency=CLP")
        .headers(headers.clone())
        .send()
        .await?;

    let r_resp_json = r_resp.json::<Root>().await?;

    route.clear();

    for outbound in &r_resp_json.outbound {
        route.push(outbound.departure_date.to_string());
    }

    let arrival_route = OneWay {
        min_date: route[0].to_string(),
        max_date: route[route.len()-1].to_string(),
        valid_days_of_week: vec![],
        included_dates: route.clone(),
        excluded_dates: vec![],
        excluded_date_ranges: vec![],
    };

    let blackout_dates = BlackoutDates {
        departure: departure_route,
        arrival: arrival_route,
    };

    // dbg!(&blackout_dates);

    let data_json = stringify(&blackout_dates).unwrap();

    println!("{}", data_json);

    Ok(())
}
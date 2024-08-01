use dotenv::dotenv;
use env_logger;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Deserialize)]
struct ExchangeRate {
    rate: f64,
}

#[derive(Serialize)]
struct GA4EventParams {
    currency: String,
    value: f64,
}

#[derive(Serialize)]
struct GA4Event {
    name: String,
    params: GA4EventParams,
}

#[derive(Serialize)]
struct GA4Payload {
    client_id: String,
    events: Vec<GA4Event>,
}

async fn fetch_exchange_rate(client: &Client) -> Result<f64, Box<dyn std::error::Error>> {
    info!("Fetching exchange rate from National Bank of Ukraine");
    let response = client
        .get("https://bank.gov.ua/NBUStatService/v1/statdirectory/exchange?valcode=USD&json")
        .send()
        .await?;

    if response.status().is_success() {
        let rates = response.json::<Vec<ExchangeRate>>().await?;
        info!("Successfully fetched exchange rate: {}", rates[0].rate);
        Ok(rates[0].rate)
    } else {
        let status = response.status();
        error!("Failed to fetch exchange rate: HTTP {}", status);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP error: {}", status),
        )))
    }
}

async fn send_event_to_ga(
    client: &Client,
    exchange_rate: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let ga_mp_measurement_id = env::var("GA_MP_TID")?;
    let ga_mp_api_secret = env::var("GA_MP_SECRET")?;
    let client_id = Uuid::new_v4().to_string();

    let payload = GA4Payload {
        client_id: client_id.to_string(),
        events: vec![GA4Event {
            name: "currency_exchange".to_string(),
            params: GA4EventParams {
                currency: "UAH/USD".to_string(),
                value: exchange_rate,
            },
        }],
    };

    let response = client
        .post(&format!(
            "https://www.google-analytics.com/mp/collect?measurement_id={}&api_secret={}",
            ga_mp_measurement_id, ga_mp_api_secret
        ))
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        info!("Successfully sent event to Google Analytics");
        Ok(())
    } else {
        let status = response.status();
        let response_text = response.text().await?;
        error!("Failed to send event to Google Analytics: HTTP {}", status);
        error!("Response: {}", response_text);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("HTTP error: {}", status),
        )))
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Starting UAH/USD exchange rate tracker");

    let client = Client::new();

    loop {
        info!("Starting new iteration of fetching and sending data");
        match fetch_exchange_rate(&client).await {
            Ok(rate) => {
                if let Err(e) = send_event_to_ga(&client, rate).await {
                    error!("Failed to send event to Google Analytics: {}", e);
                }
            }
            Err(e) => error!("Failed to fetch exchange rate: {}", e),
        }

        info!("Sleeping for 1 hour");
        sleep(Duration::from_secs(3600)).await; // Wait for 1 hour
    }
}

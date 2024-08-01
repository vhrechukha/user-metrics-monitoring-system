# User Metrics Monitoring System

This project is a user metrics monitoring system that tracks the UAH/USD exchange rate and sends this data to Google Analytics using the Google Analytics 4 (GA4) Measurement Protocol. The system fetches the exchange rate from the National Bank of Ukraine every hour and logs the data as events in Google Analytics.

<img src="docs/currency-exchange-ga-events.gif" width="220px">

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)
- Google Analytics 4 account with a Measurement Protocol API secret

## How to get a Measurement ID

Guide: [link](https://support.google.com/analytics/answer/12270356?hl=en).

## How to create a Measurement Protocol API secret

1. First, go to your Google Analytics **Admin** > **Data collection and modification** > **Data Streams** and open **your chosen stream**:
2. Scroll down to the **Events** tab and click on **Measurement Protocol API secrets**; then click **Create**:
3. You will have to create a nickname for your new API secret. Click **Create**
4. You will need to copy the **Secret value** provided, this is your `GA_MP_SECRET`

## Installation

1. **Set up environment variables:**

    Create a `.env` file in the root directory and add the following environment variables:

    ```env
    GA_MP_TID=your_ga4_measurement_id
    GA_MP_SECRET=your_ga4_api_secret
    ```

2. **Install dependencies:**

    ```sh
    cargo build
    ```

3. **Usage**

    To run the user metrics monitoring system, use the following command:

    ```sh
    cargo run

use rust_decimal::prelude::*;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PriceResponse {
    price: String,
}

pub fn get_bitcoin_price() -> Result<Decimal, Box<dyn std::error::Error>> {
    let resp =
        reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")?;
    let body = resp.json::<PriceResponse>()?;

    let price = match Decimal::from_str(&body.price) {
        Ok(num) => num,
        Err(_) => {
            println!("Error on converting");
            let a = Decimal::new(0, 1);
            a
        }
    };

    Ok(price)
}

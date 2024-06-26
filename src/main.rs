use std::{fs, str::FromStr};

use clap::Parser;
use cosmos::{Address, AddressHrp, Coin, CosmosNetwork, SeedPhrase};
use regex::Regex;

#[derive(Parser)]
struct Args {
    coin: String,
    address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let address = Address::from_str(&args.address).unwrap();

    let coin_regex = Regex::new(r"^(\d+)([a-zA-z0-9/]+)$").unwrap();

    let captures = coin_regex.captures(&args.coin).unwrap();

    let amount = captures.get(1).unwrap().as_str();

    let denom = captures.get(2).unwrap().as_str();

    let coin = Coin {
        denom: denom.to_owned(),
        amount: amount.to_owned(),
    };

    let coins = vec![coin];

    let cosmos = CosmosNetwork::OsmosisTestnet.connect().await.unwrap();

    let secret = fs::read_to_string("./secret").unwrap();

    let wallet = SeedPhrase::from_str(secret.trim())
        .unwrap()
        .with_hrp(AddressHrp::from_static("osmo"))
        .unwrap();

    let result = wallet.send_coins(&cosmos, address, coins).await.unwrap();

    println!("{}", result.txhash);
}

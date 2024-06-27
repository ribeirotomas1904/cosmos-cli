use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use cosmos::{Address, AddressHrp, CosmosNetwork, ParsedCoin, SeedPhrase};

#[derive(Parser)]
struct Args {
    coin: String,
    address: String,

    #[arg(long = "cosmos-wallet", env = "COSMOS_WALLET")]
    cosmos_wallet: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let parsed_coin = ParsedCoin::from_str(&args.coin)
        .with_context(|| format!("Invalid coin: {:?}", args.coin))?;

    let coins = vec![parsed_coin.into()];

    let address = Address::from_str(&args.address)
        .with_context(|| format!("Invalid address: {:?}", args.address))?;

    let wallet = SeedPhrase::from_str(&args.cosmos_wallet)
        .with_context(|| format!("Invalid cosmos wallet: {:?}", args.cosmos_wallet))?
        .with_hrp(AddressHrp::from_static("osmo"))
        .with_context(|| "Invalid address HRP".to_string())?;

    let cosmos = CosmosNetwork::OsmosisTestnet
        .connect()
        .await
        .with_context(|| "Failed to connect to the Osmosis testnet".to_string())?;

    let result = wallet
        .send_coins(&cosmos, address, coins)
        .await
        .with_context(|| "Failed to send coins".to_string())?;

    Ok(println!("Transaction hash: {}", result.txhash))
}

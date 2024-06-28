use anyhow::Context;
use clap::Parser;
use cosmos::{Address, AddressHrp, CosmosNetwork, ParsedCoin, SeedPhrase};

#[derive(Parser)]
struct Args {
    coin: ParsedCoin,
    address: Address,

    #[arg(long = "cosmos-wallet", env = "COSMOS_WALLET")]
    cosmos_wallet: SeedPhrase,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let coins = vec![args.coin.into()];

    let wallet = args
        .cosmos_wallet
        .with_hrp(AddressHrp::from_static("osmo"))
        .with_context(|| "Invalid address HRP".to_string())?;

    let cosmos = CosmosNetwork::OsmosisTestnet
        .connect()
        .await
        .with_context(|| "Failed to connect to the Osmosis testnet".to_string())?;

    let result = wallet
        .send_coins(&cosmos, args.address, coins)
        .await
        .with_context(|| "Failed to send coins".to_string())?;

    Ok(println!("Transaction hash: {}", result.txhash))
}

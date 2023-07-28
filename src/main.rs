use clap::Parser;

#[derive(Parser)]
struct Cli {
    // Stake Pool bech 32 ID
    pool_id: String,
}

#[tokio::main]
async fn main() -> blockfrost::Result<()> {
    let args = Cli::parse();
    let pool_id = &args.pool_id;

    let pkhs = snapshotter::fetch_pkhs(pool_id).await.expect("Error");

    for pkh in pkhs {
        println!("{}", pkh);
    }

    Ok(())
}

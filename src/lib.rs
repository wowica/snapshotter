use blockfrost::{load, BlockFrostApi};
use pallas_addresses::Address;

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let configurations = load::configurations_from_env()?;
    let project_id = configurations["project_id"].as_str().unwrap();
    let mut api = BlockFrostApi::new(project_id, Default::default());

    api.settings = api.settings.use_preview();

    Ok(api)
}

async fn fetch_delegators(pool_id: &str) -> blockfrost::Result<Vec<String>> {
    let api = build_api().expect("Error");
    let pool_bech32 = String::from(pool_id);
    let delegators = api.pools_delegators(&pool_bech32).await.expect("Error");
    let delegator_addresses = delegators.into_iter().map(|d| d.address).collect();

    Ok(delegator_addresses)
}

async fn fetch_payment_addresses(stake_address: &str) -> blockfrost::Result<Vec<String>> {
    let api = build_api().expect("Error");
    let addresses = api.accounts_addresses(stake_address).await.expect("Error");
    let payment_addresses = addresses
        .into_iter()
        .map(|address| address.address)
        .collect();

    Ok(payment_addresses)
}

pub fn derive_pkh_from_address(address: Address) -> Option<String> {
    match address {
        Address::Shelley(shelley_address) => {
            let hash = shelley_address.payment().as_hash().to_vec();
            let s_hash: Vec<_> = hash.iter().map(|b| format!("{:02x}", b)).collect();
            let ss_hash = s_hash.join("");
            Some(ss_hash)
        }
        _ => None,
    }
}

pub async fn fetch_pkhs(pool_id: &str) -> Option<Vec<String>> {
    let delegators = fetch_delegators(pool_id).await.expect("Error");
    let mut pkhs = Vec::new();

    for delegator in delegators.iter() {
        //println!("Stake Address: {}", delegator);
        let addresses = fetch_payment_addresses(delegator).await.expect("Error");
        for address in addresses {
            //println!("\tPayment Address: {}", address);
            let pkh = derive_pkh_from_address(Address::from_bech32(&address).unwrap()).unwrap();
            //println!("{}", pkh);
            pkhs.push(pkh);
        }
    }

    Some(pkhs)
}

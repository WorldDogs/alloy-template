use alloy::{
    providers::{Provider, ProviderBuilder},
    sol,
};

sol!(
    #[sol(rpc)]
    Fluid,
    "abi/abi.json"
);

#[tokio::main]
async fn main() {
    let rpc = "https://eth.llamarpc.com".parse().unwrap();
    let provider = ProviderBuilder::new().on_http(rpc);
    let bn = provider.get_block_number().await.unwrap();
    println!("Current block number: {}", bn);
}

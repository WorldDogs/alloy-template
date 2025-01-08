use alloy::{providers::{Provider, ProviderBuilder}, sol};



sol!(
    #[sol(rpc)]
    Fluid,
    "abi/abi.json"

);

#[tokio::main]
async fn main() {
}

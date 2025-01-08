use alloy::{
    providers::{Provider, ProviderBuilder},
    primitives::address,
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
    let addr =  address!("5C20B550819128074FD538Edf79791733ccEdd18");
    let fluid = Fluid::new(addr,provider.clone());
    let name = fluid.name().call().await.unwrap();
    println!("Name: {}", name._0);
}

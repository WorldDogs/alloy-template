use alloy::{
    primitives::{address, Address}, providers::{Provider, ProviderBuilder}, sol
};
use clap::Parser;


sol!(
    #[sol(rpc)]
    Fluid,
    "abi/abi.json"
);

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    addr: Address,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();
    println!("Address: {}", args.addr);

    let rpc = "https://eth.llamarpc.com".parse().unwrap();
    let provider = ProviderBuilder::new().on_http(rpc);
    let bn = provider.get_block_number().await.unwrap();
    println!("Current block number: {}", bn);
    let addr =  address!("5C20B550819128074FD538Edf79791733ccEdd18");
    let fluid = Fluid::new(addr,provider.clone());
    let name = fluid.name().call().await.unwrap();
    println!("Name: {}", name._0);

    // calc balance 
}

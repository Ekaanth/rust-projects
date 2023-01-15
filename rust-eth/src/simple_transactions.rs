use std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet,Middleware,Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};

use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = "hip switch rival lizard sick glide message oblige position sentence bulb wrong";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();

    println!("HTTP end point: {}", ganache.endpoint());

    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();

    println!("Waller first address: {}", first_address.encode_hex::<String>());
    
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    let first_balance = provider.get_balance(first_address, None).await?;


    println!("Wallet first address balance: {}", first_balance);

    let other_address_hex = "0xd5e3941f2e15649457B010FBA435bd2b24f647bB";
    let other_address = other_address_hex.parse::<Address>()?;

    let other_balance = provider.get_balance(other_address, None).await?;

    println!("Waller other address: {}", other_address.encode_hex::<String>());
    println!("Wallet other address balance: {}", other_balance);

    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);

    let receipt = provider
    .send_transaction(tx, None)
    .await?
    .log_msg("Pending transfer")
    .confirmations(1)
    .await?
    .context("Missing receipt")?;


    println!("TX minted in block {}", receipt.block_number.context("can't get block number")?);

    println!("Balance of {} is {}", other_address_hex, provider.get_balance(other_address,None).await?);

    Ok(())

}
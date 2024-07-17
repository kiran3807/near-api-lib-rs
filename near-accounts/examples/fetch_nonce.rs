use near_accounts::Account;
use near_primitives::types::AccountId;
use near_providers::JsonRpcProvider;
use near_crypto::{InMemorySigner, SecretKey};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer_account_id: AccountId= "your account name".parse()?;
    let signer_secret_key: SecretKey = "private key without the ed25519 or secp256k1 prefix".parse()?;
    
    let provider = Arc::new(JsonRpcProvider::new("https://rpc.testnet.near.org"));
    let signer = Arc::new(InMemorySigner::from_secret_key(
        signer_account_id.clone(), 
        signer_secret_key
    ));
    let account = Account::new(signer_account_id, signer.clone(), provider);
    let public_key = signer.public_key.clone();

    let result = account.fetch_nonce(&account.account_id, &public_key).await?;

    println!("response: {:#?}", result);

    Ok(())
}

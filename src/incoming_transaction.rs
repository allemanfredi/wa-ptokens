use serde::Deserialize;

#[derive(PartialEq, Deserialize)]
pub struct IncomingTransaction {
    pub broadcast: bool,
    pub broadcast_tx_hash: String,
}

pub fn get(
    transaction: &String,
    endpoint: &String,
) -> Result<IncomingTransaction, Box<dyn std::error::Error>> {
    let req = format!("{}/incoming-tx-hash/{}", endpoint, transaction);
    let incoming_transaction: IncomingTransaction = reqwest::blocking::get(&req[..])?.json()?;
    Ok(incoming_transaction)
}

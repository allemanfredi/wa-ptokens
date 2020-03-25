use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Response {
    enclavePublicKey: String,
    nonce: u32,
    pub nativeDepositAddress: String,
}

pub fn get(address: &String, endpoint: &String) -> Result<String, Box<dyn std::error::Error>> {
    let req = format!("{}/get-native-deposit-address/{}", endpoint, address);
    let resp: Response = reqwest::blocking::get(&req[..])?.json()?;
    Ok(resp.nativeDepositAddress)
}

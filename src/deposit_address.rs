use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DepositAddress {
    enclavePublicKey: String,
    nonce: u32,
    pub nativeDepositAddress: String,
}

impl DepositAddress {
  fn verify(&self, )
}

pub fn get(address: &String, endpoint: &String) -> Result<String, Box<dyn std::error::Error>> {
    let req = format!("{}/get-native-deposit-address/{}", endpoint, address);
    let resp: DepositAddress = reqwest::blocking::get(&req[..])?.json()?;
    Ok(resp.nativeDepositAddress)
}

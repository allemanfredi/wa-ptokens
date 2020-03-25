extern crate hex;

use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::Builder;
use bitcoin::blockdata::script::Script;
use bitcoin::hashes::{sha256d, Hash};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use hex::FromHex;
use serde::Deserialize;
use std::str;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DepositAddress {
    enclavePublicKey: String,
    nonce: i64,
    pub nativeDepositAddress: String,
}

impl DepositAddress {
    fn verify(&self, host_address: &String, network: &Network) -> bool {
        let mut host_address_and_nonce =
            Vec::from_hex(&host_address[2..host_address.len()]).unwrap();
        host_address_and_nonce.extend_from_slice(&self.nonce.to_le_bytes());
        let host_address_and_nonce_hash = sha256d::Hash::hash(&host_address_and_nonce.to_vec());

        let script = Builder::new()
            .push_slice(&host_address_and_nonce_hash)
            .push_opcode(opcodes::all::OP_DROP)
            .push_slice(Vec::from_hex(&self.enclavePublicKey).unwrap().as_slice())
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();

        let address = Address::p2sh(&Script::from(script), *network);

        if address.to_string() == self.nativeDepositAddress {
            return true;
        }

        false
    }
}

pub fn get(
    address: &String,
    endpoint: &String,
    network: &Network,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = format!("{}/get-native-deposit-address/{}", endpoint, address);
    let deposit_address: DepositAddress = reqwest::blocking::get(&req[..])?.json()?;
    match deposit_address.verify(&address, &network) {
        true => Ok(deposit_address.nativeDepositAddress),
        false => Err("Error during address generation".into()),
    }
}

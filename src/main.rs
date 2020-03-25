#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate serde;

use rocket::request::Form;

mod constants;
mod deposit_address;
mod request;
mod wa_message;

#[post("/new-message", data = "<message>")]
fn new_message(message: Form<wa_message::Message>) -> String {
    let request: request::Request = request::parse(&message.Body);
    println!("new request: {}", request);

    let which: &str = match request.which {
        request::Which::PbtcOnEthMainnet => constants::PBTC_ON_ETH_MAINNET_ENDPOINT,
        request::Which::PbtcOnEthTestnet => constants::PBTC_ON_ETH_TESTNET_ENDPOINT,
    };

    match request.command {
        request::Command::GetDepositAddress => {
            deposit_address::get(&request.data, &which.to_string()).unwrap()
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![new_message]).launch();
}

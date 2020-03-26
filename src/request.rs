use std::fmt;
use std::fmt::Display;

// Which
pub enum Which {
    PbtcOnEthMainnet,
    PbtcOnEthTestnet,
}

impl Display for Which {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Which::PbtcOnEthMainnet => write!(f, "PBTC_ON_ETH_MAINNET"),
            Which::PbtcOnEthTestnet => write!(f, "PBTC_ON_ETH_TESTNET"),
        }
    }
}

// Command
pub enum Command {
    GetDepositAddress,
    MonitorIncomingTransaction,
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::GetDepositAddress => write!(f, "GET_DEPOSIT_ADDRESS"),
            Command::MonitorIncomingTransaction => write!(f, "MONITOR_INCOMING_TRANSACTION"),
        }
    }
}

// Request
pub struct Request {
    pub which: Which,
    pub command: Command,
    pub data: String,
}

impl Request {
    fn new(which: Which, command: Command, data: String) -> Request {
        Request {
            which: which,
            command: command,
            data: data,
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}: {}", self.which, self.command, self.data)
    }
}

pub fn parse(cmd: &String) -> Request {
    let vec: Vec<String> = cmd.split(" ").map(|s| s.to_string()).collect();
    Request::new(
        match vec[0].as_str() {
            "pbtc-on-eth-testnet" => Which::PbtcOnEthTestnet,
            "pbtc-on-eth-mainnet" => Which::PbtcOnEthMainnet,
            _ => Which::PbtcOnEthMainnet,
        },
        match vec[1].as_str() {
            "dep-addr" => Command::GetDepositAddress,
            "inc-tx" => Command::MonitorIncomingTransaction,
            _ => Command::GetDepositAddress,
        },
        vec[2].clone(),
    )
}

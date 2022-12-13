use {
    anyhow::Result,
    serde::{Deserialize, Serialize},
    std::{
        fs::{read_to_string, File},
        io::Write,
    },
};

pub const CHAIN_NET_DIRECTORY: &str = "chain_net";

#[derive(Serialize, Deserialize)]
pub struct ChainNet {
    pub chain_net_name: String,
    pub chain_net_address: String,
    pub query_port: u32,
    pub submit_transaction_port: u32,
    pub tendermint_port: u32,
    pub web3_rpc_port: u32,
}
impl Default for ChainNet {
    fn default() -> Self {
        Self {
            chain_net_name: String::from("local"),
            chain_net_address: String::from("http://127.0.0.1"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
}
impl ChainNet {
    pub fn add(
        home_path: &str,
        chain_net_name: &str,
        chain_net_address: &str,
        query_port: u32,
        submit_transaction_port: u32,
        tendermint_port: u32,
        web3_rpc_port: u32,
    ) -> Result<Self> {
        let net = Self {
            chain_net_name: String::from(chain_net_name),
            chain_net_address: String::from(chain_net_address),
            query_port,
            submit_transaction_port,
            tendermint_port,
            web3_rpc_port,
        };
        let mut file = File::create(format!(
            "{}/{}/{}.json",
            home_path, CHAIN_NET_DIRECTORY, net.chain_net_name
        ))?;
        file.write_all(serde_json::to_string(&net)?.as_bytes())?;
        Ok(net)
    }

    #[inline(always)]
    pub fn load_from_file(file_name: &str) -> Result<Self> {
        let json = read_to_string(file_name)?;
        Ok(serde_json::from_str::<Self>(json.as_str())?)
    }

    pub fn show(&self) {
        println!(
            "\n\x1b[31;01m{: <25}:\x1b[00m {}",
            "ChainNet Name", self.chain_net_name
        );
        println!(
            "\x1b[31;01m{: <25}:\x1b[00m {}",
            "ChainNet Address", self.chain_net_address
        );
        println!(
            "\x1b[31;01m{: <25}:\x1b[00m {}",
            "Query Port", self.query_port
        );
        println!(
            "\x1b[31;01m{: <25}:\x1b[00m {}",
            "Submit Transaction Port", self.submit_transaction_port
        );
        println!(
            "\x1b[31;01m{: <25}:\x1b[00m {}",
            "Tendermint Port", self.tendermint_port
        );
        println!(
            "\x1b[31;01m{: <25}:\x1b[00m {}",
            "Web3 Rpc Port", self.web3_rpc_port
        );
    }

    pub fn mainnet() -> Self {
        Self {
            chain_net_name: String::from("mainnet"),
            chain_net_address: String::from("https://prod-mainnet.prod.findora.org"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
    pub fn testnet() -> Self {
        Self {
            chain_net_name: String::from("testnet"),
            chain_net_address: String::from("https://prod-testnet.prod.findora.org"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
    pub fn forge() -> Self {
        Self {
            chain_net_name: String::from("forge"),
            chain_net_address: String::from("https://prod-forge.prod.findora.org"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
    pub fn qa01() -> Self {
        Self {
            chain_net_name: String::from("qa01"),
            chain_net_address: String::from("https://dev-qa01.dev.findora.org"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
    pub fn qa02() -> Self {
        Self {
            chain_net_name: String::from("qa02"),
            chain_net_address: String::from("https://dev-qa02.dev.findora.org"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
    pub fn qa03() -> Self {
        Self {
            chain_net_name: String::from("qa03"),
            chain_net_address: String::from("https://dev-qa03.dev.findora.org"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
    pub fn qa04() -> Self {
        Self {
            chain_net_name: String::from("qa04"),
            chain_net_address: String::from("https://dev-qa04.dev.findora.org"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
}

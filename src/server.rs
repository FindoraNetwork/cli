use {
    anyhow::{anyhow, Result},
    serde::{Deserialize, Serialize},
    std::{
        fs::{read_to_string, File},
        io::Write,
        path::Path,
    },
};

const SERVER_FILE_NAME: &str = "server.json";

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub server_address: String,
    pub query_port: u32,
    pub submit_transaction_port: u32,
    pub tendermint_port: u32,
    pub web3_rpc_port: u32,
}
impl Default for Server {
    fn default() -> Self {
        Self {
            server_address: String::from("http://127.0.0.1"),
            query_port: 8668,
            submit_transaction_port: 8669,
            tendermint_port: 26657,
            web3_rpc_port: 8545,
        }
    }
}
impl Server {
    pub fn new(
        server_address: String,
        query_port: u32,
        submit_transaction_port: u32,
        tendermint_port: u32,
        web3_rpc_port: u32,
    ) -> Self {
        Self {
            server_address,
            query_port,
            submit_transaction_port,
            tendermint_port,
            web3_rpc_port,
        }
    }
    #[inline(always)]
    pub fn load_from_file(home_path: &str) -> Result<Self> {
        let file_name = format!("{}/{}", home_path, SERVER_FILE_NAME);
        if !Path::new(file_name.as_str()).exists() {
            return Err(anyhow!("server info does not exist, please init server"));
        }
        let json = read_to_string(file_name)?;
        Ok(serde_json::from_str::<Self>(json.as_str())?)
    }

    pub fn write_to_file(&self, home_path: &str) -> Result<()> {
        let file_name = format!("{}/{}", home_path, SERVER_FILE_NAME);
        let mut file = File::create(file_name)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }
}

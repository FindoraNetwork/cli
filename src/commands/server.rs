use {crate::server::Server, anyhow::Result, clap::Args};

#[derive(Debug, Args)]
///Asset Management
pub struct ServerCli {
    ///link to initialize findora
    #[arg(short, long, conflicts_with = "show")]
    init: bool,
    ///findora url, default http://127.0.0.1
    #[arg(
        short = 'u',
        long,
        value_name = "SERVER ADDRESS",
        conflicts_with = "show"
    )]
    server_address: Option<String>,
    ///query rpc port, default 8668
    #[arg(short, long, value_name = "QUERY PORT", conflicts_with = "show")]
    query_port: Option<u32>,
    ///submit transaction rpc port, default 8669
    #[arg(
        short = 'S',
        long,
        value_name = "SUBMIT TRANSACTIONN PORT",
        conflicts_with = "show"
    )]
    submit_transaction_port: Option<u32>,
    ///tendermint rpc port, default 26657
    #[arg(short, long, value_name = "TENDERMINT PORT", conflicts_with = "show")]
    tendermint_port: Option<u32>,
    ///tendermint rpc port, default 26657
    #[arg(short, long, value_name = "WEB3 RPC PORT", conflicts_with = "show")]
    web3_rpc_port: Option<u32>,
    ///show asset info
    #[arg(
        short,
        long,
        conflicts_with = "init",
        conflicts_with = "server_address",
        conflicts_with = "query_port",
        conflicts_with = "submit_transaction_port",
        conflicts_with = "tendermint_port"
    )]
    show: bool,
}

impl ServerCli {
    pub fn execute(self, home: &str) -> Result<()> {
        if self.init {
            let server_address = self
                .server_address
                .unwrap_or_else(|| String::from("http://127.0.0.1"));
            let query_port = self.query_port.unwrap_or(8668);
            let submit_transaction_port = self.submit_transaction_port.unwrap_or(8669);
            let tendermint_port = self.tendermint_port.unwrap_or(26657);
            let web3_rpc_port = self.web3_rpc_port.unwrap_or(8545);
            if let Err(e) = Server::new(
                server_address,
                query_port,
                submit_transaction_port,
                tendermint_port,
                web3_rpc_port,
            )
            .write_to_file(home)
            {
                println!("init server error: {:?}", e);
            }
        } else {
            match Server::load_from_file(home) {
                Ok(s) => {
                    println!(
                        "\x1b[31;01m{: <25}:\x1b[00m {}",
                        "Server Address", s.server_address
                    );
                    println!("\x1b[31;01m{: <25}:\x1b[00m {}", "Query Port", s.query_port);
                    println!(
                        "\x1b[31;01m{: <25}:\x1b[00m {}",
                        "Submit Transaction Port", s.submit_transaction_port
                    );
                    println!(
                        "\x1b[31;01m{: <25}:\x1b[00m {}",
                        "Tendermint Port", s.tendermint_port
                    );
                    println!(
                        "\x1b[31;01m{: <25}:\x1b[00m {}",
                        "Web3 Rpc Port", s.web3_rpc_port
                    );
                }
                Err(e) => {
                    println!("load server info error: {:?}", e);
                }
            }
        }
        Ok(())
    }
}

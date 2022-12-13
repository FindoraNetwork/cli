use {crate::chain_net::ChainNetMgr, anyhow::Result, clap::Args};

#[derive(Debug, Args)]
///Asset Management
pub struct ChainNet {
    ///link to initialize findora
    #[arg(short, long, conflicts_with = "show")]
    add: bool,
    ///findora url, default http://127.0.0.1
    #[arg(long, value_name = "CHAIN NET ADDRESS", conflicts_with = "show")]
    chain_net_address: Option<String>,
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
    ///chain net name
    #[arg(short = 'n', long, value_name = "CHAIN NET ADDRESS")]
    chain_net_name: Option<String>,
    ///show asset info
    #[arg(
        short,
        long,
        conflicts_with = "add",
        conflicts_with = "chain_net_address",
        conflicts_with = "query_port",
        conflicts_with = "submit_transaction_port",
        conflicts_with = "tendermint_port"
    )]
    show: bool,
}

impl ChainNet {
    pub fn execute(self, home: &str) -> Result<()> {
        if self.add {
            let chain_net_address = match self.chain_net_address.as_deref() {
                Some(val) => val,
                None => {
                    println!("chain_net_name mot found");
                    return Ok(());
                }
            };
            let query_port = self.query_port.unwrap_or(8668);
            let submit_transaction_port = self.submit_transaction_port.unwrap_or(8669);
            let tendermint_port = self.tendermint_port.unwrap_or(26657);
            let web3_rpc_port = self.web3_rpc_port.unwrap_or(8545);
            let chain_net_name = match self.chain_net_name.as_deref() {
                Some(val) => val,
                None => {
                    println!("chain_net_name not found");
                    return Ok(());
                }
            };
            if let Err(e) = ChainNetMgr::add(
                home,
                chain_net_name,
                chain_net_address,
                query_port,
                submit_transaction_port,
                tendermint_port,
                web3_rpc_port,
            ) {
                println!("add chain_net error: {:?}", e);
            }
        } else {
            let mgr = match ChainNetMgr::load_all(home) {
                Ok(v) => v,
                Err(e) => {
                    println!("load chain_net error:{}", e);
                    return Ok(());
                }
            };
            match self.chain_net_name.as_deref() {
                Some(val) => {
                    match mgr.chain_nets.get(val) {
                        Some(v) => v.show(),
                        None => {
                            println!("chain_net_name not found");
                            return Ok(());
                        }
                    };
                }
                None => {
                    mgr.show();
                }
            };
        }
        Ok(())
    }
}

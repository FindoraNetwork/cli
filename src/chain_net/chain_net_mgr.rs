use {
    crate::chain_net::{ChainNet, CHAIN_NET_DIRECTORY},
    anyhow::Result,
    std::{
        collections::HashMap,
        fs::{create_dir_all, read_dir},
        path::Path,
    },
};

pub struct ChainNetMgr {
    pub home: String,
    pub chain_nets: HashMap<String, ChainNet>,
}

impl ChainNetMgr {
    pub fn add(
        home: &str,
        chain_net_name: &str,
        chain_net_address: &str,
        query_port: u32,
        submit_transaction_port: u32,
        tendermint_port: u32,
        web3_rpc_port: u32,
    ) -> Result<Self> {
        let chain_net_path = format!("{}/{}", home, CHAIN_NET_DIRECTORY);
        let chain_net_path = Path::new(chain_net_path.as_str());
        if !chain_net_path.exists() {
            create_dir_all(chain_net_path)?;
        }
        let mut chain_nets = HashMap::new();
        let chain_net = ChainNet::add(
            home,
            chain_net_name,
            chain_net_address,
            query_port,
            submit_transaction_port,
            tendermint_port,
            web3_rpc_port,
        )?;
        chain_nets.insert(chain_net.chain_net_name.clone(), chain_net);

        Ok(Self {
            home: String::from(home),
            chain_nets,
        })
    }
    pub fn load_all(home: &str) -> Result<Self> {
        let mut mgr = Self::load_from_file(home)?;
        let chain_net = ChainNet::mainnet();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        let chain_net = ChainNet::testnet();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        let chain_net = ChainNet::forge();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        let chain_net = ChainNet::qa01();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        let chain_net = ChainNet::qa02();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        let chain_net = ChainNet::qa03();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        let chain_net = ChainNet::qa04();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        let chain_net = ChainNet::default();
        mgr.chain_nets
            .insert(chain_net.chain_net_name.clone(), chain_net);
        Ok(mgr)
    }

    fn load_from_file(home: &str) -> Result<Self> {
        let chain_net_path = format!("{}/{}", home, CHAIN_NET_DIRECTORY);
        let chain_net_path = Path::new(chain_net_path.as_str());
        if !chain_net_path.exists() {
            create_dir_all(chain_net_path)?;
        }
        let mut chain_nets = HashMap::new();
        for path in read_dir(chain_net_path)? {
            let file = path?.path();
            if !file.is_dir() {
                let chain_net = ChainNet::load_from_file(file.display().to_string().as_str())?;
                chain_nets.insert(chain_net.chain_net_name.clone(), chain_net);
            }
        }
        Ok(Self {
            home: String::from(home),
            chain_nets,
        })
    }

    pub fn show(&self) {
        for (_, chain_net) in self.chain_nets.iter() {
            chain_net.show();
        }
    }
}

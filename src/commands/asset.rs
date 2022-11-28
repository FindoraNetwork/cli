use {
    crate::{server::Server, wallet::AccountMgr},
    anyhow::Result,
    clap::Args,
};

#[derive(Debug, Args)]
///Asset Management
pub struct Asset {
    ///the address of the asset to display, default all
    #[arg(short, long)]
    address: Option<String>,
    ///show asset type, default all
    #[arg(short, long = "type", value_name = "TYPE")]
    typ: Option<String>,
}

impl Asset {
    pub fn execute(self, home: &str) -> Result<()> {
        let mgr = match AccountMgr::load_from_file(home) {
            Ok(val) => val,
            Err(e) => {
                println!("load account error: {:?}", e);
                return Ok(());
            }
        };
        let server = match Server::load_from_file(home) {
            Ok(val) => val,
            Err(e) => {
                println!("load server info error: {:?}", e);
                return Ok(());
            }
        };
        let address = match self.address.as_ref() {
            Some(val) => vec![val.clone()],
            None => {
                let mut address = vec![];
                for (addr, _) in mgr.accounts.iter() {
                    address.push(addr.clone());
                }
                address
            }
        };

        Ok(())
    }
}

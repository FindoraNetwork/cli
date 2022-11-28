use {
    crate::{
        asset::{get_evm_balance, get_owned_utxo_balance},
        server::Server,
        wallet::{AccountMgr, AccountType},
    },
    anyhow::Result,
    clap::Args,
    primitive_types::U256,
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
        for addr in address {
            let account = match mgr.accounts.get(&addr) {
                Some(acc) => acc.clone(),
                None => {
                    println!("account {} not found", addr);
                    return Ok(());
                }
            };
            let pk = match account.get_key_pair() {
                Ok(pk) => pk.get_pk(),
                Err(e) => {
                    println!("account {} key pair error:{}", addr, e);
                    return Ok(());
                }
            };
            println!("\n\x1b[31;01mAddress:\x1b[00m {}", addr);
            match account.account_type {
                AccountType::Evm => {
                    let balance = match get_evm_balance(&server, &addr) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("account {} get_evm_balance error:{}", addr, e);
                            return Ok(());
                        }
                    };
                    if U256::zero() == balance {
                        println!("\t\x1b[31;01mAmount:\x1b[00m 0");
                    } else {
                        println!("\t\x1b[31;01mEvm Balance:\x1b[00m {}", balance);
                    }
                }
                _ => {
                    let utxo = match get_owned_utxo_balance(&server, &pk) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("account {} get_owned_utxos error:{}", addr, e);
                            return Ok(());
                        }
                    };
                    if utxo.is_empty() {
                        println!("\t\x1b[31;01mAmount:\x1b[00m 0");
                    } else {
                        for (asset, amount) in utxo {
                            println!("\t\x1b[31;01m{}:\x1b[00m {}", asset, amount);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

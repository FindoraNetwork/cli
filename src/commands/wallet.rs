use {
    crate::wallet::{AccountMgr, AccountType},
    anyhow::Result,
    clap::Args,
};

#[derive(Debug, Args)]
///Wallet management
pub struct Wallet {
    ///init wallet
    #[arg(
        short,
        long,
        conflicts_with = "create",
        conflicts_with = "typ",
        conflicts_with = "show"
    )]
    init: bool,
    ///new account password
    #[arg(short, long)]
    passphrase: Option<String>,
    ///create a new account
    #[arg(
        short,
        long,
        conflicts_with = "init",
        conflicts_with = "passphrase",
        conflicts_with = "show"
    )]
    create: bool,
    ///type(fra/eth/evm) of new account, default fra
    #[arg(
        short = 't',
        long = "type",
        conflicts_with = "init",
        conflicts_with = "passphrase",
        conflicts_with = "show",
        value_name = "TYPE"
    )]
    typ: Option<String>,
    ///show all account info
    #[arg(
        short,
        long,
        conflicts_with = "init",
        conflicts_with = "passphrase",
        conflicts_with = "create",
        conflicts_with = "typ"
    )]
    show: bool,
    ///import private key
    #[arg(
        short = 'I',
        long,
        conflicts_with = "init",
        conflicts_with = "passphrase",
        conflicts_with = "create",
        conflicts_with = "show",
        value_name = "PRIVATE KEY"
    )]
    import: Option<String>,
}

impl Wallet {
    pub fn execute(&self, home: &str) -> Result<()> {
        if self.init {
            let lang = "en";
            let wordslen = 24;
            let passphrase = self.passphrase.as_deref().unwrap_or_default();

            if let Err(e) = AccountMgr::init(lang, wordslen, passphrase, home) {
                println!("init error: {}", e);
            }
        } else if self.create {
            let account_type = match self.typ.clone().unwrap_or_default().as_str() {
                "fra" => AccountType::Fra,
                "eth" => AccountType::Eth,
                "evm" => AccountType::Evm,
                _ => AccountType::Fra,
            };
            match AccountMgr::load_from_file(home) {
                Ok(mut mgr) => {
                    if let Err(e) = mgr.generate_account(account_type, home) {
                        println!("generate_account error: {}", e);
                    }
                }
                Err(e) => println!("load_from_file error: {}", e),
            };
        } else if self.import.is_some() {
            let account_type = match self.typ.clone().unwrap_or_default().as_str() {
                "fra" => AccountType::Fra,
                "eth" => AccountType::Eth,
                "evm" => AccountType::Evm,
                _ => {
                    println!("please specify the import type(fra/eth/evm)");
                    return Ok(());
                }
            };
            let key = self.import.as_deref().unwrap_or_default();
            match AccountMgr::load_from_file(home) {
                Ok(mut mgr) => {
                    if let Err(e) = mgr.import_from_private_key(account_type, key) {
                        println!("import_from_private_key error: {}", e);
                    }
                }
                Err(e) => println!("load_from_file error: {}", e),
            };
        } else {
            match AccountMgr::load_from_file(home) {
                Ok(mgr) => {
                    if let Err(e) = mgr.show() {
                        println!("show account error: {}", e);
                    }
                }
                Err(e) => println!("load_from_file error: {}", e),
            };
        }
        Ok(())
    }
}

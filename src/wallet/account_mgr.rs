use {
    super::{Account, RootAccount, ACCOUNT_DIRECTORY},
    crate::wallet::AccountType,
    anyhow::Result,
    std::{
        collections::HashMap,
        fs::{create_dir_all, read_dir},
        path::Path,
    },
};

pub struct AccountMgr {
    pub home: String,
    pub root_account: RootAccount,
    pub accounts: HashMap<String, Account>,
}

impl AccountMgr {
    pub fn init(
        lang: &str,
        wordslen: u8,
        mnemonic: Option<String>,
        passphrase: &str,
        home: &str,
    ) -> Result<Self> {
        let home_path = Path::new(home);
        if !home_path.exists() {
            create_dir_all(home_path)?;
        }
        let (root_account, mnemonic) =
            RootAccount::generate(lang, wordslen, mnemonic, passphrase, home)?;

        let account_path = format!("{}/{}", home, ACCOUNT_DIRECTORY);
        let account_path = Path::new(account_path.as_str());
        if !account_path.exists() {
            create_dir_all(account_path)?;
        }
        let mut accounts = HashMap::new();
        for path in read_dir(account_path)? {
            let file = path?.path();
            if !file.is_dir() {
                let account = Account::load_from_file(file.display().to_string().as_str())?;
                accounts.insert(account.address.clone(), account);
            }
        }

        println!("\x1b[31;01mGenerate a new Mnemonic, please backup it\x1b[00m");
        println!("\x1b[31;01mMnemonic:\x1b[00m {}", mnemonic);

        let seed = root_account.get_seed()?;
        let account = Account::generate(AccountType::Fra, 0, &seed, home)?;
        accounts.insert(account.address.clone(), account);
        let account = Account::generate(AccountType::Eth, 1, &seed, home)?;
        accounts.insert(account.address.clone(), account);
        let account = Account::generate(AccountType::Evm, 2, &seed, home)?;
        accounts.insert(account.address.clone(), account);
        Ok(AccountMgr {
            home: String::from(home),
            root_account,
            accounts,
        })
    }

    pub fn load_from_file(home_path: &str) -> Result<Self> {
        let root_account = RootAccount::load_from_file(home_path)?;
        let account_path = format!("{}/{}", home_path, ACCOUNT_DIRECTORY);
        let account_path = Path::new(account_path.as_str());
        if !account_path.exists() {
            create_dir_all(account_path)?;
        }
        let mut accounts = HashMap::new();
        for path in read_dir(account_path)? {
            let file = path?.path();
            if !file.is_dir() {
                let account = Account::load_from_file(file.display().to_string().as_str())?;
                accounts.insert(account.address.clone(), account);
            }
        }
        Ok(AccountMgr {
            home: String::from(home_path),
            root_account,
            accounts,
        })
    }
    pub fn import_from_private_key(
        &mut self,
        account_type: AccountType,
        private_key: &str,
    ) -> Result<()> {
        let account =
            Account::import_from_private_key(self.home.as_str(), account_type, private_key)?;
        account.show()?;
        self.accounts.insert(account.address.clone(), account);
        Ok(())
    }
    pub fn show(&self) -> Result<()> {
        for (_, account) in self.accounts.iter() {
            account.show()?;
        }
        Ok(())
    }

    pub fn generate_account(&mut self, account_type: AccountType, home_path: &str) -> Result<()> {
        let seed = self.root_account.get_seed()?;
        let account =
            Account::generate(account_type, self.accounts.len() as u32, &seed, home_path)?;
        account.show()?;
        self.accounts.insert(account.address.clone(), account);
        Ok(())
    }
}

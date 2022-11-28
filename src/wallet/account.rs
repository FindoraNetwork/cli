use {
    anyhow::{anyhow, Result},
    bech32::ToBase32,
    ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey},
    noah::xfr::sig::{
        convert_libsecp256k1_public_key_to_address, XfrKeyPair, XfrPublicKey, XfrPublicKeyInner,
        XfrSecretKey,
    },
    noah_algebra::serialization::NoahFromToBytes,
    primitive_types::H160,
    serde::{Deserialize, Serialize},
    std::{
        fs::{read_to_string, File},
        io::Write,
    },
};

const ETH: u32 = 60;
const FRA: u32 = 917;

pub(crate) const ACCOUNT_DIRECTORY: &str = "accounts";
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AccountType {
    Fra,
    Eth,
    Evm,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub private_key: String,
    pub account_type: AccountType,
    pub num: u32,
    pub address: String,
}
impl Account {
    pub fn generate(
        account_type: AccountType,
        num: u32,
        seed: &[u8; 64],
        home_path: &str,
    ) -> Result<Self> {
        let account = match account_type {
            AccountType::Fra => Self::generate_fra(num, seed),
            AccountType::Eth => Self::generate_eth(num, seed),
            AccountType::Evm => Self::generate_evm(num, seed),
        }?;
        let mut file = File::create(format!(
            "{}/{}/{}.json",
            home_path, ACCOUNT_DIRECTORY, account.address
        ))?;
        file.write_all(serde_json::to_string(&account)?.as_bytes())?;
        Ok(account)
    }

    #[inline(always)]
    pub fn load_from_file(file_name: &str) -> Result<Self> {
        let json = read_to_string(file_name)?;
        let account = serde_json::from_str::<Self>(json.as_str())?;
        Ok(account)
    }
    pub fn import_from_private_key(
        home_path: &str,
        account_type: AccountType,
        private_key: &str,
    ) -> Result<Self> {
        let key = if let Some(stripped) = private_key.strip_prefix("0x") {
            stripped.to_string()
        } else {
            private_key.to_string()
        };
        let data = hex::decode(key.as_str())?;
        if 32 != data.len() {
            return Err(anyhow!(
                "Invalid length, required 32, actual {}",
                data.len()
            ));
        }
        let (account_type, address) = match account_type {
            AccountType::Fra => {
                let key_pair = XfrSecretKey::noah_from_bytes(&data)
                    .map_err(|e| anyhow!("XfrSecretKey::noah_from_bytes error {:?}", e))?
                    .into_keypair();

                let bytes = &XfrPublicKey::noah_to_bytes(&key_pair.pub_key);
                println!("{}", hex::encode(bytes));
                let address = if 0u8 == bytes[0] {
                    bech32::encode("fra", (<&[u8; 32]>::try_from(&bytes[1..33])?).to_base32())?
                } else {
                    return Err(anyhow!("fra addr format error"));
                };
                (AccountType::Fra, address)
            }
            AccountType::Eth => {
                let key_pair = XfrKeyPair::generate_secp256k1_from_bytes(&data).map_err(|e| {
                    anyhow!("XfrKeyPair::generate_secp256k1_from_bytes error {:?}", e)
                })?;
                let bytes = XfrPublicKey::noah_to_bytes(&key_pair.pub_key);
                let address = if 1u8 == bytes[0] {
                    bech32::encode("eth", (<&[u8; 32]>::try_from(&bytes[1..33])?).to_base32())?
                } else {
                    return Err(anyhow!("eth addr format error"));
                };
                (AccountType::Eth, address)
            }
            AccountType::Evm => {
                let key_pair = XfrKeyPair::generate_secp256k1_from_bytes(&data).map_err(|e| {
                    anyhow!("XfrKeyPair::generate_secp256k1_from_bytes error {:?}", e)
                })?;
                let address =
                    if let XfrPublicKeyInner::Secp256k1(pub_key) = key_pair.pub_key.inner() {
                        H160::from(convert_libsecp256k1_public_key_to_address(pub_key))
                    } else {
                        return Err(anyhow!("evm public key error"));
                    };
                (AccountType::Evm, format!("{:?}", address))
            }
        };
        let account = Account {
            private_key: format!("0x{}", key),
            account_type,
            num: 0,
            address,
        };
        let mut file = File::create(format!(
            "{}/{}/{}.json",
            home_path, ACCOUNT_DIRECTORY, account.address
        ))?;
        file.write_all(serde_json::to_string(&account)?.as_bytes())?;
        Ok(account)
    }
    pub fn get_key_pair(&self) -> Result<XfrKeyPair> {
        let private_key = if self.private_key.starts_with("0x") {
            self.private_key[2..].to_string()
        } else {
            self.private_key.to_string()
        };
        let data = hex::decode(private_key)?;
        let kp = match self.account_type {
            AccountType::Fra => XfrSecretKey::noah_from_bytes(&data)
                .map_err(|e| anyhow!("XfrSecretKey::noah_from_bytes error {:?}", e))?
                .into_keypair(),
            AccountType::Eth => XfrKeyPair::generate_secp256k1_from_bytes(&data)
                .map_err(|e| anyhow!("XfrKeyPair::generate_secp256k1_from_bytes error {:?}", e))?,
            AccountType::Evm => XfrKeyPair::generate_secp256k1_from_bytes(&data)
                .map_err(|e| anyhow!("XfrKeyPair::generate_secp256k1_from_bytes error {:?}", e))?,
        };
        Ok(kp)
    }

    pub fn show(&self) -> Result<()> {
        println!(
            "\x1b[31;01m{:?} Address:\x1b[00m {}",
            self.account_type, self.address
        );
        let pub_key = XfrPublicKey::noah_to_bytes(&self.get_key_pair()?.pub_key);
        println!(
            "\x1b[31;01m{:?} Public Key in hex:\x1b[00m 0x{}",
            self.account_type,
            match self.account_type {
                AccountType::Fra => hex::encode(&pub_key),
                AccountType::Eth => hex::encode(&pub_key),
                AccountType::Evm => hex::encode(&pub_key[1..]),
            }
        );
        println!("\x1b[31;01mAmount:\x1b[00m {}\n", get_amount());
        Ok(())
    }

    fn generate_fra(num: u32, seed: &[u8; 64]) -> Result<Self> {
        let key_pair = DerivationPath::bip44(FRA, num, num, num)
            .map_err(|e| anyhow!("DerivationPath::bip44 error:{:?}", e))
            .and_then(|path| {
                ExtendedSecretKey::from_seed(seed)
                    .map_err(|e| anyhow!("ExtendedSecretKey::from_seed error:{:?}", e))
                    .and_then(|kp| {
                        kp.derive(&path)
                            .map_err(|e| anyhow!("kp.derive error:{:?}", e))
                    })
            })
            .and_then(|kp| {
                XfrSecretKey::noah_from_bytes(&kp.secret_key.to_bytes()[..])
                    .map_err(|e| anyhow!("XfrSecretKey::noah_from_bytes error:{:?}", e))
                    .map(|secret_key| secret_key.into_keypair())
            })?;
        let bytes = XfrPublicKey::noah_to_bytes(&key_pair.pub_key);
        let address = if 0u8 == bytes[0] {
            bech32::encode("fra", (<&[u8; 32]>::try_from(&bytes[1..33])?).to_base32())?
        } else {
            return Err(anyhow!("fra format error"));
        };
        Ok(Account {
            private_key: format!("0x{}", hex::encode(&key_pair.get_sk().to_bytes()[1..])),
            account_type: AccountType::Fra,
            num,
            address,
        })
    }

    fn generate_eth(num: u32, seed: &[u8; 64]) -> Result<Self> {
        let key_pair = format!("m/44'/{}'/{}'/{}/{}", ETH, num, num, num)
            .parse::<bip32::DerivationPath>()
            .map_err(|e| anyhow!("parse::<bip32::DerivationPath> error:{:?}", e))
            .and_then(|path| {
                bip32::XPrv::derive_from_path(seed, &path)
                    .map_err(|e| anyhow!("bip32::XPrv::derive_from_path error:{:?}", e))
            })
            .and_then(|ext| {
                XfrKeyPair::generate_secp256k1_from_bytes(&ext.to_bytes())
                    .map_err(|e| anyhow!("XfrKeyPair::generate_secp256k1_from_bytes error:{:?}", e))
            })?;
        let bytes = &XfrPublicKey::noah_to_bytes(&key_pair.pub_key);
        let address = if 1u8 == bytes[0] {
            bech32::encode("eth", (<&[u8; 32]>::try_from(&bytes[1..33])?).to_base32())?
        } else {
            return Err(anyhow!("eth format error"));
        };
        Ok(Account {
            private_key: format!("0x{}", hex::encode(&key_pair.get_sk().to_bytes()[1..])),
            account_type: AccountType::Eth,
            num,
            address,
        })
    }

    fn generate_evm(num: u32, seed: &[u8; 64]) -> Result<Self> {
        let key_pair = format!("m/44'/{}'/{}'/{}/{}", ETH, num, num, num)
            .parse::<bip32::DerivationPath>()
            .map_err(|e| anyhow!("parse::<bip32::DerivationPath> error:{:?}", e))
            .and_then(|path| {
                bip32::XPrv::derive_from_path(seed, &path)
                    .map_err(|e| anyhow!("bip32::XPrv::derive_from_path error:{:?}", e))
            })
            .and_then(|ext| {
                XfrKeyPair::generate_secp256k1_from_bytes(&ext.to_bytes())
                    .map_err(|e| anyhow!("XfrKeyPair::generate_secp256k1_from_bytes error:{:?}", e))
            })?;
        let address = if let XfrPublicKeyInner::Secp256k1(pub_key) = key_pair.pub_key.inner() {
            H160::from(convert_libsecp256k1_public_key_to_address(pub_key))
        } else {
            return Err(anyhow!("evm public key error"));
        };
        Ok(Account {
            private_key: format!("0x{}", hex::encode(&key_pair.get_sk().to_bytes()[1..])),
            account_type: AccountType::Evm,
            num,
            address: format!("{:?}", address),
        })
    }
}

fn get_amount() -> u64 {
    0
}

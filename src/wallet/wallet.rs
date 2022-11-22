use {
    anyhow::{anyhow, Result},
    bip0039::{Count, Language, Mnemonic},
    serde::{Deserialize, Serialize},
    std::{fs::read_to_string, fs::File, io::Write, path::Path},
};

const ROOT_FILE_NAME: &str = "root_wallet.key";
#[derive(Serialize, Deserialize)]
pub struct RootAccount {
    seed: String,
}
impl RootAccount {
    pub fn generate(
        lang: &str,
        wordslen: u8,
        passphrase: &str,
        home_path: &str,
    ) -> Result<(Self, String)> {
        let language = check_lang(lang)?;
        let word_count = check_word(wordslen)?;
        let mnemonic = Mnemonic::generate_in(language, word_count);
        let seed = mnemonic.to_seed(passphrase);
        let account = RootAccount {
            seed: hex::encode(seed),
        };
        let file_name = format!("{}/{}", home_path, ROOT_FILE_NAME);
        if Path::new(file_name.as_str()).exists() {
            return Err(anyhow!("root account already exists"));
        }
        let mut file = File::create(file_name)?;
        file.write_all(serde_json::to_string(&account)?.as_bytes())?;
        Ok((account, mnemonic.into_phrase()))
    }
    #[inline(always)]
    pub fn load_from_file(home_path: &str) -> Result<Self> {
        let file_name = format!("{}/{}", home_path, ROOT_FILE_NAME);
        if !Path::new(file_name.as_str()).exists() {
            return Err(anyhow!(
                "root account does not exist, please create it first"
            ));
        }
        let json = read_to_string(file_name)?;
        Ok(serde_json::from_str::<Self>(json.as_str())?)
    }
    pub fn get_seed(&self) -> Result<[u8; 64]> {
        let data = hex::decode(self.seed.clone())?;
        if 64 != data.len() {
            return Err(anyhow!("seed length error"));
        }
        let mut seed = [0u8; 64];
        seed.copy_from_slice(&data);
        Ok(seed)
    }
}
#[inline(always)]
pub fn check_lang(lang: &str) -> Result<Language> {
    let l = match lang {
        "en" => Language::English,
        "zh" => Language::SimplifiedChinese,
        "zh_traditional" => Language::TraditionalChinese,
        "fr" => Language::French,
        "it" => Language::Italian,
        "ko" => Language::Korean,
        "sp" => Language::Spanish,
        "jp" => Language::Japanese,
        _ => {
            return Err(anyhow!("Unsupported language"));
        }
    };
    Ok(l)
}
#[inline(always)]
pub fn check_word(wordslen: u8) -> Result<Count> {
    let cnt = match wordslen {
        12 => Count::Words12,
        15 => Count::Words15,
        18 => Count::Words18,
        21 => Count::Words21,
        24 => Count::Words24,
        _ => {
            return Err(anyhow!(
                "Invalid words length, only 12/15/18/21/24 can be accepted."
            ));
        }
    };
    Ok(cnt)
}

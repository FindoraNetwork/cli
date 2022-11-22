use {anyhow::Result, clap::Args};

#[derive(Debug, Args)]
///Asset Management
pub struct Asset {}

impl Asset {
    pub fn execute(self, _home: &str) -> Result<()> {
        Ok(())
    }
}

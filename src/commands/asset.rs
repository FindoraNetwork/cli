use clap::Args;

use crate::Result;

#[derive(Debug, Args)]
pub struct Asset {

}

impl Asset {
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}


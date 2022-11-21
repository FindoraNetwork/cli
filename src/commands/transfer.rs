use clap::Args;

use crate::Result;

#[derive(Debug, Args)]
/// Do transfer
pub struct Transfer {
    #[arg(short, long)]
    from: String,

    #[arg(short, long)]
    to: String,

    #[arg(short, long)]
    asset: String,

    #[arg(short, long)]
    sub_asset: String,

    #[arg(short, long)]
    confidential_amount: String,
}

impl Transfer {
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}


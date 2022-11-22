use {anyhow::Result, clap::Args};

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

    #[arg(long)]
    confidential_amount: bool,

    #[arg(long)]
    confidential_asset: bool,

    #[arg(long)]
    confidential_amount_asset: bool,

    #[arg(long)]
    confidential_amount_asset_address: bool,

    #[arg(long)]
    lowlevel_data: String,
}

impl Transfer {
    pub fn execute(self, _home: &str) -> Result<()> {
        Ok(())
    }
}

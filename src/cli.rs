/* src/cli.rs */

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "CatWrt configuration toolbox", long_about = None)]
pub struct Cli {
    /// Skip update check
    #[arg(short, long)]
    pub update: bool,

    /// Specify config file path
    #[arg(long)]
    pub config: Option<String>,
}

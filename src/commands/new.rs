use clap::Parser;

#[derive(Debug, Parser)]
pub struct NewCommand {
    pub name: String,
    #[clap(long)]
    pub dir: Option<String>,
    #[clap(short, long)]
    pub message: Option<String>,
}

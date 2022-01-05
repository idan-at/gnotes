use clap::Parser;

#[derive(Debug, Parser)]
pub struct AppendCommand {
    pub name: String,
    pub message: String,
    #[clap(long)]
    pub dir: Option<String>,
}

use clap::Parser;

#[derive(Debug, Parser)]
pub struct RemoveCommand {
    pub name: String,
    #[clap(long)]
    pub dir: Option<String>,
}

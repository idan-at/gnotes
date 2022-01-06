use clap::Parser;

// TODO: What about list all?
#[derive(Debug, Parser)]
pub struct ListCommand {
    #[clap(long)]
    pub dir: Option<String>,
}

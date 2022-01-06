use clap::Parser;

#[derive(Debug, Parser)]
pub struct AddCommand {
    pub name: String,
    pub message: String,
    #[clap(long)]
    pub dir: Option<String>,
}

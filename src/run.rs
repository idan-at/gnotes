use crate::config::Config;
use anyhow::Result;

pub trait Run {
    fn run(&self, config: &Config) -> Result<()>;
}

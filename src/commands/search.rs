use crate::common::notes::resolve_dir;
use crate::common::tags::load_tags;
use crate::common::writers::write_as_markdown;
use crate::config::Config;
use crate::run::Run;
use anyhow::Result;
use clap::Parser;
use log::debug;
use std::path::{PathBuf, MAIN_SEPARATOR};
use std::process;

#[derive(Debug, Parser)]
pub struct SearchCommand {
    pub tag: String,
    #[clap(long)]
    pub dir: Option<PathBuf>,
    #[clap(long)]
    pub all: bool,
    #[clap(long)]
    pub show: bool,
}

impl Run for SearchCommand {
    fn run(&self, config: &Config) -> Result<()> {
        debug!("search command {:?}", self);

        if self.dir.is_some() && self.all {
            eprintln!("--dir can't be used with --all");

            process::exit(1);
        }

        let dir = resolve_dir(&self.dir);
        let tags = load_tags(&config.notes_dir)?;

        let results = match tags.get(&self.tag) {
            Some(tags_set) => {
                if self.all {
                    tags_set.into_iter().collect()
                } else {
                    let note_identifier_prefix =
                        format!("{}{}", String::from(dir.to_string_lossy()), MAIN_SEPARATOR);

                    tags_set
                        .into_iter()
                        .filter(|note_identifier| {
                            note_identifier.starts_with(&note_identifier_prefix)
                        })
                        .collect()
                }
            }
            _ => vec![],
        };

        if results.len() > 0 {
            println!("total {}", results.len());
        }

        for result in results {
            if self.show {
                write_as_markdown(&config.notes_dir, result)?;
            } else {
                println!("{}", result);
            }
        }

        Ok(())
    }
}

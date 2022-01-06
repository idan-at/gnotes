use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use chrono::Datelike;
use log::debug;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

pub fn format_system_time(system_time: SystemTime) -> String {
    let date_time: DateTime<Utc> = system_time.into();
    let now: DateTime<Utc> = SystemTime::now().into();

    if date_time.year() == now.year()
        && date_time.month() == now.month()
        && date_time.day() == now.day()
    {
        format!("{}", date_time.format("%H:%M"))
    } else {
        format!("{}", date_time.format("%b %e %H:%M"))
    }
}

pub fn write_note(note_parent_dir: &Path, note_file_name: &str, content: &str) -> Result<()> {
    let note_file_path = note_parent_dir.join(note_file_name);

    debug!("Writing message '{}' to {:?}", content, note_file_path);

    fs::create_dir_all(note_parent_dir)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(note_file_path)
        .unwrap();

    writeln!(file, "{}", content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn format_system_time_not_today() {
        let system_time = SystemTime::UNIX_EPOCH;

        let formatted = format_system_time(system_time);

        assert_eq!(formatted, String::from("Jan  1 00:00"))
    }

    #[test]
    fn format_system_time_today() {
        let system_time = SystemTime::now();
        let now: DateTime<Utc> = SystemTime::now().into();

        let formatted = format_system_time(system_time);

        assert_eq!(formatted, format!("{}:{}", now.hour(), now.minute()))
    }
}

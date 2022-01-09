use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use chrono::Datelike;
use std::fs;
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

pub fn write_as_markdown(notes_dir: &Path, note_identifier: &str) -> Result<()> {
    let note_file_path = notes_dir.join(note_identifier);
    let content = fs::read_to_string(note_file_path)?;

    println!("{}:", note_identifier);
    termimad::print_text(&content);

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

        assert_eq!(formatted, format!("{:02}:{:02}", now.hour(), now.minute()))
    }
}

use std::{env, error::Error, path::PathBuf};

use unraid_backup_util::{run, Config};
use unraid_notification::notifier::Notifier;

mod job_description;
mod notifier;

static DEFAULT_CONFIG_PATH: &'static str = "config.json";

fn main() {
    let config = match read_config() {
        Ok(config) => config,
        Err(e) => {
            eprint!("{:?}", e);
            notifier::send_error(&e);
            return;
        }
    };

    if let Err(e) = run(config) {
        notifier::send_error(&e);
        eprint!("{:?}", e);
    }else{
        println!("Backup succeeded...");
        notifier::NOTIFIER.send("Backup succeeded".into(), unraid_notification::notificationlevel::NotificationLevel::Info).expect("Could not send message");
    }
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    let config_path = env::args()
        .skip(1)
        .next()
        .clone()
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from(DEFAULT_CONFIG_PATH));
    let content = std::fs::read_to_string(config_path).map_err(Box::new)?;

    let value = serde_json::from_str(&content)?;
    Ok(value)
}

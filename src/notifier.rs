use std::{error::Error, path::Path};

use unraid_notification::notifier::{Notifier, UnraidNotifier};
use once_cell::sync::Lazy;

pub static NOTIFIER : Lazy<UnraidNotifier<&Path, String>> = Lazy::new(|| {
    UnraidNotifier::default().with_sender("Backup service")
});

pub fn send_error(error: &Box<dyn Error>){
    NOTIFIER.send(error.to_string(), unraid_notification::notificationlevel::NotificationLevel::Error).expect("could not contact unraid notifier service");
}

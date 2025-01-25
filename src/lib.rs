use std::{
    error::Error,
    process::{Child, Command, ExitStatus}, time::Duration,
};

use job_description::Transfer;
use macaddr::MacAddr6;
use serde::Deserialize;
use wol::MacAddr;

mod job_description;
mod notifier;

#[derive(Deserialize, Debug)]
pub struct Config {
    mac_address: Option<MacAddr6>,
    shutdown: bool,
    user: String,
    hostname: String,
    options: Vec<String>,
    jobs: Vec<Transfer>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(address) = &config.mac_address {
        wakeup(address)?;
    }

    transfer(&config)?;

    if config.shutdown {
        sleep(&config)?;
    }

    Ok(())
}

fn transfer(config: &Config) -> Result<(), Box<dyn Error>> {
    let failed = config
        .jobs
        .iter()
        .map(|job| handle_transfer(&config, job))
        .map(|mut process| process.wait())
        .filter(|result| match result {
            Ok(exit_status) => {
                if !exit_status.success() {
                    let error = format!(
                        "rsync terminated with non-success status code: {:?}",
                        exit_status
                    );
                    eprintln!("{}", &error);
                    true
                } else {
                    false
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                true
            }
        })
        .count();

    if failed != 0 {
        Err(format!("Some transfers went wrong: {:?}", failed).into())
    } else {
        Ok(())
    }
}

fn handle_transfer(config: &Config, description: &Transfer) -> Child {
    let options = description
        .override_options
        .as_ref()
        .map(|x| x.as_slice())
        .unwrap_or(&config.options);

    let destination = format!(
        "{}@{}:{}",
        config.user, config.hostname, description.destination.to_str().unwrap()
    );

    Command::new("rsync")
        .args(options)
        .arg(&description.source)
        .arg(&destination)
        .spawn()
        .expect("could not spawn process...")
}

fn wakeup(mac_address: &MacAddr6) -> Result<(), Box<dyn Error>> {
    let mac_address_bytes = mac_address.as_bytes();

    let _addr = MacAddr([
        mac_address_bytes[0],
        mac_address_bytes[1],
        mac_address_bytes[2],
        mac_address_bytes[3],
        mac_address_bytes[4],
        mac_address_bytes[5],
    ]);

    wol::send_wol(_addr, None, None)?;

    println!("WoL send, entering 4 minute sleep");

    std::thread::sleep(Duration::new(240, 0));

    println!("2 minute sleep over, assuming device has woken up");

    Ok(())
}

fn sleep(config: &Config) -> Result<(), Box<dyn Error>> {
    let ssh_commad = format!(
        "ssh {}@{} 'sudo shutdown -P now'",
        config.user, config.hostname
    );

    Command::new("sh")
        .arg("-c")
        .arg(ssh_commad)
        .output()
        .expect("Failed to execute process");

    Ok(())
}

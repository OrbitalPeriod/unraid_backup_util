use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Transfer {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub override_options: Option<Vec<String>>,
}

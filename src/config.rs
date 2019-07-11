use crate::commands;

use log::info;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use toml;
use yansi::Paint;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    Push,
    PullRequest,
    Commands,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: Site,
    pub gitlab: Site,
    pub features: Vec<Feature>,
    pub commands: Commands,
    pub pr_branch_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct RepoMapping {
    pub mappings: Vec<Mapping>,
}

pub fn feature_enabled(feature: &Feature) -> bool {
    CONFIG.features.contains(&feature)
}

pub fn command_enabled(command: &commands::CommandAction) -> bool {
    feature_enabled(&Feature::Commands) && CONFIG.commands.enabled_commands.contains(&command)
}

#[derive(Debug, Deserialize)]
pub struct Commands {
    pub enabled_commands: Vec<commands::CommandAction>,
}

#[derive(Debug, Deserialize)]
pub struct Site {
    pub webhook_secret: String,
    pub username: String,
    pub ssh_key: String,
    pub api_token: String,
    pub hostname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Mapping {
    pub github_repo: String,
    pub gitlab_repo: String,
}

lazy_static! {
    pub static ref HUB_TO_LAB: Mutex<HashMap<String, String>> = {
        let m: HashMap<String, String> = HashMap::new();
        Mutex::new(m)
    };
}

lazy_static! {
    pub static ref LAB_TO_HUB: Mutex<HashMap<String, String>> = {
        let m: HashMap<String, String> = HashMap::new();
        Mutex::new(m)
    };
}

fn get_labhub_toml_path() -> String {
    env::var("LABHUB_TOML").unwrap_or_else(|_| "LabHub.toml".to_string())
}

fn get_repo_mapping_toml_path() -> String {
    env::var("LABHUB_REPO_MAPPING_TOML").unwrap_or_else(|_| "LabHub-repo-mapping.toml".to_string())
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let labhub_toml_path = get_labhub_toml_path();
        let config: Config = toml::from_str(&read_file_to_string(&labhub_toml_path)).unwrap();
        config
    };
}

lazy_static! {
    pub static ref REPO_MAPPING: RepoMapping = {
        let repo_mapping_toml_path = get_repo_mapping_toml_path();
        let repo_mapping: RepoMapping = toml::from_str(&read_file_to_string(&repo_mapping_toml_path)).unwrap();
        repo_mapping
    };
}

fn read_file_to_string(filename: &str) -> String {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

pub fn load_config() {
    info!(
        "Loaded LabHub configuration values from {}",
        get_labhub_toml_path()
    );
    info!("CONFIG => {:#?}", Paint::red(&*CONFIG));

    info!(
        "Loaded LabHub repo mapping values from {}",
        get_repo_mapping_toml_path()
    );
    info!("REPO_MAPPING => {:#?}", Paint::red(&*REPO_MAPPING));

    for mapping in REPO_MAPPING.mappings.iter() {
        let mut hub_to_lab_lock = HUB_TO_LAB.lock();
        let hub_to_lab = hub_to_lab_lock.as_mut().unwrap();
        hub_to_lab.insert(mapping.github_repo.clone(), mapping.gitlab_repo.clone());

        let mut lab_to_hub_lock = LAB_TO_HUB.lock();
        let lab_to_hub = lab_to_hub_lock.as_mut().unwrap();
        lab_to_hub.insert(mapping.gitlab_repo.clone(), mapping.github_repo.clone());
    }
    info!(
        "HUB_TO_LAB => {:#?}",
        Paint::red(HUB_TO_LAB.lock().unwrap())
    );
    info!(
        "LAB_TO_HUB => {:#?}",
        Paint::red(LAB_TO_HUB.lock().unwrap())
    );
}

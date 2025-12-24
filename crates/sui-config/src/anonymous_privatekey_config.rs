use clap::Args;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Context;
use tracing::trace;

#[derive(Default, Debug, Clone, Deserialize, Serialize, Args)]
#[serde(rename_all = "kebab-case")]
pub struct AnonymousPrivateKeyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub anonymous_privatekey: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub anonymous_coordseed: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub anonymous_rpc: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub enable_anonymous_rpc: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub fullnode_rpc_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub zklogin_verify_rpc_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub enable_anonymous_version_v2: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long)]
    pub anonymous_data_v2_open_epoch: Option<u64>,
}

pub const DEFAULT_ANONYMOUS_DATA_V2_OPEN_EPOCH :u64 = 0;

impl AnonymousPrivateKeyConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_anonymous_data_v2_open_epoch(&mut self, anonymous_data_v2_open_epoch: u64) {
        self.anonymous_data_v2_open_epoch = Some(anonymous_data_v2_open_epoch);
    }

    pub fn get_anonymous_data_v2_open_epoch(&self) -> u64 {
        self.anonymous_data_v2_open_epoch.unwrap_or(DEFAULT_ANONYMOUS_DATA_V2_OPEN_EPOCH)
    }

    pub fn set_enable_anonymous_version_v2(&mut self, value: bool) {
        self.enable_anonymous_version_v2 = Some(value);
    }

    pub fn enable_anonymous_version_v2(&self) -> Option<bool> {
        self.enable_anonymous_version_v2.clone()
    }

    pub fn get_private_key(&self) -> Option<String> {
        self.anonymous_privatekey.clone()
    }

    pub fn get_anonymous_coordseed(&self) -> Option<u64> {
        self.anonymous_coordseed.clone()
    }

    pub fn set_private_key(&mut self, key: String) {
        self.anonymous_privatekey = Some(key);
    }

    pub fn set_anonymous_coordseed(&mut self, key: u64) {
        self.anonymous_coordseed = Some(key);
    }

    pub fn set_fullnode_rpc_path(&mut self, key: String) {
        self.fullnode_rpc_path = Some(key);
    }

    pub fn set_zklogin_verify_rpc_path(&mut self, key: String) {
        self.zklogin_verify_rpc_path = Some(key);
    }

    pub fn enable_anonymous_rpc(&mut self, key: bool) {
        self.enable_anonymous_rpc = Some(key);
    }

    pub fn get_anonymous_rpc(&self) -> Option<Vec<String>> {
        self.anonymous_rpc.clone()
    }

    pub fn set_anonymous_rpc(&mut self, rpc_urls: Vec<String>) {
        self.anonymous_rpc = Some(rpc_urls);
    }

    pub fn add_anonymous_rpc(&mut self, rpc_url: String) {
        match &mut self.anonymous_rpc {
            Some(urls) => urls.push(rpc_url),
            None => self.anonymous_rpc = Some(vec![rpc_url]),
        }
    }

    /// Load configuration from YAML file
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;
        
        let private_key = yaml_value
            .get("anonymous-privatekey")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let coord_seed = yaml_value
            .get("anonymous-coordseed")
            .and_then(|s| s.as_u64());

        let enable_anonymous_rpc = yaml_value
            .get("enable-anonymous-rpc")
            .and_then(|v| v.as_bool());
        
        let anonymous_rpc = yaml_value
            .get("anonymous-rpc")
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|item| item.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            });

        let fullnode_rpc_path = yaml_value
            .get("fullnode-rpc-path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let zklogin_verify_rpc_path = yaml_value
            .get("zklogin-verify-rpc-path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let enable_anonymous_version_v2 = yaml_value
            .get("enable-anonymous-version-v2")
            .and_then(|v| v.as_bool());

        let anonymous_data_v2_open_epoch = yaml_value
            .get("anonymous-data-v2-open-epoch")
            .and_then(|s| s.as_u64());

        Ok(AnonymousPrivateKeyConfig { 
            anonymous_privatekey: private_key,
            anonymous_coordseed: coord_seed,
            anonymous_rpc,
            enable_anonymous_rpc,
            fullnode_rpc_path,
            zklogin_verify_rpc_path,
            enable_anonymous_version_v2,
            anonymous_data_v2_open_epoch,
        })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<(), anyhow::Error> {
        let path = path.as_ref();
        trace!("Writing config to {}", path.display());
        let config = serde_yaml::to_string(&self)?;
        fs::write(path, config)
            .with_context(|| format!("Unable to save config to {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_from_yaml_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("data/bfc_anonymous_config.yaml");
        
        let config = AnonymousPrivateKeyConfig::from_yaml_file(&path)
            .expect("Failed to load config from yaml file");
        println!("the config key is {:?}", config.anonymous_privatekey);
        println!("the coord seed is {:?}", config.anonymous_coordseed);

        println!("the anonymous_rpc is {:?}", config.anonymous_rpc);

        assert!(config.anonymous_privatekey.is_some());
        assert!(config.anonymous_coordseed.is_some());
        assert_eq!(config.anonymous_coordseed.unwrap(), 116540450355);
    }


}
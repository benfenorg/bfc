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
}


impl AnonymousPrivateKeyConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_private_key(&self) -> Option<String> {
        self.anonymous_privatekey.clone()
    }

    pub fn set_private_key(&mut self, key: String) {
        self.anonymous_privatekey = Some(key);
    }

    /// Load configuration from YAML file
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;
        
        let private_key = yaml_value
            .get("anonymous_privatekey")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        Ok(AnonymousPrivateKeyConfig { anonymous_privatekey: private_key })
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
        // 构建到测试yaml文件的路径
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("data/bfc_anonymous_config.yaml");
        
        // 从yaml文件加载配置
        let config = AnonymousPrivateKeyConfig::from_yaml_file(&path)
            .expect("Failed to load config from yaml file");
        println!("the config key is {:?}", config.anonymous_privatekey);
        
        // 验证私钥是否正确加载
        assert!(config.anonymous_privatekey.is_some());
        assert_eq!(config.anonymous_privatekey.unwrap(), "your-private-key-here");
    }


}
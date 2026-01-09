// Config save/load functionality

use crate::get_windows_path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TweakConfig
{
        pub tweaks: std::collections::HashMap<String, bool>,
        #[serde(default)]
        pub input_values: std::collections::HashMap<String, String>,
}

/// Get default config directory (Documents/W11Boost/Configs)
pub fn get_config_dir() -> Option<std::path::PathBuf>
{
        get_windows_path(&winsafe::co::KNOWNFOLDERID::Documents)
                .ok()
                .map(|d| std::path::PathBuf::from(d).join("W11Boost").join("Configs"))
}

/// Get the fixed path for the dynamic settings file
pub fn get_default_config_path() -> Option<std::path::PathBuf>
{
        get_config_dir().map(|d| d.join("settings.json"))
}

impl TweakConfig
{
        pub fn new(
                tweaks: std::collections::HashMap<String, bool>,
                input_values: std::collections::HashMap<String, String>,
        ) -> Self
        {
                Self { tweaks, input_values }
        }

        pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()>
        {
                if let Some(parent) = path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                }
                let json = serde_json::to_string_pretty(self)?;
                std::fs::write(path, json)?;
                Ok(())
        }

        pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self>
        {
                let json = std::fs::read_to_string(path)?;
                let config = serde_json::from_str(&json)?;
                Ok(config)
        }

        /// Save to the default dynamic config path
        pub fn save_default(&self) -> anyhow::Result<()>
        {
                if let Some(path) = get_default_config_path() {
                        self.save_to_file(&path)?;
                }
                Ok(())
        }

        /// Load from the default dynamic config path
        pub fn load_default() -> anyhow::Result<Self>
        {
                if let Some(path) = get_default_config_path() {
                        Self::load_from_file(&path)
                } else {
                        anyhow::bail!("Could not determine config path")
                }
        }
}

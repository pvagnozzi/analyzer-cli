#![allow(dead_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct EnvGuard {
    _lock: MutexGuard<'static, ()>,
    previous: HashMap<String, Option<String>>,
}

impl EnvGuard {
    pub fn new(vars: &[(&str, Option<String>)]) -> Self {
        let lock = env_lock().lock().expect("environment mutex poisoned");
        let mut previous = HashMap::new();

        for (key, value) in vars {
            previous.insert((*key).to_string(), std::env::var(key).ok());
            match value {
                Some(v) => unsafe { std::env::set_var(key, v) },
                None => unsafe { std::env::remove_var(key) },
            }
        }

        Self {
            _lock: lock,
            previous,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in &self.previous {
            match value {
                Some(v) => unsafe { std::env::set_var(key, v) },
                None => unsafe { std::env::remove_var(key) },
            }
        }
    }
}

pub fn config_env(temp_root: &Path) -> Vec<(&'static str, String)> {
    let appdata = temp_root.join("appdata");
    let xdg = temp_root.join("xdg");
    let home = temp_root.join("home");
    let config_root = temp_root.join("config-root");

    std::fs::create_dir_all(&appdata).expect("failed to create appdata temp dir");
    std::fs::create_dir_all(&xdg).expect("failed to create xdg temp dir");
    std::fs::create_dir_all(&home).expect("failed to create home temp dir");
    std::fs::create_dir_all(&config_root).expect("failed to create config root temp dir");

    vec![
        ("APPDATA", appdata.display().to_string()),
        ("XDG_CONFIG_HOME", xdg.display().to_string()),
        ("HOME", home.display().to_string()),
        ("USERPROFILE", home.display().to_string()),
        ("ANALYZER_CONFIG_DIR", config_root.display().to_string()),
    ]
}

pub fn apply_config_env(temp_root: &Path) -> EnvGuard {
    apply_full_env(temp_root, &[])
}

pub fn set_runtime_env(pairs: &[(&str, &str)]) -> EnvGuard {
    let vars: Vec<(&str, Option<String>)> = pairs
        .iter()
        .map(|(k, v)| (*k, Some((*v).to_string())))
        .collect();
    EnvGuard::new(&vars)
}

pub fn apply_full_env(temp_root: &Path, pairs: &[(&str, &str)]) -> EnvGuard {
    let mut vars: Vec<(&str, Option<String>)> = config_env(temp_root)
        .into_iter()
        .map(|(k, v)| (k, Some(v)))
        .collect();

    vars.extend([
        ("ANALYZER_API_KEY", None),
        ("ANALYZER_URL", None),
        ("ANALYZER_PROFILE", None),
        ("ANALYZER_LANG", None),
    ]);

    vars.extend(pairs.iter().map(|(k, v)| (*k, Some((*v).to_string()))));

    EnvGuard::new(&vars)
}

pub fn config_file_path(temp_root: &Path) -> PathBuf {
    temp_root.join("config-root").join("config.toml")
}

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

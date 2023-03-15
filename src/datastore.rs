use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use once_cell::sync::Lazy;

static DIR: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("com.github", "bubbler-4", "cargo-boj").unwrap()
});
// create config dir and config file on first config access
static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let mut dir = DIR.config_dir().to_path_buf();
    fs::create_dir_all(dir.clone()).unwrap();
    dir.push("config.json");
    // create file if not exists, ignore any errors otherwise
    let file = fs::OpenOptions::new().append(true).create_new(true).open(dir.clone());
    drop(file);
    dir
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookies {
    pub onlinejudge: String,
    pub bojautologin: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Credentials {
    pub id: Option<String>,
    pub cookies: Option<Cookies>,
}

impl Credentials {
    pub fn load() -> Self {
        let config_str = fs::read_to_string(CONFIG_FILE.as_path()).unwrap();
        serde_json::from_str(&config_str).unwrap_or_default()
    }

    pub fn update_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
        let config_str = serde_json::to_string(self).unwrap();
        fs::write(CONFIG_FILE.as_path(), &config_str).unwrap();
    }

    pub fn update_cookie(&mut self, cookies: &Cookies) {
        self.cookies = Some(cookies.clone());
        let config_str = serde_json::to_string(self).unwrap();
        fs::write(CONFIG_FILE.as_path(), &config_str).unwrap();
    }

    pub fn remove(&mut self) {
        self.id = None;
        self.cookies = None;
        let config_str = serde_json::to_string(self).unwrap();
        fs::write(CONFIG_FILE.as_path(), &config_str).unwrap();
    }
}
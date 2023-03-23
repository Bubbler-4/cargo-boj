use directories::ProjectDirs;
use once_cell::sync::Lazy;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

static DIR: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("com.github", "bubbler-4", "cargo-boj").unwrap());

// create config dir and config file on first config access
static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let dir = DIR.config_dir().to_path_buf();
    fs::create_dir_all(dir.clone()).unwrap();
    let mut file = dir;
    file.push("config.json");
    // create file if not exists, ignore any errors otherwise
    let file_handle = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(file.clone());
    drop(file_handle);
    file
});

static CACHE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = DIR.cache_dir().to_path_buf();
    fs::create_dir_all(dir.clone()).unwrap();
    dir
});

fn cache_file(problem_id: &str) -> PathBuf {
    let mut file = CACHE_DIR.clone();
    if problem_id.contains('/') {
        file.push("contest");
    }
    file.push(problem_id);
    let file_handle = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(file.clone());
    drop(file_handle);
    file
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemData {
    pub spj: bool,
    pub testcases: Vec<(String, String)>,
}

impl ProblemData {
    fn fetch_test_cases(problem_id: &str) -> Self {
        let url = if problem_id.contains('/') {
            format!("https://www.acmicpc.net/contest/problem/{}", problem_id)
        } else {
            format!("https://www.acmicpc.net/problem/{}", problem_id)
        };
        let res = get(url).unwrap().text().unwrap();
        let html = Html::parse_document(&res);
        let spj_selector =
            Selector::parse("span.problem-label-spj, span.problem-label-two-steps").unwrap();
        let mut it = html.select(&spj_selector);
        let spj = it.next().is_some();
        let selector = Selector::parse("pre.sampledata").unwrap();
        let mut it = html.select(&selector);
        let mut testcases = vec![];
        while let Some(inel) = it.next() {
            let input = inel.text().collect::<String>();
            let output = it.next().unwrap().text().collect::<String>();
            testcases.push((input, output));
        }
        Self { spj, testcases }
    }

    pub fn load(problem_id: &str) -> Self {
        let cache_path = cache_file(problem_id);
        let cache_str = fs::read_to_string(cache_path.as_path()).unwrap();
        serde_json::from_str(&cache_str).unwrap_or_else(|_e| {
            let data = Self::fetch_test_cases(problem_id);
            let cache_str = serde_json::to_string(&data).unwrap();
            fs::write(cache_path.as_path(), cache_str).unwrap();
            data
        })
    }
}

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

    #[allow(unused)]
    pub fn update_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
        let config_str = serde_json::to_string(self).unwrap();
        fs::write(CONFIG_FILE.as_path(), config_str).unwrap();
    }

    pub fn update_cookie(&mut self, cookies: &Cookies) {
        self.cookies = Some(cookies.clone());
        let config_str = serde_json::to_string(self).unwrap();
        fs::write(CONFIG_FILE.as_path(), config_str).unwrap();
    }

    pub fn remove(&mut self) {
        self.id = None;
        self.cookies = None;
        let config_str = serde_json::to_string(self).unwrap();
        fs::write(CONFIG_FILE.as_path(), config_str).unwrap();
    }
}

use reqwest::{cookie::Jar, Url, blocking::Client};
use std::sync::Arc;
use std::io::Read;
use tokio::runtime::Runtime;
use scraper::{Html, Selector};

mod login;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

// cargo-boj login [--id=<id>]
//   login to BOJ with <id>. password is entered through a prompt.
//   if login is successful, id and cookies are stored for submitting solutions.
//   if id is supplied, it is used to log in to BOJ, and the id is stored for later runs.
//   if id is not supplied, the stored id is used; if it doesn't exist, the command fails.
// cargo-boj logout
//   delete id and cookies. may be useful if using cargo-boj on a shared computer.
// cargo-boj test <prob> [--bin=<bin>]
//   fetch sample tests for <prob> and run tests on the binary.
//   sample tests are cached by problem id.
//   if bin is supplied, uses its bin name. if set is also set, it is stored for later runs.
//   if bin is not supplied, the stored bin name is used; if it doesn't exist, defaults to `main`.
// cargo-boj submit <prob> [--path=<path>] [--lang-id=<lang>] [--code-open=(y|n|acc)]
//   submit the file at <path> as the solution to problem <prob>.
//   each option defaults to:
//   path = src/main.rs or src/bin/main.rs
//   lang-id = 113 (Rust 2021)
//   code-open = follow account default
// cargo-boj set [--bin=<bin>] [--path=<path>]
//   store settings for test binary name and submit file path.

fn main() -> Result<()> {
    Ok(())
}

fn _main() -> Result<()> {
    let should_login = false;
    let id = "";
    let password = "";
    let (onlinejudge, bojautologin) = if should_login {
        let rt = Runtime::new()?;
        rt.block_on(login::login_procedure(id, password))?
    } else {
        unimplemented!()
    };
    let problem_id = "2557";
    let source = r#"fn main(){println!("Hello World!");}"#;
    submit_solution(&onlinejudge, &bojautologin, problem_id, source);
    Ok(())
}

fn submit_solution(onlinejudge: &str, bojautologin: &str, problem_id: &str, source: &str) {
    let cookie = format!("bojautologin={}; domain=.acmicpc.net, OnlineJudge={}; domain=.acmicpc.net", bojautologin, onlinejudge);
    let url = "https://acmicpc.net/".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);
    let jar = Arc::new(jar);

    let client = Client::builder()
        .user_agent(UA)
        .cookie_store(true)
        .cookie_provider(jar.clone())
        .build()
        .unwrap();
    let submit_page = format!("https://www.acmicpc.net/submit/{}", problem_id); // TODO check contest submissions
    let get = client.get(&submit_page);
    let mut res = get.send().unwrap();
    let mut output = String::new();
    res.read_to_string(&mut output).unwrap();
    if !res.url().as_str().contains("submit") {
        println!("Submit page access failed. Please log in.");
        return;
    }
    
    let html = Html::parse_document(&output);
    let csrf_selector = Selector::parse(r#"[name="csrf_key"]"#).unwrap();
    let csrf_el = html.select(&csrf_selector).next().unwrap();
    let csrf_key = csrf_el.value().attr("value").unwrap();
    let form_data = [
        ("problem_id", problem_id),
        ("language", "113"),
        ("code_open", "onlyaccepted"),
        ("source", source),
        ("csrf_key", csrf_key)
    ];
    let mut res = client.post(&submit_page).form(&form_data).send().unwrap();
    output.clear();
    res.read_to_string(&mut output).unwrap();
    println!("{}", output);
    println!("{:?}", res);
}
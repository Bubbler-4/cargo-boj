use reqwest::{cookie::Jar, Url, blocking::Client};
use std::sync::Arc;
use std::io::Read;
use tokio::runtime::Runtime;
use scraper::{Html, Selector};

mod login;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

fn main() -> Result<()> {
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
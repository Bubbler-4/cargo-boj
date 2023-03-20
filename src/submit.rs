use reqwest::{cookie::Jar, Url, blocking::Client};
use std::sync::Arc;
use std::io::Read;
use scraper::{Html, Selector};
use crate::UA;
use crate::datastore::Cookies;

pub fn submit_solution(cookies: &Cookies, problem_id: &str, source: &str, language: usize) {
    let Cookies { bojautologin, onlinejudge } = cookies;
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
    if res.url().as_str().contains("login") {
        println!("Submit page access failed. Please log in.");
        return;
    }
    
    let html = Html::parse_document(&output);
    let csrf_selector = Selector::parse(r#"[name="csrf_key"]"#).unwrap();
    let csrf_el = html.select(&csrf_selector).next().unwrap();
    let csrf_key = csrf_el.value().attr("value").unwrap();
    let form_data = [
        ("problem_id", problem_id),
        ("language", &language.to_string()),
        ("code_open", "onlyaccepted"),
        ("source", source),
        ("csrf_key", csrf_key)
    ];
    let mut res = client.post(&submit_page).form(&form_data).send().unwrap();
    output.clear();
    res.read_to_string(&mut output).unwrap();
    // println!("{}", output);
    // println!("{:?}", res);
    let url = res.url().as_str();
    println!("Submit successful. Check your submission at {}", url);
}
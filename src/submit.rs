use reqwest::{cookie::Jar, Url, blocking::Client};
use std::sync::Arc;
use std::io::{Read, Write};
use scraper::{Html, Selector};
use crate::UA;
use crate::datastore::Cookies;

pub fn submit_solution(cookies: &Cookies, problem_id: &str, source: &str, language: usize, code_open: Option<String>) {
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
    let code_open_selector = Selector::parse(r#"input[name="code_open"][checked]"#).unwrap();
    let code_open_el = html.select(&code_open_selector).next().unwrap();
    let code_open_value = code_open_el.value().attr("value").unwrap();
    let code_open = code_open.unwrap_or(code_open_value.to_string());
    let csrf_selector = Selector::parse(r#"[name="csrf_key"]"#).unwrap();
    let csrf_el = html.select(&csrf_selector).next().unwrap();
    let csrf_key = csrf_el.value().attr("value").unwrap();
    let form_data = [
        ("problem_id", problem_id),
        ("language", &language.to_string()),
        ("code_open", &code_open),
        ("source", source),
        ("csrf_key", csrf_key)
    ];
    let mut res = client.post(&submit_page).form(&form_data).send().unwrap();
    output.clear();
    res.read_to_string(&mut output).unwrap();
    
    let html = Html::parse_document(&output);
    let sol_selector = Selector::parse(r#"tbody tr"#).unwrap();
    let sol_el = html.select(&sol_selector).next().unwrap();
    let sol_id = sol_el.value().id().unwrap();
    let sol_id_no = sol_id.split('-').skip(1).next().unwrap();

    let url = res.url().as_str();
    println!("Submit successful (sol ID {}). Check your submission at {}", sol_id_no, url);
    print!("Or press Enter to fetch the judging status. Press any other key and Enter to exit. ");
    let mut stdout = std::io::stdout();
    stdout.flush().unwrap();
    loop {
        let mut buf = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buf).unwrap();
        if !buf.trim().is_empty() { break; }

        let mut res = client.get(url).send().unwrap();
        let mut output = String::new();
        res.read_to_string(&mut output).unwrap();
        let html = Html::parse_document(&output);
        let sol_selector = Selector::parse(&format!("#{} td.result span.result-text", sol_id)).unwrap();
        let sol_el = html.select(&sol_selector).next().unwrap();
        let classes = sol_el.value().classes().collect::<Vec<_>>();
        let verdict = classify_class(&classes);
        print!("Current status: {} ", verdict);
        stdout.flush().unwrap();
    }
}

fn classify_class(classes: &[&str]) -> &'static str {
    if classes.contains(&"result-wait") {
        "Pending"
    } else if classes.contains(&"result-compile") {
        "Compiling"
    } else if classes.contains(&"result-judging") {
        "Judging"
    } else if classes.contains(&"result-ac") {
        "Accepted"
    } else if classes.contains(&"result-pac") {
        "Partially Accepted"
    } else if classes.contains(&"result-pe") {
        "Presentation Error"
    } else if classes.contains(&"result-wa") {
        "Wrong Answer"
    } else if classes.contains(&"result-tle") {
        "Time Limit Exceeded"
    } else if classes.contains(&"result-mle") {
        "Memory Limit Exceeded"
    } else if classes.contains(&"result-ole") {
        "Output Limit Exceeded"
    } else if classes.contains(&"result-rte") {
        "Runtime Error"
    } else if classes.contains(&"result-ce") {
        "Compilation Error"
    } else if classes.contains(&"result-co") {
        "Cannot Judge"
    } else {
        "Unknown Error"
    }
}
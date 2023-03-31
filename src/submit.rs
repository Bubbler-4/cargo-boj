use crate::datastore::Cookies;
use crate::UA;
use reqwest::{blocking::Client, cookie::Jar, Url};
use scraper::{Html, Selector};
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::{Instant, Duration};
use crossterm::{terminal::{self, ClearType}, cursor, event::{self, Event}, execute};

pub fn submit_solution(
    cookies: &Cookies,
    problem_id: &str,
    source: &str,
    language: usize,
    code_open: Option<String>,
) {
    let Cookies {
        bojautologin,
        onlinejudge,
    } = cookies;
    let cookie = format!(
        "bojautologin={}; domain=.acmicpc.net, OnlineJudge={}; domain=.acmicpc.net",
        bojautologin, onlinejudge
    );
    let url = "https://acmicpc.net/".parse::<Url>().unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);
    let jar = Arc::new(jar);

    let client = Client::builder()
        .user_agent(UA)
        .cookie_store(true)
        .cookie_provider(jar)
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
        ("csrf_key", csrf_key),
    ];
    let mut res = client.post(&submit_page).form(&form_data).send().unwrap();
    output.clear();
    res.read_to_string(&mut output).unwrap();

    let html = Html::parse_document(&output);
    let sol_selector = Selector::parse(r#"tbody tr"#).unwrap();
    let sol_el = html.select(&sol_selector).next().unwrap();
    let sol_id = sol_el.value().id().unwrap();
    let sol_id_no = sol_id.split('-').nth(1).unwrap();

    let url = res.url().as_str();
    println!(
        "Submit successful (sol ID {}). Status page: {}",
        sol_id_no, url
    );
    println!("Press any key to exit.");
    submit_loop(&client, url, sol_id);
}

fn submit_loop(client: &Client, url: &str, sol_id: &str) {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, cursor::SavePosition).unwrap();
    'outer: loop {
        let mut res = client.get(url).send().unwrap();
        let mut output = String::new();
        res.read_to_string(&mut output).unwrap();
        let html = Html::parse_document(&output);
        let sol_selector =
            Selector::parse(&format!("#{} td.result span.result-text", sol_id)).unwrap();
        let sol_el = html.select(&sol_selector).next().unwrap();
        let classes = sol_el.value().classes().collect::<Vec<_>>();
        let verdict = classify_class(&classes);
        execute!(stdout, terminal::Clear(ClearType::CurrentLine), cursor::RestorePosition).unwrap();
        print!("Current status: {} ", verdict);
        stdout.flush().unwrap();
        if judge_finished(verdict) { break; }

        let now = Instant::now();
        while now.elapsed() < Duration::from_secs(1) {
            if event::poll(Duration::from_secs(0)).unwrap() {
                match event::read().unwrap() {
                    Event::Key(_) => break 'outer,
                    _ => {}
                }
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
    println!();
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

fn judge_finished(verdict: &str) -> bool {
    match verdict {
        "Pending" | "Compiling" | "Judging" => false,
        _ => true
    }
}
